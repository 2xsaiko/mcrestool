use std::os::raw::c_char;

#[no_mangle]
pub static mut MCRT_ERROR: McrtError = McrtError::None;

#[no_mangle]
pub static mut MCRT_ERROR_TEXT: [c_char; 128] = [0; 128];

#[repr(u8)]
pub enum McrtError {
    None,
    NotFound,
    PermissionDenied,
    IoError,
    UnsupportedZip,
    InvalidZip,
}