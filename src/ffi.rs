#![allow(unsafe_code)]
#![allow(dead_code)]

use std::ffi::{c_char, c_int, c_void};

// https://github.com/file/file/blob/master/src/magic.h.in

/// No flags
pub const MAGIC_NONE: c_int = 0x0000000;
/// Turn on debugging
pub const MAGIC_DEBUG: c_int = 0x0000001;
/// Follow symlinks
pub const MAGIC_SYMLINK: c_int = 0x0000002;
/// Check inside compressed files
pub const MAGIC_COMPRESS: c_int = 0x0000004;
/// Look at the contents of devices
pub const MAGIC_DEVICES: c_int = 0x0000008;
/// Return the MIME type
pub const MAGIC_MIME_TYPE: c_int = 0x0000010;
/// Return all matches
pub const MAGIC_CONTINUE: c_int = 0x0000020;
/// Print warnings to stderr
pub const MAGIC_CHECK: c_int = 0x0000040;
/// Restore access time on exit
pub const MAGIC_PRESERVE_ATIME: c_int = 0x0000080;
/// Don't convert unprintable chars
pub const MAGIC_RAW: c_int = 0x0000100;
/// Handle ENOENT etc as real errors
pub const MAGIC_ERROR: c_int = 0x0000200;
/// Return the MIME encoding
pub const MAGIC_MIME_ENCODING: c_int = 0x0000400;
/// Combined: MIME type and MIME encoding
pub const MAGIC_MIME: c_int = MAGIC_MIME_TYPE | MAGIC_MIME_ENCODING;
/// Return the Apple creator/type
pub const MAGIC_APPLE: c_int = 0x0000800;
/// Return a /-separated list of extensions
pub const MAGIC_EXTENSION: c_int = 0x1000000;
/// Check inside compressed files but not report compression
pub const MAGIC_COMPRESS_TRANSP: c_int = 0x2000000;
/// Don't allow decompression that needs to fork
pub const MAGIC_NO_COMPRESS_FORK: c_int = 0x4000000;
/// Combined: extension, mime, apple
pub const MAGIC_NODESC: c_int = MAGIC_EXTENSION | MAGIC_MIME | MAGIC_APPLE;
/// Don't check for compressed files
pub const MAGIC_NO_CHECK_COMPRESS: c_int = 0x0001000;
/// Don't check for tar files
pub const MAGIC_NO_CHECK_TAR: c_int = 0x0002000;
/// Don't check magic entries
pub const MAGIC_NO_CHECK_SOFT: c_int = 0x0004000;
/// Don't check application type
pub const MAGIC_NO_CHECK_APPTYPE: c_int = 0x0008000;
/// Don't check for elf details
pub const MAGIC_NO_CHECK_ELF: c_int = 0x0010000;
/// Don't check for text files
pub const MAGIC_NO_CHECK_TEXT: c_int = 0x0020000;
/// Don't check for cdf files
pub const MAGIC_NO_CHECK_CDF: c_int = 0x0040000;
/// Don't check for CSV files
pub const MAGIC_NO_CHECK_CSV: c_int = 0x0080000;
/// Don't check tokens
pub const MAGIC_NO_CHECK_TOKENS: c_int = 0x0100000;
/// Don't check text encodings
pub const MAGIC_NO_CHECK_ENCODING: c_int = 0x0200000;
/// Don't check for JSON files
pub const MAGIC_NO_CHECK_JSON: c_int = 0x0400000;
/// Don't check for SIMH tape files
pub const MAGIC_NO_CHECK_SIMH: c_int = 0x0800000;

// https://man7.org/linux/man-pages/man3/libmagic.3.html
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
