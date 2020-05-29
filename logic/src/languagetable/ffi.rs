#![allow(clippy::missing_safety_doc)]

use std::convert::TryFrom;
use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw::c_char;
use std::ptr::{drop_in_place, null, null_mut};

use crate::datasource::DataSource;
use crate::ffihelper::clear_error;
use crate::languagetable::LanguageTable;

#[no_mangle]
pub extern "C" fn languagetable_create() -> *mut LanguageTable {
    Box::into_raw(Box::new(LanguageTable::new()))
}

#[no_mangle]
pub unsafe extern "C" fn languagetable_load_from(ds: &DataSource, dir: *const c_char) -> *mut LanguageTable {
    clear_error();
    if dir.is_null() { return null_mut(); }
    let dir = CStr::from_ptr(dir);

    let lt = try_ffi!(LanguageTable::read_from(ds, dir.to_str().unwrap()));

    Box::into_raw(Box::new(lt))
}

#[no_mangle]
pub unsafe extern "C" fn languagetable_load_into(lt: &mut LanguageTable, ds: &DataSource, dir: *const c_char) -> bool {
    clear_error();
    if dir.is_null() { return false; }
    let dir = CStr::from_ptr(dir);

    *lt = try_ffi!(LanguageTable::read_from(ds, dir.to_str().unwrap()), false);
    true
}

#[no_mangle]
pub unsafe extern "C" fn languagetable_write_to(lt: &LanguageTable, ds: &DataSource, dir: *const c_char) -> bool {
    clear_error();
    if dir.is_null() { return false; }
    let dir = CStr::from_ptr(dir);

    let dir = dir.to_str().unwrap();
    try_ffi!(ds.create_dir_all(dir), false);
    try_ffi!(lt.write_to(ds, dir), false);

    true
}

#[no_mangle]
pub unsafe extern "C" fn languagetable_add_language(lt: &mut LanguageTable, lang: *const c_char) {
    if lang.is_null() { return; }
    let lang = CStr::from_ptr(lang);
    lt.add_language(lang.to_str().unwrap());
}

#[no_mangle]
pub unsafe extern "C" fn languagetable_add_localization_key(lt: &mut LanguageTable, key: *const c_char) {
    if key.is_null() { return; }
    let key = CStr::from_ptr(key);
    lt.add_localization_key(key.to_str().unwrap());
}

#[no_mangle]
pub unsafe extern "C" fn languagetable_insert(lt: &mut LanguageTable, lang: *const c_char, key: *const c_char, name: *const c_char) {
    if lang.is_null() || key.is_null() || name.is_null() { return; }
    let lang = CStr::from_ptr(lang);
    let key = CStr::from_ptr(key);
    let name = CStr::from_ptr(name);
    lt.insert(lang.to_str().unwrap(), key.to_str().unwrap(), name.to_str().unwrap());
}

#[no_mangle]
pub unsafe extern "C" fn languagetable_get(lt: &mut LanguageTable, lang: *const c_char, key: *const c_char) -> *const c_char {
    clear_error();
    if lang.is_null() || key.is_null() { return null(); }
    let lang = CStr::from_ptr(lang);
    let key = CStr::from_ptr(key);
    let entry = lt.get(lang.to_str().unwrap(), key.to_str().unwrap());
    match entry {
        None => null(),
        Some(s) => try_ffi!(CString::new(s)).into_raw(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn languagetable_get_col_name(lt: &mut LanguageTable, idx: usize) -> *const c_char {
    clear_error();
    let entry = lt.column_name(idx);
    match entry {
        None => null(),
        Some(s) => try_ffi!(CString::new(s)).into_raw(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn languagetable_get_row_name(lt: &mut LanguageTable, idx: usize) -> *const c_char {
    clear_error();
    let entry = lt.row_name(idx);
    match entry {
        None => null(),
        Some(s) => try_ffi!(CString::new(s)).into_raw(),
    }
}


#[no_mangle]
pub unsafe extern "C" fn languagetable_row_count(lt: &mut LanguageTable) -> usize {
    lt.row_count()
}

#[no_mangle]
pub unsafe extern "C" fn languagetable_col_count(lt: &mut LanguageTable) -> usize {
    lt.column_count()
}

#[no_mangle]
pub unsafe extern "C" fn languagetable_delete(lt: *mut LanguageTable) {
    drop_in_place(lt);
}