use std::ffi::CString;
use std::io::ErrorKind;
use std::os::raw::c_char;
use std::ptr::null;

use crate::ffi::{MCRT_ERROR, MCRT_ERROR_TEXT, McrtError};

unsafe fn delete_error_text() {
    if !MCRT_ERROR_TEXT.is_null() {
        drop(CString::from_raw(MCRT_ERROR_TEXT as *mut c_char));
        MCRT_ERROR_TEXT = null();
    }
}

pub unsafe fn clear_error() {
    MCRT_ERROR = McrtError::None;
    delete_error_text();
}

pub unsafe fn set_error(error: McrtError, text: &str) {
    MCRT_ERROR = error;
    delete_error_text();
    MCRT_ERROR_TEXT = CString::new(text).expect("Failed to convert error text to C string").into_raw();
}

pub trait FfiError {
    fn kind(&self) -> McrtError;

    fn description(&self) -> &str {
        self.kind().description()
    }
}

impl McrtError {
    pub fn description(&self) -> &'static str {
        match self {
            McrtError::None => "",
            McrtError::NotFound => "File not found",
            McrtError::PermissionDenied => "Permission denied",
            McrtError::IoError => "I/O Error",
            McrtError::UnsupportedZip => "Unsupported ZIP archive",
            McrtError::InvalidZip => "Invalid ZIP archive",
            McrtError::ReadOnly => "Filesystem is read-only",
            McrtError::CorruptedFile => "Corrupted file",
            McrtError::NulError => "0-byte found in string",
        }
    }
}

pub unsafe fn set_error_from(error: impl FfiError) {
    set_error(error.kind(), error.description());
}

macro_rules! try_ffi {
    ($e:expr) => {
        try_ffi!($e, ::std::ptr::null_mut());
    };
    ($e:expr, $rv:expr) => {
        match $e {
            ::std::result::Result::Ok(v) => v,
            ::std::result::Result::Err(e) => {
                $crate::ffihelper::set_error_from(e);
                return $rv;
            }
        }
    }
}

impl FfiError for std::io::Error {
    fn kind(&self) -> McrtError {
        match self.kind() {
            ErrorKind::NotFound => McrtError::NotFound,
            ErrorKind::PermissionDenied => McrtError::PermissionDenied,
            _ => McrtError::IoError
        }
    }
}

impl FfiError for std::ffi::NulError {
    fn kind(&self) -> McrtError {
        McrtError::NulError
    }
}