use std::{mem, slice};
use std::ffi::{CStr, CString};
use std::io::{Read, Write};
use std::os::raw::{c_char, c_void};
use std::ptr::{drop_in_place, null, null_mut};

use crate::datasource::{DataSource, Error, OpenOptions};
use crate::datasource::dir::DirDataSource;
use crate::datasource::resfile::ResFile;
use crate::datasource::zip::ZipDataSource;
use crate::ffihelper::*;

#[no_mangle]
#[repr(u8)]
pub enum DataSourceType {
    Dir,
    Zip,
}

#[no_mangle]
pub unsafe extern "C" fn datasource_dir_create(path: *const c_char) -> *mut DataSource {
    clear_error();
    if path.is_null() { return null_mut(); }
    let raw = CStr::from_ptr(path);

    let ds = DataSource::Dir(try_ffi!(DirDataSource::new(&*raw.to_string_lossy())));

    Box::into_raw(Box::new(ds))
}

#[no_mangle]
pub unsafe extern "C" fn datasource_zip_create(path: *const c_char) -> *mut DataSource {
    clear_error();
    if path.is_null() { return null_mut(); }
    let raw = CStr::from_ptr(path);

    let ds = DataSource::Zip(try_ffi!(ZipDataSource::new(&*raw.to_string_lossy())));

    Box::into_raw(Box::new(ds))
}

#[no_mangle]
pub extern "C" fn datasource_type(ds: &mut DataSource) -> DataSourceType {
    match ds {
        DataSource::Dir(_) => DataSourceType::Dir,
        DataSource::Zip(_) => DataSourceType::Zip,
    }
}

#[no_mangle]
pub unsafe extern "C" fn datasource_open_file(ds: &mut DataSource, path: *const c_char, opts: OpenOptions) -> *mut ResFile {
    clear_error();
    if path.is_null() { return null_mut(); }
    let raw = CStr::from_ptr(path);

    let file = try_ffi!(ds.open(&*raw.to_string_lossy(), opts));

    Box::into_raw(Box::new(file))
}

#[no_mangle]
pub unsafe extern "C" fn datasource_list_dir(ds: &mut DataSource, path: *const c_char) -> *const *const c_char {
    clear_error();
    if path.is_null() { return null_mut(); }
    let raw = CStr::from_ptr(path);

    let mut dir_list = try_ffi!(ds.list_dir(&*raw.to_string_lossy())).into_iter()
        .map(|s| CString::new(&*s.file_name().unwrap().to_string_lossy()).expect("Invalid 0-char in file name"))
        .collect::<Vec<_>>();

    dir_list.shrink_to_fit();

    let mut p_dir_list = dir_list.into_iter()
        .map(|arg| {
            let ptr = arg.as_ptr();
            mem::forget(arg);
            ptr
        })
        .collect::<Vec<_>>();

    p_dir_list.push(null());
    p_dir_list.shrink_to_fit();

    let ptr = p_dir_list.as_ptr();
    mem::forget(p_dir_list);
    ptr
}

#[no_mangle]
pub unsafe extern "C" fn dirlist_delete(dirlist: *const *const c_char) {
    if dirlist.is_null() { return; }
    let mut len = 1; // start at 1 to include the 0-ptr at the end
    let mut aptr = dirlist;
    while !aptr.is_null() {
        drop(CString::from_raw(*aptr as *mut c_char));
        aptr = aptr.add(1);
        len += 1;
    }
    drop(Vec::from_raw_parts(aptr as *mut *const i8, len, len));
}

#[no_mangle]
pub unsafe extern "C" fn datasource_delete_file(ds: &mut DataSource, path: *const c_char) -> bool {
    clear_error();
    if path.is_null() { return false; }
    let raw = CStr::from_ptr(path);

    try_ffi!(ds.delete_file(&*raw.to_string_lossy()), false);

    true
}

#[no_mangle]
pub unsafe extern "C" fn datasource_delete(ds: *mut DataSource) {
    if ds.is_null() { return; }
    drop_in_place(ds);
}

#[no_mangle]
pub unsafe extern "C" fn resfile_write(data: *const c_void, len: usize, file: &mut ResFile) -> usize {
    clear_error();

    let slice = slice::from_raw_parts(data as *const u8, len);

    try_ffi!(file.write(slice), 0)
}

#[no_mangle]
pub unsafe extern "C" fn resfile_read(data: *mut c_void, len: usize, file: &mut ResFile) -> usize {
    clear_error();

    let slice = slice::from_raw_parts_mut(data as *mut u8, len);

    try_ffi!(file.read(slice), 0)
}

#[no_mangle]
pub unsafe extern "C" fn resfile_flush(file: &mut ResFile) {
    clear_error();

    if let Err(e) = file.flush() {
        set_error_from(e);
    }
}

#[no_mangle]
pub unsafe extern "C" fn resfile_close(file: *mut ResFile) {
    if file.is_null() { return; }
    drop_in_place(file);
}
