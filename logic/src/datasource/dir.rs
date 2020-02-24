use std::{fs, io};
use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};

use crate::datasource::normalize_path;

pub struct DirDataSource {
    dir: PathBuf,
}

impl DirDataSource {
    pub fn new(dir: impl AsRef<Path>) -> Result<Self, io::Error> {
        match fs::read_dir(&dir) {
            Err(e) => Err(e),
            Ok(_) => Ok(DirDataSource {
                dir: dir.as_ref().to_path_buf(),
            })
        }
    }

    pub fn open(&self, path: impl AsRef<Path>, opts: OpenOptions) -> Result<File, Error> {
        let path = self.dir.join(normalize_path(&path).ok_or_else(|| Error::InvalidPath(path.as_ref().to_path_buf()))?);
        Ok(opts.open(path)?)
    }
}

pub enum Error {
    RootDirNotFound(io::Error),
    InvalidPath(PathBuf),
    IoError(io::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IoError(err)
    }
}