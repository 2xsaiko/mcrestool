#![feature(str_split_once)]

use std::ffi::OsStr;
use std::path::Path;

use matryoshka::DataSource;

pub mod gamedata;
pub mod langtable;
pub mod workspace;
pub mod ident;
#[cfg(feature = "cpp")]
mod ffi;

mod binserde;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum FileType {
    Language,
    LanguagePart,
    Recipe,
}

pub fn get_file_type<P: AsRef<Path>>(ds: &DataSource, path: P) -> Option<FileType> {
    // shitty detection for now
    let path = path.as_ref();
    if ds.is_file(path) && has_extension(path, "json") && path.parent().and_then(|p| get_file_type(ds, p)) == Some(FileType::Language) {
        Some(FileType::LanguagePart)
    } else if ds.is_dir(path) && has_file_name(path, "lang") {
        Some(FileType::Language)
    } else {
        None
    }
}

fn has_extension<P: AsRef<Path>, S: AsRef<OsStr>>(path: P, ext: S) -> bool {
    path.as_ref().extension().map_or(false, |s| s == ext.as_ref())
}

fn has_file_name<P: AsRef<Path>, S: AsRef<OsStr>>(path: P, name: S) -> bool {
    path.as_ref().file_name().map_or(false, |s| s == name.as_ref())
}

fn encode_min(num: i32) -> u32 {
    let u_num = num as u32;
    (u_num << 1 ^ (num >> 31) as u32) | u_num >> 31
}

fn unencode_min(num: u32) -> i32 {
    (num >> 1) as i32 ^ ((num << 31) as i32) >> 31
}

#[test]
fn test_encode_min() {
    for i in -5..5 {
        let r = encode_min(i);
        println!("{} = {:08X}", i, r);
        assert_eq!(i, unencode_min(r));
    }

    for i in i32::MAX-5..=i32::MAX {
        let r = encode_min(i);
        println!("{} = {:08X}", i, r);
        assert_eq!(i, unencode_min(r));
    }

    for i in i32::MIN..i32::MIN+5 {
        let r = encode_min(i);
        println!("{} = {:08X}", i, r);
        assert_eq!(i, unencode_min(r));
    }
}