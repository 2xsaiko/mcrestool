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
        Ok(opts.open(self.get_full_path(path)?)?)
    }

    pub fn list_dir(&self, path: impl AsRef<Path>) -> Result<Vec<PathBuf>, Error> {
        let result = fs::read_dir(self.get_full_path(path)?)?;
        Ok(result
            .filter_map(|e| e.ok())
            .map(|e| e.path().strip_prefix(&self.dir).unwrap().to_path_buf())
            .collect())
    }

    pub fn create_dir(&self, path: impl AsRef<Path>) -> Result<(), Error> {
        Ok(fs::create_dir(self.get_full_path(path)?)?)
    }

    pub fn create_dir_all(&self, path: impl AsRef<Path>) -> Result<(), Error> {
        Ok(fs::create_dir_all(self.get_full_path(path)?)?)
    }

    pub fn delete_file(&self, path: impl AsRef<Path>) -> Result<(), Error> {
        Ok(fs::remove_file(self.get_full_path(path)?)?)
    }

    pub fn delete_dir(&self, path: impl AsRef<Path>) -> Result<(), Error> {
        Ok(fs::remove_dir(self.get_full_path(path)?)?)
    }

    pub fn delete_dir_all(&self, path: impl AsRef<Path>) -> Result<(), Error> {
        Ok(fs::remove_dir_all(self.get_full_path(path)?)?)
    }

    fn get_full_path(&self, path: impl AsRef<Path>) -> Result<PathBuf, Error> {
        let buf = self.dir.join(normalize_path(&path).ok_or_else(|| Error::InvalidPath(path.as_ref().to_path_buf()))?.strip_prefix("/").unwrap());
        println!("{}", buf.to_str().unwrap());
        Ok(buf)
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