use std::cell::RefCell;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::{Path, PathBuf};

use thiserror::Error;
use zip::ZipArchive;

use crate::datasource::normalize_path;

pub struct DataSource {
    archive: RefCell<ZipArchive<File>>,
}

impl DataSource {
    pub fn new(path: impl AsRef<Path>) -> Result<Self, Error> {
        let file = File::open(path)?;
        let za = ZipArchive::new(file)?;
        Ok(DataSource {
            archive: RefCell::new(za),
        })
    }

    pub fn open(&self, path: impl AsRef<Path>) -> Result<Vec<u8>, Error> {
        let path = DataSource::resolve_path_for_archive(&path).ok_or_else(|| Error::InvalidPath(path.as_ref().to_path_buf()))?;
        let mut archive = self.archive.borrow_mut();
        let mut file = archive.by_name(&path)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        Ok(buf)
    }

    pub fn list_dir(&self, path: impl AsRef<Path>) -> Result<Vec<PathBuf>, Error> {
        unimplemented!()
    }

    fn resolve_path_for_archive(path: impl AsRef<Path>) -> Option<String> {
        let pb = normalize_path(path)?;
        Some(pb.strip_prefix("/").unwrap().to_str().unwrap().to_string())
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid path")]
    InvalidPath(PathBuf),
    #[error("I/O error")]
    Io(io::Error),
    #[error("archive error")]
    Zip(zip::result::ZipError),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<zip::result::ZipError> for Error {
    fn from(err: zip::result::ZipError) -> Self {
        Error::Zip(err)
    }
}
