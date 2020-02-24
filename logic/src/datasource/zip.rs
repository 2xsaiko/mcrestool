use std::cell::RefCell;
use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read};
use std::path::{Path, PathBuf};

use zip::read::ZipFile;
use zip::ZipArchive;

use crate::datasource::normalize_path;
use crate::ffihelper::{FfiError, FfiErrorKind};

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
    fn kind(&self) -> FfiErrorKind {
        use zip::result::ZipError;

        match self {
            Error::IoError(e) | Error::ZipError(ZipError::Io(e)) => match e.kind() {
                ErrorKind::NotFound => FfiErrorKind::NotFound,
                ErrorKind::PermissionDenied => FfiErrorKind::PermissionDenied,
                _ => FfiErrorKind::IoError
            }
            Error::ZipError(ZipError::UnsupportedArchive(text)) => FfiErrorKind::UnsupportedZip,
            Error::ZipError(ZipError::InvalidArchive(text)) => FfiErrorKind::InvalidZip,
            Error::ZipError(ZipError::FileNotFound) => FfiErrorKind::NotFound,
            _ => FfiErrorKind::IoError,
        }
    }

    fn description(&self) -> &str {
        use zip::result::ZipError;

        match self {
            Error::IoError(e) | Error::ZipError(ZipError::Io(e)) => match e.kind() {
                ErrorKind::NotFound => "File not found",
                ErrorKind::PermissionDenied => "Permission denied",
                _ => "I/O Error"
            }
            Error::ZipError(ZipError::UnsupportedArchive(text)) => text,
            Error::ZipError(ZipError::InvalidArchive(text)) => text,
            Error::ZipError(ZipError::FileNotFound) => "File not found",
            _ => "I/O Error",
        }
    }
}