use std::fs;
use std::io::{Cursor, ErrorKind};
use std::path::{Component, Path, PathBuf};

use ::zip::result::ZipError;

use crate::datasource::dir::DirDataSource;
use crate::datasource::resfile::ResFile;
use crate::datasource::zip::ZipDataSource;
use crate::ffi::McrtError;
use crate::ffihelper::FfiError;

pub mod dir;
pub mod zip;
pub mod resfile;

pub mod ffi;

pub enum DataSource {
    Dir(DirDataSource),
    Zip(ZipDataSource),
}

impl DataSource {
    pub fn open(&self, path: impl AsRef<Path>, opts: OpenOptions) -> Result<ResFile, Error> {
        match self {
            DataSource::Dir(ds) => {
                Ok(ResFile::File(ds.open(path, opts.into())?))
            }
            DataSource::Zip(ds) => {
                if opts.write {
                    Err(Error::PermissionDenied)
                } else {
                    let result: Result<Vec<u8>, Error> = ds.open(path).map_err(|e| e.into())
                        .map_err(|e| match e {
                            Error::NotFound if opts.create => Error::ReadOnly,
                            x => x
                        });
                    Ok(ResFile::ZipEntry(Cursor::new(result?)))
                }
            }
        }
    }

    pub fn create_dir(&self, path: impl AsRef<Path>) -> Result<(), Error> {
        match self {
            DataSource::Dir(ds) => Ok(ds.create_dir(path)?),
            DataSource::Zip(_) => Err(Error::ReadOnly),
        }
    }

    pub fn create_dir_all(&self, path: impl AsRef<Path>) -> Result<(), Error> {
        match self {
            DataSource::Dir(ds) => Ok(ds.create_dir_all(path)?),
            DataSource::Zip(_) => Err(Error::ReadOnly),
        }
    }

    pub fn delete_file(&self, path: impl AsRef<Path>) -> Result<(), Error> {
        match self {
            DataSource::Dir(ds) => Ok(ds.delete_file(path)?),
            DataSource::Zip(_) => Err(Error::ReadOnly),
        }
    }

    pub fn delete_dir(&self, path: impl AsRef<Path>) -> Result<(), Error> {
        match self {
            DataSource::Dir(ds) => Ok(ds.delete_dir(path)?),
            DataSource::Zip(_) => Err(Error::ReadOnly),
        }
    }

    pub fn delete_dir_all(&self, path: impl AsRef<Path>) -> Result<(), Error> {
        match self {
            DataSource::Dir(ds) => Ok(ds.delete_dir_all(path)?),
            DataSource::Zip(_) => Err(Error::ReadOnly),
        }
    }

    pub fn list_dir(&self, path: impl AsRef<Path>) -> Result<Vec<PathBuf>, Error> {
        match self {
            DataSource::Dir(ds) => Ok(ds.list_dir(path)?),
            DataSource::Zip(ds) => Ok(ds.list_dir(path)?),
        }
    }
}

pub enum Error {
    InvalidPath(PathBuf),
    NotFound,
    PermissionDenied,
    ReadOnly,
    IoError,
}

impl From<dir::Error> for Error {
    fn from(err: dir::Error) -> Self {
        match err {
            dir::Error::RootDirNotFound(_) => unreachable!(),
            dir::Error::InvalidPath(p) => Error::InvalidPath(p),
            dir::Error::IoError(e) => {
                match e.kind() {
                    ErrorKind::NotFound => Error::NotFound,
                    ErrorKind::PermissionDenied => Error::PermissionDenied,
                    _ => Error::IoError
                }
            }
        }
    }
}

impl From<zip::Error> for Error {
    fn from(err: zip::Error) -> Self {
        match err {
            zip::Error::ZipError(ZipError::FileNotFound) => Error::NotFound,
            zip::Error::ZipError(ZipError::InvalidArchive(_)) |
            zip::Error::ZipError(ZipError::UnsupportedArchive(_)) => Error::IoError,
            zip::Error::ZipError(ZipError::Io(e)) => {
                match e.kind() {
                    ErrorKind::NotFound => Error::NotFound,
                    ErrorKind::PermissionDenied => Error::PermissionDenied,
                    _ => Error::IoError
                }
            }
            zip::Error::IoError(_) => unreachable!(),
            zip::Error::InvalidPath(p) => Error::InvalidPath(p)
        }
    }
}

impl FfiError for Error {
    fn kind(&self) -> McrtError {
        match self {
            Error::InvalidPath(_) => McrtError::NotFound,
            Error::NotFound => McrtError::NotFound,
            Error::PermissionDenied => McrtError::PermissionDenied,
            Error::ReadOnly => McrtError::ReadOnly,
            Error::IoError => McrtError::IoError,
        }
    }
}

pub fn normalize_path(path: impl AsRef<Path>) -> Option<PathBuf> {
    let mut pb = PathBuf::from("/");
    for c in path.as_ref().components() {
        match c {
            Component::Prefix(_) => return None,
            Component::RootDir => {}
            Component::CurDir => {}
            Component::ParentDir => {
                pb.pop();
            }
            Component::Normal(s) => pb.push(s),
        }
    }
    Some(pb)
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct OpenOptions {
    read: bool,
    write: bool,
    create: bool,
}

impl Default for OpenOptions {
    fn default() -> Self {
        OpenOptions::reading()
    }
}

impl OpenOptions {
    pub fn reading() -> OpenOptions {
        OpenOptions { read: true, write: false, create: false }
    }

    pub fn writing(create: bool) -> OpenOptions {
        OpenOptions { read: false, write: true, create }
    }

    pub fn read(&mut self, read: bool) -> &mut OpenOptions {
        self.read = read;
        self
    }

    pub fn write(&mut self, write: bool) -> &mut OpenOptions {
        self.write = write;
        self
    }

    pub fn create(&mut self, create: bool) -> &mut OpenOptions {
        self.create = create;
        self
    }
}

impl Into<fs::OpenOptions> for OpenOptions {
    fn into(self) -> fs::OpenOptions {
        let mut options = fs::OpenOptions::new();
        options.read(self.read);
        options.write(self.write);
        options.create(self.create);
        options
    }
}

pub struct DirEntry {
    pub is_file: bool,
    pub is_dir: bool,
    pub is_symlink: bool,
    pub name: String,
}