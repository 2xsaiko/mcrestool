use std::cell::RefCell;
use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read};
use std::path::{Path, PathBuf};

use zip::read::ZipFile;
use zip::ZipArchive;

use crate::datasource::normalize_path;
use crate::ffi::McrtError;
use crate::ffihelper::FfiError;

pub struct ZipDataSource {
    archive: RefCell<ZipArchive<File>>,
}

impl ZipDataSource {
    pub fn new(path: impl AsRef<Path>) -> Result<Self, Error> {
        let file = File::open(path)?;
        let za = ZipArchive::new(file)?;
        Ok(ZipDataSource {
            archive: RefCell::new(za),
        })
    }

    pub fn open(&self, path: impl AsRef<Path>) -> Result<Vec<u8>, Error> {
        let path = ZipDataSource::resolve_path_for_archive(&path).ok_or_else(|| Error::InvalidPath(path.as_ref().to_path_buf()))?;
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

pub enum Error {
    InvalidPath(PathBuf),
    Io(io::Error),
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

impl FfiError for Error {
    fn kind(&self) -> McrtError {
        use zip::result::ZipError;

        match self {
            Error::Io(e) | Error::Zip(ZipError::Io(e)) => match e.kind() {
                ErrorKind::NotFound => McrtError::NotFound,
                ErrorKind::PermissionDenied => McrtError::PermissionDenied,
                _ => McrtError::Io
            }
            Error::Zip(ZipError::UnsupportedArchive(text)) => McrtError::UnsupportedZip,
            Error::Zip(ZipError::InvalidArchive(text)) => McrtError::InvalidZip,
            Error::Zip(ZipError::FileNotFound) => McrtError::NotFound,
            _ => McrtError::Io,
        }
    }

    fn description(&self) -> &str {
        use zip::result::ZipError;

        match self {
            Error::Zip(ZipError::UnsupportedArchive(text)) => text,
            Error::Zip(ZipError::InvalidArchive(text)) => text,
            _ => self.kind().description(),
        }
    }
}