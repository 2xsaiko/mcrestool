use std::{fs, io};
use std::fs::{File, Metadata, OpenOptions};
use std::path::{Path, PathBuf};

use crate::datasource::{DirEntry, FileInfo, normalize_path};

#[derive(Debug)]
pub struct DataSource {
    dir: PathBuf,
}

impl DataSource {
    pub fn new<P: Into<PathBuf>>(dir: P) -> Result<Self, io::Error> {
        let dir = dir.into();
        match fs::read_dir(&dir) {
            Err(e) => Err(e),
            Ok(_) => Ok(DataSource { dir })
        }
    }

    pub fn open<P: AsRef<Path>>(&self, path: P, opts: OpenOptions) -> Result<File, Error> {
        Ok(opts.open(self.get_full_path(path)?)?)
    }

    pub fn list_dir<P: AsRef<Path>>(&self, path: P) -> Result<Vec<DirEntry>, Error> {
        let result = fs::read_dir(self.get_full_path(path)?)?;
        Ok(result
            .filter_map(|e| e.ok())
            .map(|e| {
                let meta = e.metadata().expect("failed to load file metadata");
                DirEntry {
                    path: Path::new("/").join(e.path().strip_prefix(&self.dir).unwrap()),
                    info: meta.into(),
                }
            })
            .collect())
    }

    pub fn read_info<P: AsRef<Path>>(&self, path: P) -> Result<FileInfo, Error> {
        Ok(fs::symlink_metadata(self.get_full_path(path)?)?.into())
    }

    pub fn create_dir<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        Ok(fs::create_dir(self.get_full_path(path)?)?)
    }

    pub fn create_dir_all<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        Ok(fs::create_dir_all(self.get_full_path(path)?)?)
    }

    pub fn delete_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        Ok(fs::remove_file(self.get_full_path(path)?)?)
    }

    pub fn delete_dir<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        Ok(fs::remove_dir(self.get_full_path(path)?)?)
    }

    pub fn delete_dir_all<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        Ok(fs::remove_dir_all(self.get_full_path(path)?)?)
    }

    fn get_full_path<P: AsRef<Path>>(&self, path: P) -> Result<PathBuf, Error> {
        Ok(self.dir.join(normalize_path(&path).ok_or_else(|| Error::InvalidPath(path.as_ref().to_path_buf()))?.strip_prefix("/").unwrap()))
    }
}

impl From<fs::Metadata> for FileInfo {
    fn from(meta: Metadata) -> Self {
        let ft = meta.file_type();
        FileInfo {
            is_file: ft.is_file(),
            is_dir: ft.is_dir(),
            is_symlink: ft.is_symlink(),
            read_only: meta.permissions().readonly(),
        }
    }
}

pub enum Error {
    RootDirNotFound(io::Error),
    InvalidPath(PathBuf),
    Io(io::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}