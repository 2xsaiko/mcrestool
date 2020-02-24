use std::ffi::CStr;
use std::io::ErrorKind;
use std::os::raw::c_char;
use std::ptr::{drop_in_place, null_mut};

use zip::result::ZipError;

use crate::datasource::{DataSource, OpenOptions};
use crate::datasource::dir::DirDataSource;
use crate::datasource::resfile::ResFile;
use crate::datasource::zip::{Error, ZipDataSource};
use crate::ffihelper::*;

#[no_mangle]
pub unsafe extern "C" fn datasource_dir_create(path: *const c_char) -> *mut DataSource {
    clear_error();

    if path.is_null() {
        return null_mut();
    }

    let raw = CStr::from_ptr(path);

    let ds = DataSource::Dir(try_ffi!(DirDataSource::new(&*raw.to_string_lossy())));

    Box::into_raw(Box::new(ds))
}

#[no_mangle]
pub unsafe extern "C" fn datasource_zip_create(path: *const c_char) -> *mut DataSource {
    clear_error();

    if path.is_null() {
        return null_mut();
    }

    let raw = CStr::from_ptr(path);

    let ds = try_ffi!(ZipDataSource::new(&*raw.to_string_lossy()));

    let ds = DataSource::Zip(ds);
    Box::into_raw(Box::new(ds))
}

#[no_mangle]
pub unsafe extern "C" fn datasource_open_file(ds: &mut DataSource, path: *const c_char, opts: OpenOptions) -> *mut ResFile {
    clear_error();

    if path.is_null() {
        return null_mut();
    }

    let raw = CStr::from_ptr(path);

    ds.open(&*raw.to_string_lossy(), opts);

    null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn datasource_delete(ds: *mut DataSource) {
    if ds.is_null() { return; }
    drop_in_place(ds);
}