#![feature(str_split_once)]

use std::ffi::OsStr;
use std::path::Path;

use matryoshka::DataSource;

use ffmtutil::{BinSerialize, BinDeserialize};

pub mod gamedata;
pub mod langtable;
pub mod workspace;
#[cfg(feature = "cpp")]
mod ffi;

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

#[derive(BinSerialize)]
struct S {
    inner: String,
    inner1: String,
}

#[derive(BinSerialize)]
struct S1(String);

#[derive(BinSerialize)]
enum E {
    None,
    Some(String),
}