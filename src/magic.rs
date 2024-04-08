use anyhow::{bail, Result};
use std::ffi::{c_char, c_int, c_void, CStr, CString};

const MAGIC_EXTENSION: c_int = 0x1000000; // Returns a separated list of extensions

// https://manpages.debian.org/bookworm/libmagic-dev/libmagic.3.en.html
#[link(name = "magic")]
extern "C" {
    // magic_t is an opaque type.

    // magic_t magic_open(int flags)
    fn magic_open(flags: c_int) -> *mut c_void;

    // int magic_load(magic_t cookie, const char *filename)
    fn magic_load(cookie: *mut c_void, filename: *const c_char) -> c_int;

    // void magic_close(magic_t cookie)
    fn magic_close(cookie: *mut c_void);

    // const char * magic_file(magic_t cookie, const char *filename)
    fn magic_file(cookie: *mut c_void, filename: *const c_char) -> *const c_char;

    // const char * magic_error(magic_t cookie)
    fn magic_error(cookie: *mut c_void) -> *const c_char;
}

pub fn get_exts(filename: &str) -> Result<String> {
    let flags = MAGIC_EXTENSION;
    let cookie = unsafe { magic_open(flags) };
    if cookie.is_null() {
        let error = unsafe { magic_error(cookie) };
        bail!("magic_open failed: {:?}", unsafe { CStr::from_ptr(error) });
    }

    let result = unsafe { magic_load(cookie, std::ptr::null()) };
    if result != 0 {
        let error = unsafe { magic_error(cookie) };
        bail!("magic_load failed: {:?}", unsafe { CStr::from_ptr(error) });
    }

    let file_name = CString::new(filename)?;

    let result = unsafe { magic_file(cookie, file_name.as_ptr()) };
    if result.is_null() {
        let error = unsafe { magic_error(cookie) };
        bail!("magic_file failed: {:?}", unsafe { CStr::from_ptr(error) });
    }

    let result_str = unsafe { CStr::from_ptr(result).to_str()? }.to_string();

    unsafe { magic_close(cookie) };

    Ok(result_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_exts_dir() {
        let filename = "tests/data/";
        assert!(get_exts(filename).is_err());
    }

    #[test]
    fn test_get_exts_file() {
        let filename = "tests/data/jpg.pdf";
        assert!(get_exts(filename).unwrap().contains("jpg"));
    }
}
