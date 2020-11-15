use std::ffi::OsStr;
use std::fs;
use std::io::{Cursor, ErrorKind};
use std::path::{Component, Path, PathBuf};

use ::zip::result::ZipError;
use thiserror::Error;

use resfile::ResFile;

pub mod dir;
pub mod zip;
pub mod resfile;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub enum DataSource {
    Dir(dir::DataSource),
    Zip(zip::DataSource),
}

impl DataSource {
    pub fn open<P: AsRef<Path>>(&self, path: P, opts: OpenOptions) -> Result<ResFile> {
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

    pub fn create_dir<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        match self {
            DataSource::Dir(ds) => Ok(ds.create_dir(path)?),
            DataSource::Zip(_) => Err(Error::ReadOnly),
        }
    }

    pub fn create_dir_all<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        match self {
            DataSource::Dir(ds) => Ok(ds.create_dir_all(path)?),
            DataSource::Zip(_) => Err(Error::ReadOnly),
        }
    }

    pub fn delete_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        match self {
            DataSource::Dir(ds) => Ok(ds.delete_file(path)?),
            DataSource::Zip(_) => Err(Error::ReadOnly),
        }
    }

    pub fn delete_dir<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        match self {
            DataSource::Dir(ds) => Ok(ds.delete_dir(path)?),
            DataSource::Zip(_) => Err(Error::ReadOnly),
        }
    }

    pub fn delete_dir_all<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        match self {
            DataSource::Dir(ds) => Ok(ds.delete_dir_all(path)?),
            DataSource::Zip(_) => Err(Error::ReadOnly),
        }
    }

    pub fn list_dir<P: AsRef<Path>>(&self, path: P) -> Result<Vec<DirEntry>> {
        match self {
            DataSource::Dir(ds) => Ok(ds.list_dir(path)?),
            DataSource::Zip(ds) => Ok(ds.list_dir(path)?),
        }
    }

    pub fn read_info<P: AsRef<Path>>(&self, path: P) -> Result<FileInfo> {
        match self {
            DataSource::Dir(ds) => Ok(ds.read_info(path)?),
            DataSource::Zip(ds) => Ok(ds.read_info(path)?),
        }
    }

    pub fn is_file<P: AsRef<Path>>(&self, path: P) -> bool {
        self.read_info(path).map(|i| i.is_file).unwrap_or(false)
    }

    pub fn is_dir<P: AsRef<Path>>(&self, path: P) -> bool {
        self.read_info(path).map(|i| i.is_dir).unwrap_or(false)
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid path")]
    InvalidPath(PathBuf),
    #[error("file or directory not found")]
    NotFound,
    #[error("permission denied")]
    PermissionDenied,
    #[error("the specified file is read only")]
    ReadOnly,
    #[error("I/O error")]
    Io,
}

impl From<dir::Error> for Error {
    fn from(err: dir::Error) -> Self {
        match err {
            dir::Error::RootDirNotFound(_) => unreachable!(),
            dir::Error::InvalidPath(p) => Error::InvalidPath(p),
            dir::Error::Io(e) => {
                match e.kind() {
                    ErrorKind::NotFound => Error::NotFound,
                    ErrorKind::PermissionDenied => Error::PermissionDenied,
                    e => {
                        eprintln!("unhandled error: {:?}", e);
                        Error::Io
                    }
                }
            }
        }
    }
}

impl From<zip::Error> for Error {
    fn from(err: zip::Error) -> Self {
        match err {
            zip::Error::Zip(ZipError::FileNotFound) => Error::NotFound,
            zip::Error::Zip(ZipError::InvalidArchive(_)) |
            zip::Error::Zip(ZipError::UnsupportedArchive(_)) => Error::Io,
            zip::Error::Zip(ZipError::Io(e)) | zip::Error::Io(e) => {
                match e.kind() {
                    ErrorKind::NotFound => Error::NotFound,
                    ErrorKind::PermissionDenied => Error::PermissionDenied,
                    _ => Error::Io
                }
            }
            zip::Error::InvalidPath(p) => Error::InvalidPath(p)
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
    append: bool,
}

impl Default for OpenOptions {
    fn default() -> Self {
        OpenOptions::reading()
    }
}

impl OpenOptions {
    pub fn new() -> OpenOptions {
        OpenOptions { read: false, write: false, create: false, append: false }
    }

    pub fn reading() -> OpenOptions {
        OpenOptions { read: true, write: false, create: false, append: false }
    }

    pub fn writing(create: bool) -> OpenOptions {
        OpenOptions { read: false, write: true, create, append: false }
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

    pub fn append(&mut self, append: bool) -> &mut OpenOptions {
        self.append = append;
        self
    }
}

impl Into<fs::OpenOptions> for OpenOptions {
    fn into(self) -> fs::OpenOptions {
        let mut options = fs::OpenOptions::new();
        options.read(self.read);
        options.write(self.write);
        options.create(self.create);
        options.append(self.write && self.append);
        options.truncate(self.write && !self.append);
        options
    }
}

#[derive(Debug)]
pub struct DirEntry {
    pub path: PathBuf,
    pub info: FileInfo,
}

impl DirEntry {
    pub fn file_name(&self) -> &OsStr {
        self.path.file_name().unwrap()
    }
}

#[derive(Debug)]
pub struct FileInfo {
    pub is_file: bool,
    pub is_dir: bool,
    pub is_symlink: bool,
    pub read_only: bool,
}