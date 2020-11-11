use std::ffi::OsStr;
use std::path::Path;

use crate::datasource::DataSource;

pub mod datasource;
mod ffi;

#[derive(Eq, PartialEq)]
enum FileType {
    Language,
    LanguagePart,
    Recipe,
}

fn get_file_type<P: AsRef<Path>>(ds: &DataSource, path: P) -> Option<FileType> {
    // shitty detection for now
    let path = path.as_ref();
    if ds.is_file(path) && path.extension().map_or(false, |s| s == OsStr::new("json")) && path.parent().and_then(|p| get_file_type(ds, p)) == Some(FileType::Language) {
        Some(FileType::LanguagePart)
    } else if ds.is_dir(path) && path.file_name().map_or(false, |s| s == OsStr::new("lang")) {
        Some(FileType::Language)
    } else {
        None
    }
}