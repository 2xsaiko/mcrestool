use std::ffi::CString;
use std::io::ErrorKind;
use std::mem;

use crate::ffi::{MCRT_ERROR, MCRT_ERROR_TEXT, McrtError};
pub use crate::ffi::McrtError as FfiErrorKind;

pub unsafe fn clear_error() {
    MCRT_ERROR = McrtError::None;
    MCRT_ERROR_TEXT[0] = 0;
}

pub unsafe fn set_error(error: McrtError, text: &str) {
    let cstr = CString::new(text).unwrap();
    let data = cstr.to_bytes_with_nul();
    MCRT_ERROR = error;
    MCRT_ERROR_TEXT[0..data.len()].copy_from_slice(mem::transmute(data));
}

pub trait FfiError {
    fn kind(&self) -> FfiErrorKind;

    fn description(&self) -> &str;
}

pub unsafe fn set_error_from(error: impl FfiError) {
    set_error(error.kind(), error.description());
}

macro_rules! try_ffi {
    ($e:expr) => {
        match $e {
            ::std::result::Result::Ok(v) => v,
            ::std::result::Result::Err(e) => {
                $crate::ffihelper::set_error_from(e);
                return ::std::ptr::null_mut();
            }
        }
    };
}

impl FfiError for std::io::Error {
    fn kind(&self) -> FfiErrorKind {
        match self.kind() {
            ErrorKind::NotFound => FfiErrorKind::NotFound,
            ErrorKind::PermissionDenied => FfiErrorKind::PermissionDenied,
            _ => FfiErrorKind::IoError
        }
    }

    fn description(&self) -> &str {
        match self.kind() {
            ErrorKind::NotFound => "File or directory not found",
            ErrorKind::PermissionDenied => "Permission denied",
            _ => "I/O Error"
        }
    }
}