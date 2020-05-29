use std::os::raw::c_char;
use std::ptr::null;
use std::ffi::CString;

#[no_mangle]
pub static mut MCRT_ERROR: McrtError = McrtError::None;

#[no_mangle]
pub static mut MCRT_ERROR_TEXT: *const c_char = null();

#[repr(u8)]
pub enum McrtError {
    None,
    NotFound,
    PermissionDenied,
    Io,
    UnsupportedZip,
    InvalidZip,
    ReadOnly,
    CorruptedFile,
    Nul,
}

#[no_mangle]
pub unsafe extern "C" fn mcrt_str_delete(text: *const c_char) {
    if text.is_null() { return; }
    drop(CString::from_raw(text as *mut c_char));
}