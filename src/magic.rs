#![allow(unsafe_code)]

use std::{
    cell::RefCell,
    collections::HashMap,
    ffi::{CStr, CString, c_void},
    path::{Path, PathBuf},
};

use ignore::WalkBuilder;
use rayon::prelude::*;

use crate::{
    app::Config,
    detector::FileTypeDetector,
    error::{Error, Result},
    ffi,
    scanner::{ScanSummary, Scanner},
};

/// This is not thread-safe.
#[derive(Debug)]
pub struct LibMagicDetector {
    magic_file: Option<String>,
    cookie: *mut c_void,
}

impl LibMagicDetector {
    pub fn build(magic_file: Option<String>) -> Result<Self> {
        // If only MAGIC_EXTENSION is specified, some file formats will appear as “???”.
        // By specifying MAGIC_MIME_TYPE, they will fall back to the MIME format.
        let cookie = unsafe { ffi::magic_open(ffi::MAGIC_EXTENSION | ffi::MAGIC_MIME_TYPE) };
        if cookie.is_null() {
            return Err(Error::Magic(format!("magic_open failed: {:?}", unsafe {
                CStr::from_ptr(ffi::magic_error(cookie))
            })));
        }
        Ok(Self { magic_file, cookie })
    }
}

impl FileTypeDetector for LibMagicDetector {
    fn detect(&mut self, file_path: &Path) -> Result<Vec<String>> {
        let r = match &self.magic_file {
            Some(magic_file) => {
                let mgc = CString::new(magic_file.as_str())?;
                unsafe { ffi::magic_load(self.cookie, mgc.as_ptr()) }
            }
            None => unsafe { ffi::magic_load(self.cookie, std::ptr::null()) },
        };
        if r != 0 {
            return Err(Error::Magic(format!("magic_load failed: {:?}", unsafe {
                CStr::from_ptr(ffi::magic_error(self.cookie))
            })));
        }

        let file_name = CString::new(file_path.to_str().unwrap())?;

        let r = unsafe { ffi::magic_file(self.cookie, file_name.as_ptr()) };
        if r.is_null() {
            return Err(Error::Magic(format!("magic_file failed: {:?}", unsafe {
                CStr::from_ptr(ffi::magic_error(self.cookie))
            })));
        }

        let result = unsafe { CStr::from_ptr(r).to_str()? }
            .split('/')
            .map(|s| s.to_string())
            .collect();

        Ok(result)
    }
}

impl Drop for LibMagicDetector {
    fn drop(&mut self) {
        unsafe {
            ffi::magic_close(self.cookie);
        }
    }
}

#[derive(Debug)]
pub struct LibMagicScanner {
    config: Config,
}

impl LibMagicScanner {
    pub fn new(config: Config) -> Self {
        LibMagicScanner { config }
    }
}

thread_local! {
    static DETECTOR: RefCell<Option<LibMagicDetector>> = const {RefCell::new(None)};
}

impl Scanner for LibMagicScanner {
    fn scan(&self) -> Result<ScanSummary> {
        let paths = WalkBuilder::new(&self.config.file_path)
            .hidden(false)
            .ignore(false)
            .git_ignore(false)
            .parents(false)
            .follow_links(false)
            .max_depth(if self.config.recursive { None } else { Some(1) })
            .build()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_some_and(|t| t.is_file()))
            // Skip empty files.
            .filter(|e| e.metadata().map(|m| m.len()).unwrap_or(0) > 0)
            // Skip files without extensions.
            .filter(|e| {
                e.path()
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .is_some_and(|ext| !ext.is_empty())
            })
            .map(|e| e.into_path())
            .collect::<Vec<_>>();

        let mismatched_files = paths
            .par_iter()
            .map(|path| -> Result<Option<(PathBuf, Vec<String>)>> {
                DETECTOR.with(|detector_rc| {
                    let mut detector_opt = detector_rc.borrow_mut();
                    // libmagic is not thread-safe, so create a separate instance of libmagic for each thread.
                    if detector_opt.is_none() {
                        *detector_opt =
                            Some(LibMagicDetector::build(self.config.magic_file.clone())?);
                    }
                    let detector = detector_opt.as_mut().unwrap();
                    let expected_exts = detector.detect(path)?;
                    let actual_ext = path
                        .extension()
                        .and_then(|e| e.to_str())
                        .ok_or(Error::Other("Failed to get file extension".to_string()))?;
                    let is_mismatch = !expected_exts
                        .iter()
                        .any(|e| e.eq_ignore_ascii_case(actual_ext));
                    if is_mismatch {
                        Ok(Some((path.clone(), expected_exts)))
                    } else {
                        Ok(None)
                    }
                })
            })
            .try_fold(
                Vec::new,
                |mut acc, r| -> Result<Vec<(PathBuf, Vec<String>)>> {
                    if let Some(pair) = r? {
                        acc.push(pair);
                    }
                    Ok(acc)
                },
            )
            .try_reduce(
                Vec::new,
                |mut acc, mut v| -> Result<Vec<(PathBuf, Vec<String>)>> {
                    acc.append(&mut v);
                    Ok(acc)
                },
            )?
            .into_iter()
            .collect::<HashMap<_, _>>();

        Ok(ScanSummary {
            mismatched_files,
            total_num: paths.len(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_jpg() {
        let mut detector = LibMagicDetector::build(None).unwrap();
        let expected = "jpg";
        let actual = detector.detect(Path::new("tests/data/jpg.pdf")).unwrap();
        assert!(actual.iter().any(|e| e == expected));
    }

    #[test]
    #[should_panic]
    fn detect_dir() {
        let mut detector = LibMagicDetector::build(None).unwrap();
        detector.detect(Path::new("tests/data/")).unwrap();
    }
}
