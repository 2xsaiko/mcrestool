use std::path::Path;
use crate::datasource::DataSource;

mod datasource;
mod ffi;

enum FileType {
    Language,
    LanguagePart,
    Recipe,
}

fn get_file_type<P: AsRef<Path>>(ds: &DataSource, path: P) -> Option<FileType> {
    unimplemented!()
}