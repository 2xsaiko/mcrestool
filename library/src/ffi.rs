use std::io;

use crate::datasource::{self, DataSource, dir, zip};
use crate::datasource::DataSource as DataSource_;
use crate::datasource::resfile::ResFile as ResFile_;
use std::io::{Read, Write};

#[cxx::bridge(namespace = "mcrtlib::ffi")]
mod types {
    pub struct DataSource {
        pub inner: Box<DataSource_>,
    }

    pub struct ResFile {
        pub inner: Box<ResFile_>,
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

    pub enum FileType {
        FILETYPE_NONE,
        FILETYPE_LANGUAGE,
        FILETYPE_LANGUAGE_PART,
        FILETYPE_RECIPE,
    }

    extern "C" {}

    extern "Rust" {
        type DataSource_;
        type ResFile_;

        fn datasource_open(path: &str) -> Result<DataSource>;

        fn datasource_open_zip(path: &str) -> Result<DataSource>;

        fn get_file_type(ds: &DataSource, path: &str) -> FileType;

        // DataSource
        fn open(self: &DataSource, path: &str, mode: &str) -> Result<ResFile>;

        fn create_dir(self: &DataSource, path: &str) -> Result<()>;

        fn create_dir_all(self: &DataSource, path: &str) -> Result<()>;

        fn delete_file(self: &DataSource, path: &str) -> Result<()>;

        fn delete_dir(self: &DataSource, path: &str) -> Result<()>;

        fn delete_dir_all(self: &DataSource, path: &str) -> Result<()>;

        fn list_dir(self: &DataSource, path: &str) -> Result<Vec<DirEntry>>;

        fn read_info(self: &DataSource, path: &str) -> Result<FileInfo>;

        // ResFile
        // TODO: mutable slice support for cxx
        // fn read(self: &mut ResFile, buf: &mut [u8]) -> Result<usize>;

        fn write(self: &mut ResFile, buf: &[u8]) -> Result<usize>;
    }
}

fn datasource_open(path: &str) -> Result<types::DataSource, io::Error> {
    Ok(types::DataSource { inner: Box::new(DataSource::Dir(dir::DataSource::new(path)?)) })
}

fn datasource_open_zip(path: &str) -> Result<types::DataSource, zip::Error> {
    Ok(types::DataSource { inner: Box::new(DataSource::Zip(zip::DataSource::new(path)?)) })
}

fn get_file_type(ds: &types::DataSource, path: &str) -> types::FileType {
    match crate::get_file_type(&ds.inner, path) {
        None => types::FileType::FILETYPE_NONE,
        Some(t) => t.into(),
    }
}

impl types::DataSource {
    fn open(&self, path: &str, mode: &str) -> Result<types::ResFile, datasource::Error> {
        let mut opts = datasource::OpenOptions::new();

        for char in mode.chars() {
            match char {
                'r' => { opts.read(true); }
                'w' => { opts.write(true); }
                'c' => { opts.create(true); }
                't' => { opts.truncate(true); }
                _ => {}
            }
        }

        Ok(types::ResFile { inner: Box::new(self.inner.open(path, opts)?) })
    }

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

    fn list_dir(&self, path: &str) -> Result<Vec<types::DirEntry>, datasource::Error> {
        self.inner.list_dir(path).map(|v| v.into_iter().map(|a| a.into()).collect())
    }

    fn read_info(&self, path: &str) -> Result<types::FileInfo, datasource::Error> {
        self.inner.read_info(path).map(|v| v.into())
    }
}

impl types::ResFile {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }

    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner.write(buf)
    }
}

impl From<datasource::DirEntry> for types::DirEntry {
    fn from(e: datasource::DirEntry) -> Self {
        types::DirEntry {
            file_name: e.file_name.to_str().expect("invalid characters in file name for UTF-8 string").to_string(),
            info: e.info.into(),
        }
    }
}

impl From<datasource::FileInfo> for types::FileInfo {
    fn from(fi: datasource::FileInfo) -> Self {
        types::FileInfo {
            is_file: fi.is_file,
            is_dir: fi.is_dir,
            is_symlink: fi.is_symlink,
            read_only: fi.read_only,
        }
    }
}

impl From<crate::FileType> for types::FileType {
    fn from(t: crate::FileType) -> Self {
        match t {
            crate::FileType::Language => types::FileType::FILETYPE_LANGUAGE,
            crate::FileType::LanguagePart => types::FileType::FILETYPE_LANGUAGE_PART,
            crate::FileType::Recipe => types::FileType::FILETYPE_RECIPE,
        }
    }
}