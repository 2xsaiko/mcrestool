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
        Some(pb.strip_prefix("/").unwrap().to_string_lossy().into_owned())
    }
}

pub enum Error {
    InvalidPath(PathBuf),
    IoError(io::Error),
    ZipError(zip::result::ZipError),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IoError(err)
    }
}

impl From<zip::result::ZipError> for Error {
    fn from(err: zip::result::ZipError) -> Self {
        Error::ZipError(err)
    }
}

impl FfiError for Error {
    fn kind(&self) -> McrtError {
        use zip::result::ZipError;

        match self {
            Error::IoError(e) | Error::ZipError(ZipError::Io(e)) => match e.kind() {
                ErrorKind::NotFound => McrtError::NotFound,
                ErrorKind::PermissionDenied => McrtError::PermissionDenied,
                _ => McrtError::IoError
            }
            Error::ZipError(ZipError::UnsupportedArchive(text)) => McrtError::UnsupportedZip,
            Error::ZipError(ZipError::InvalidArchive(text)) => McrtError::InvalidZip,
            Error::ZipError(ZipError::FileNotFound) => McrtError::NotFound,
            _ => McrtError::IoError,
        }
    }

    fn description(&self) -> &str {
        use zip::result::ZipError;

        match self {
            Error::ZipError(ZipError::UnsupportedArchive(text)) => text,
            Error::ZipError(ZipError::InvalidArchive(text)) => text,
            _ => self.kind().description(),
        }
    }
}