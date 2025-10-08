#![allow(unsafe_code)]

use std::{
    cell::RefCell,
    collections::HashMap,
    ffi::{CStr, CString, c_void},
    fs,
    path::{Path, PathBuf},
};

use colored::Colorize;
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
        Ok(LibMagicDetector { magic_file, cookie })
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

    fn inspect_file(
        &self,
        detector: &mut LibMagicDetector,
        file_path: &PathBuf,
    ) -> Result<HashMap<PathBuf, Vec<String>>> {
        // Detect empty files.
        if fs::metadata(file_path).map(|m| m.len()).unwrap_or(0) == 0 {
            return Ok(HashMap::new());
        }

        let expected_exts = detector.detect(file_path)?;
        let actual_ext = file_path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();

        if actual_ext.is_empty() {
            return Ok(HashMap::new());
        }

        let is_mismatch = !expected_exts.iter().any(|e| e == &actual_ext);
        if is_mismatch {
            println!(
                "{}  {} (expected: {} actual: {})",
                "[mismatch]".red(),
                &file_path.display(),
                &expected_exts.join(", ").green(),
                &actual_ext.red(),
            );
            let mut mismatched_files = HashMap::new();
            mismatched_files.insert(file_path.clone(), expected_exts);
            return Ok(mismatched_files);
        }

        Ok(HashMap::new())
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
            .map(|e| e.into_path())
            .collect::<Vec<_>>();

        let mismatched_files = paths
            .par_iter()
            .map(|path| {
                DETECTOR.with(|detector_rc| {
                    let mut detector_opt = detector_rc.borrow_mut();
                    // libmagic is not thread-safe, so create a separate instance of libmagic for each thread.
                    if detector_opt.is_none() {
                        *detector_opt =
                            Some(LibMagicDetector::build(self.config.magic_file.clone()).unwrap());
                    }
                    self.inspect_file(detector_opt.as_mut().unwrap(), path)
                })
            })
            .try_reduce(
                HashMap::new,
                |mut acc_mismatched, res_mismatched| -> Result<HashMap<PathBuf, Vec<String>>> {
                    acc_mismatched.extend(res_mismatched);
                    Ok(acc_mismatched)
                },
            )?;

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
