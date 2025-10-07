#![allow(unsafe_code)]

use std::{
    cell::RefCell,
    collections::HashMap,
    ffi::{CStr, CString, c_void},
    fs,
    path::PathBuf,
};

use colored::Colorize;
use ignore::WalkBuilder;
use rayon::prelude::*;

use crate::{
    app::Config,
    detector::FileTypeDetector,
    error::{Error, Result},
    ffi,
    scanner::{Scanner, SummaryInfo},
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
    fn detect(&self, file_path: &str) -> Result<String> {
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

        let file_name = CString::new(file_path)?;

        let r = unsafe { ffi::magic_file(self.cookie, file_name.as_ptr()) };
        if r.is_null() {
            return Err(Error::Magic(format!("magic_file failed: {:?}", unsafe {
                CStr::from_ptr(ffi::magic_error(self.cookie))
            })));
        }

        let result_str = unsafe { CStr::from_ptr(r).to_str()? }.to_string();

        Ok(result_str)
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
        detector: &LibMagicDetector,
        file_path: &PathBuf,
    ) -> Result<(usize, HashMap<String, String>)> {
        let file_name = file_path.to_str().unwrap().to_string();

        // Detect empty files.
        if fs::metadata(file_path).map(|m| m.len()).unwrap_or(0) == 0 {
            return Ok((1, HashMap::new()));
        }

        let expected_exts = detector.detect(&file_name)?;
        let actual_ext = file_path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();

        if actual_ext.is_empty() {
            return Ok((1, HashMap::new()));
        }

        let is_mismatch = !expected_exts.split('/').any(|e| e == actual_ext);
        if is_mismatch {
            println!(
                "{}  {} (expected: {} actual: {})",
                "[mismatch]".red(),
                &file_name,
                &expected_exts.green(),
                &actual_ext.red(),
            );
            let mut mismatched_files = HashMap::new();
            mismatched_files.insert(file_name.clone(), expected_exts);
            return Ok((1, mismatched_files));
        }

        Ok((1, HashMap::new()))
    }
}

thread_local! {
    static DETECTOR: RefCell<Option<LibMagicDetector>> = const {RefCell::new(None)};
}

impl Scanner for LibMagicScanner {
    fn scan(&self) -> Result<SummaryInfo> {
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

        let (total_num, mismatched_files) = paths
            .par_iter()
            .map(|path| {
                DETECTOR.with(|detector_rc| {
                    let mut detector_opt = detector_rc.borrow_mut();
                    // libmagic is not thread-safe, so create a separate instance of libmagic for each thread.
                    if detector_opt.is_none() {
                        *detector_opt =
                            Some(LibMagicDetector::build(self.config.magic_file.clone()).unwrap());
                    }
                    self.inspect_file(detector_opt.as_ref().unwrap(), path)
                })
            })
            .try_reduce(
                || (0, HashMap::new()),
                |acc, res| -> Result<(usize, HashMap<String, String>)> {
                    let (mut acc_total, mut acc_mismatched) = acc;
                    let (t, m) = res;
                    acc_total += t;
                    acc_mismatched.extend(m);
                    Ok((acc_total, acc_mismatched))
                },
            )?;

        Ok(SummaryInfo {
            mismatched_files,
            total_num,
        })
    }
}
