use std::io;

use datasource::{DataSource, dir, zip};
use datasource::DataSource as DataSource_;

use crate::datasource::FileInfo;

mod datasource;

#[cxx::bridge(namespace = "mcrtlib::ffi")]
mod ffi {
    pub struct DataSource {
        pub inner: Box<DataSource_>,
    }

    pub struct DirEntry {
        pub file_name: String,
        pub info: FileInfo,
    }

    pub struct FileInfo {
        pub is_file: bool,
        pub is_dir: bool,
        pub is_symlink: bool,
        pub read_only: bool,
    }

    extern "C" {}

    extern "Rust" {
        type DataSource_;

        fn datasource_open(path: &str) -> Result<DataSource>;

        fn datasource_open_zip(path: &str) -> Result<DataSource>;

        fn create_dir(self: &DataSource, path: &str) -> Result<()>;

        fn create_dir_all(self: &DataSource, path: &str) -> Result<()>;

        fn delete_file(self: &DataSource, path: &str) -> Result<()>;

        fn delete_dir(self: &DataSource, path: &str) -> Result<()>;

        fn delete_dir_all(self: &DataSource, path: &str) -> Result<()>;

        fn list_dir(self: &DataSource, path: &str) -> Result<Vec<DirEntry>>;

        fn read_info(self: &DataSource, path: &str) -> Result<FileInfo>;
    }
}

fn datasource_open(path: &str) -> Result<ffi::DataSource, io::Error> {
    Ok(ffi::DataSource { inner: Box::new(DataSource::Dir(dir::DataSource::new(path)?)) })
}

fn datasource_open_zip(path: &str) -> Result<ffi::DataSource, zip::Error> {
    Ok(ffi::DataSource { inner: Box::new(DataSource::Zip(zip::DataSource::new(path)?)) })
}

impl ffi::DataSource {
    fn create_dir(&self, path: &str) -> Result<(), datasource::Error> {
        self.inner.create_dir(path)
    }

    fn create_dir_all(&self, path: &str) -> Result<(), datasource::Error> {
        self.inner.create_dir_all(path)
    }

    fn delete_file(&self, path: &str) -> Result<(), datasource::Error> {
        self.inner.delete_file(path)
    }

    fn delete_dir(&self, path: &str) -> Result<(), datasource::Error> {
        self.inner.delete_dir(path)
    }

    fn delete_dir_all(&self, path: &str) -> Result<(), datasource::Error> {
        self.inner.delete_dir_all(path)
    }

    fn list_dir(&self, path: &str) -> Result<Vec<ffi::DirEntry>, datasource::Error> {
        self.inner.list_dir(path).map(|v| v.into_iter().map(|a| a.into()).collect())
    }

    fn read_info(&self, path: &str) -> Result<ffi::FileInfo, datasource::Error> {
        self.inner.read_info(path).map(|v| v.into())
    }
}

impl From<datasource::DirEntry> for ffi::DirEntry {
    fn from(e: datasource::DirEntry) -> Self {
        ffi::DirEntry {
            file_name: e.file_name.to_str().expect("invalid characters in file name for UTF-8 string").to_string(),
            info: e.info.into(),
        }
    }
}

impl From<datasource::FileInfo> for ffi::FileInfo {
    fn from(fi: FileInfo) -> Self {
        ffi::FileInfo {
            is_file: fi.is_file,
            is_dir: fi.is_dir,
            is_symlink: fi.is_symlink,
            read_only: fi.read_only,
        }
    }
}