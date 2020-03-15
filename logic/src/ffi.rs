use std::os::raw::c_char;
use std::ptr::null;

#[no_mangle]
pub static mut MCRT_ERROR: McrtError = McrtError::None;

#[no_mangle]
pub static mut MCRT_ERROR_TEXT: *const c_char = null();

#[repr(u8)]
pub enum McrtError {
    None,
    NotFound,
    PermissionDenied,
    IoError,
    UnsupportedZip,
    InvalidZip,
    ReadOnly,
    CorruptedFile,
    NulError,
}