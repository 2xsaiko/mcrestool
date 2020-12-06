use std::ffi::OsStr;
use std::io::Cursor;
use std::path::{Component, Path, PathBuf};
use std::{fs, io};

use thiserror::Error;

use resfile::ResFile;

pub mod dir;
pub mod resfile;
pub mod zip;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub enum DataSource {
    Dir(dir::DataSource),
    Zip(zip::DataSource),
}

impl DataSource {
    pub fn new_dir<P: Into<PathBuf>>(path: P) -> Result<Self> {
        Ok(DataSource::Dir(dir::DataSource::new(path)?))
    }

    pub fn new_zip<P: AsRef<Path>>(path: P) -> Result<Self> {
        Ok(DataSource::Zip(zip::DataSource::new(path)?))
    }

    /// Opens a file at `path` inside of this `DataSource`.
    pub fn open<P: AsRef<Path>>(&self, path: P, opts: OpenOptions) -> Result<ResFile> {
        match self {
            DataSource::Dir(ds) => Ok(ResFile::File(ds.open(path, opts.into())?)),
            DataSource::Zip(ds) => {
                if opts.write {
                    Err(Error::PermissionDenied)
                } else {
                    let path = path.as_ref();
                    let result: Result<Vec<u8>, Error> =
                        ds.open(path).map_err(|e| e.into()).map_err(|e| match e {
                            Error::NotFound if opts.create => Error::ReadOnly(path.to_path_buf()),
                            x => x,
                        });
                    Ok(ResFile::ZipEntry(Cursor::new(result?)))
                }
            }
        }
    }

    /// Creates a directory at `path` inside of this `DataSource`.
    pub fn create_dir<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        match self {
            DataSource::Dir(ds) => Ok(ds.create_dir(path)?),
            DataSource::Zip(_) => Err(Error::ReadOnly(path.as_ref().to_path_buf())),
        }
    }

    /// Creates a directory including all its parent directories at `path`
    /// inside of this `DataSource`.
    pub fn create_dir_all<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        match self {
            DataSource::Dir(ds) => Ok(ds.create_dir_all(path)?),
            DataSource::Zip(_) => Err(Error::ReadOnly(path.as_ref().to_path_buf())),
        }
    }

    /// Deletes the file specified by `path` inside of this `DataSource`.
    pub fn delete_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        match self {
            DataSource::Dir(ds) => Ok(ds.delete_file(path)?),
            DataSource::Zip(_) => Err(Error::ReadOnly(path.as_ref().to_path_buf())),
        }
    }

    /// Deletes the directory specified by `path` inside of this `DataSource`.
    /// This will fail if the directory is not empty.
    pub fn delete_dir<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        match self {
            DataSource::Dir(ds) => Ok(ds.delete_dir(path)?),
            DataSource::Zip(_) => Err(Error::ReadOnly(path.as_ref().to_path_buf())),
        }
    }

    /// Deletes the directory specified by `path` inside of this `DataSource`,
    /// including all files and directories contained within, recursively.
    pub fn delete_dir_all<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        match self {
            DataSource::Dir(ds) => Ok(ds.delete_dir_all(path)?),
            DataSource::Zip(_) => Err(Error::ReadOnly(path.as_ref().to_path_buf())),
        }
    }

    /// Returns a list of contents of the directory specified by `path`.
    ///
    /// As with [`std::fs::read_dir`], the order in which this iterator returns
    /// entries is platform and filesystem dependent.
    pub fn list_dir<P: AsRef<Path>>(&self, path: P) -> Result<Vec<DirEntry>> {
        match self {
            DataSource::Dir(ds) => Ok(ds.list_dir(path)?),
            DataSource::Zip(ds) => Ok(ds.list_dir(path)?),
        }
    }

    /// Returns information about the file or directory specified by `path`.
    pub fn read_info<P: AsRef<Path>>(&self, path: P) -> Result<FileInfo> {
        match self {
            DataSource::Dir(ds) => Ok(ds.read_info(path)?),
            DataSource::Zip(ds) => Ok(ds.read_info(path)?),
        }
    }

    /// Returns whether `path` points to a file.
    pub fn is_file<P: AsRef<Path>>(&self, path: P) -> bool {
        self.read_info(path).map(|i| i.is_file).unwrap_or(false)
    }

    /// Returns whether `path` points to a directory.
    pub fn is_dir<P: AsRef<Path>>(&self, path: P) -> bool {
        self.read_info(path).map(|i| i.is_dir).unwrap_or(false)
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid path: {0}")]
    InvalidPath(PathBuf),
    #[error("root directory not found: {0}")]
    RootDirNotFound(io::Error),
    #[error("file or directory not found")]
    NotFound,
    #[error("permission denied")]
    PermissionDenied,
    #[error("the specified file is read only: {0}")]
    ReadOnly(PathBuf),
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("archive error: {0}")]
    Zip(#[from] ::zip::result::ZipError),
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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct OpenOptions {
    /// Whether to open the file with reading enabled
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
        OpenOptions {
            read: false,
            write: false,
            create: false,
            append: false,
        }
    }

    pub fn reading() -> OpenOptions {
        OpenOptions {
            read: true,
            write: false,
            create: false,
            append: false,
        }
    }

    pub fn writing(create: bool) -> OpenOptions {
        OpenOptions {
            read: false,
            write: true,
            create,
            append: false,
        }
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
    path: PathBuf,
    info: FileInfo,
}

impl DirEntry {
    /// Returns full path to this directory entry inside the `DataSource`.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Returns the metadata of this directory entry.
    pub fn info(&self) -> FileInfo {
        self.info
    }

    /// Gets the file name from this directory entry. Since the path comes from
    /// a directory entry, path.file_name() will never return `None`.
    pub fn file_name(&self) -> &OsStr {
        self.path.file_name().unwrap()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FileInfo {
    is_file: bool,
    is_dir: bool,
    is_symlink: bool,
    read_only: bool,
}

impl FileInfo {
    /// Returns whether the directory entry represented by this `FileInfo` is a
    /// file.
    pub fn is_file(&self) -> bool {
        self.is_file
    }

    /// Returns whether the directory entry represented by this `FileInfo` is a
    /// directory.
    pub fn is_dir(&self) -> bool {
        self.is_dir
    }

    /// Returns whether the directory entry represented by this `FileInfo` is a
    /// symlink.
    pub fn is_symlink(&self) -> bool {
        self.is_symlink
    }

    /// Returns whether the directory entry represented by this `FileInfo` can
    /// not be modified.
    pub fn read_only(&self) -> bool {
        self.read_only
    }
}
