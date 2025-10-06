#![allow(unsafe_code)]

use std::ffi::{CStr, CString, c_void};

use crate::{
    detector::FileTypeDetector,
    error::{Error, Result},
    ffi,
};

pub struct LibMagicDetector {
    cookie: *mut c_void,
}

impl LibMagicDetector {
    pub fn build() -> Result<Self> {
        let cookie = unsafe { ffi::magic_open(ffi::MAGIC_EXTENSION_FLAGS) };
        if cookie.is_null() {
            return Err(Error::Other(format!("magic_open failed: {:?}", unsafe {
                CStr::from_ptr(ffi::magic_error(cookie))
            })));
        }
        Ok(LibMagicDetector { cookie })
    }
}

impl FileTypeDetector for LibMagicDetector {
    fn detect(&self, filename: &str) -> Result<String> {
        let result = unsafe { ffi::magic_load(self.cookie, std::ptr::null()) };
        if result != 0 {
            return Err(Error::Other(format!("magic_load failed: {:?}", unsafe {
                CStr::from_ptr(ffi::magic_error(self.cookie))
            })));
        }

        let file_name = CString::new(filename)?;

        let result = unsafe { ffi::magic_file(self.cookie, file_name.as_ptr()) };
        if result.is_null() {
            return Err(Error::Other(format!("magic_file failed: {:?}", unsafe {
                CStr::from_ptr(ffi::magic_error(self.cookie))
            })));
        }

        let result_str = unsafe { CStr::from_ptr(result).to_str()? }.to_string();

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
