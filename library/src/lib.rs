use std::io;
use std::ops::Deref;
use std::path::PathBuf;

use datasource::{DataSource, dir, zip};
use datasource::DataSource as DataSource_;

mod datasource;

#[cxx::bridge(namespace = "mcrtlib::ffi")]
mod ffi {
    pub struct DataSource {
        pub inner: Box<DataSource_>,
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

        fn list_dir(self: &DataSource, path: &str) -> Result<Vec<String>>;
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

    fn list_dir(&self, path: &str) -> Result<Vec<String>, datasource::Error> {
        match self.inner.list_dir(path) {
            Err(e) => Err(e),
            Ok(v) => Ok(v.into_iter()
                .map(|a| a.to_str().expect("failed to convert to string").to_string())
                .collect())
        }
    }
}