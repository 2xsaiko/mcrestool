use std::os::raw::c_char;

use crate::restree::FileTree;
use std::ptr::null;
use std::ffi::CString;

#[no_mangle]
pub extern "C" fn filetree_get_name(rt: &FileTree) -> *const c_char {
    null()
}