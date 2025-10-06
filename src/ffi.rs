#![allow(unsafe_code)]

use std::ffi::{c_char, c_int, c_void};

/// Returns a separated list of extensions
pub const MAGIC_EXTENSION_FLAGS: c_int = 0x1000000;

// https://manpages.debian.org/bookworm/libmagic-dev/libmagic.3.en.html
#[link(name = "magic")]
unsafe extern "C" {
    // magic_t magic_open(int flags)
    pub fn magic_open(flags: c_int) -> *mut c_void;

    // int magic_load(magic_t cookie, const char *filename)
    pub fn magic_load(cookie: *mut c_void, filename: *const c_char) -> c_int;

    // void magic_close(magic_t cookie)
    pub fn magic_close(cookie: *mut c_void);

    // const char * magic_file(magic_t cookie, const char *filename)
    pub fn magic_file(cookie: *mut c_void, filename: *const c_char) -> *const c_char;

    // const char * magic_error(magic_t cookie)
    pub fn magic_error(cookie: *mut c_void) -> *const c_char;
}
