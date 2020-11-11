use std::cell::RefCell;
use std::ffi::OsStr;
use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read};
use std::path::{Component, Path, PathBuf};

use thiserror::Error;
use zip::ZipArchive;

use crate::datasource::{DirEntry, FileInfo, normalize_path};

pub struct DataSource {
    archive: RefCell<ZipArchive<File>>,
    tree: RefCell<Option<DirTree>>,
}

impl DataSource {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let file = File::open(path)?;
        let za = ZipArchive::new(file)?;
        Ok(DataSource {
            archive: RefCell::new(za),
            tree: Default::default(),
        })
    }

    pub fn open<P: AsRef<Path>>(&self, path: P) -> Result<Vec<u8>, Error> {
        let path = resolve_path_for_archive(&path)?;
        let mut archive = self.archive.borrow_mut();
        let mut file = archive.by_name(&path)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        Ok(buf)
    }

    pub fn list_dir<P: AsRef<Path>>(&self, path: P) -> Result<Vec<DirEntry>, Error> {
        self.init_tree();
        let tree = self.tree.borrow();
        let tree = tree.as_ref().unwrap();

        match tree.navigate(resolve_path_for_archive(path)?) {
            None => Err(Error::Io(io::Error::new(io::ErrorKind::NotFound, "directory not found"))),
            Some(t) => {
                let mut vec = Vec::new();

                for x in t.children.iter() {
                    vec.push(DirEntry {
                        file_name: x.name.to_string().into(),
                        info: FileInfo {
                            is_file: false,
                            is_dir: true,
                            is_symlink: false,
                            read_only: true,
                        },
                    });
                }

                for x in t.files.iter() {
                    vec.push(DirEntry {
                        file_name: x.into(),
                        info: FileInfo {
                            is_file: true,
                            is_dir: false,
                            is_symlink: false,
                            read_only: true,
                        },
                    });
                }

                Ok(vec)
            }
        }
    }

    pub fn read_info<P: AsRef<Path>>(&self, path: P) -> Result<FileInfo, Error> {
        self.init_tree();
        let tree = self.tree.borrow();
        let tree = tree.as_ref().unwrap();
        let path = path.as_ref();

        if Path::new("/").join(path) == Path::new("/") {
            Ok(FileInfo {
                is_file: false,
                is_dir: true,
                is_symlink: false,
                read_only: true,
            })
        } else {
            match path.parent() {
                None => Err(Error::InvalidPath(path.to_path_buf())),
                Some(parent) => {
                    let file_name = path.file_name().ok_or_else(|| Error::InvalidPath(path.to_path_buf()))?;
                    let parent = resolve_path_for_archive(parent)?;
                    let cd = tree.navigate(parent)
                        .ok_or_else(|| Error::Io(io::Error::new(io::ErrorKind::NotFound, "directory not found")))?;

                    if cd.children.binary_search_by(|a| (OsStr::new(&a.name)).cmp(&file_name)).is_ok() {
                        Ok(FileInfo { is_file: false, is_dir: true, is_symlink: false, read_only: true })
                    } else if cd.files.binary_search_by(|a| (OsStr::new(&a)).cmp(&file_name)).is_ok() {
                        Ok(FileInfo { is_file: true, is_dir: false, is_symlink: false, read_only: true })
                    } else {
                        Err(Error::Io(io::Error::new(ErrorKind::NotFound, "file or directory not found")))
                    }
                }
            }
        }
    }

    fn init_tree(&self) {
        if self.tree.borrow().is_none() {
            let mut archive = self.archive.borrow();
            let mut tree = DirTree::new("/");

            for path in archive.file_names() {
                let path = Path::new(path);

                let mut components = path.components();

                let file_name = components.next_back().and_then(|p| match p {
                    Component::Normal(p) => Some(p),
                    _ => None,
                }).expect("malformed ZIP archive");

                let dir = components.fold(&mut tree, |acc, a| acc.subdir_or_create(a.as_os_str().to_str().unwrap()));
                dir.append(file_name.to_str().unwrap());
            }

            *self.tree.borrow_mut() = Some(tree);
        }
    }
}

fn resolve_path_for_archive<P: AsRef<Path>>(path: P) -> Result<String, Error> {
    let path = path.as_ref();
    let pb = normalize_path(path).ok_or_else(|| Error::InvalidPath(path.to_path_buf()))?;
    Ok(pb.strip_prefix("/").unwrap().to_str().unwrap().to_string())
}

struct DirTree {
    name: String,
    children: Vec<DirTree>,
    files: Vec<String>,
}

impl DirTree {
    fn new<S: Into<String>>(name: S) -> DirTree {
        DirTree {
            name: name.into(),
            children: vec![],
            files: vec![],
        }
    }

    fn append(&mut self, file: &str) {
        if self.children.binary_search_by(|a| (*a.name).cmp(file)).is_ok() {
            return;
        }

        if let Err(idx) = self.files.binary_search_by(|a| (&**a).cmp(file)) {
            self.files.insert(idx, file.to_string());
        }
    }

    fn subdir(&self, dir: &str) -> Option<&DirTree> {
        self.children.binary_search_by(|a| (*a.name).cmp(dir)).ok().map(|idx| &self.children[idx])
    }

    fn subdir_or_create(&mut self, dir: &str) -> &mut DirTree {
        if let Ok(idx) = self.files.binary_search_by(|a| (&**a).cmp(dir)) {
            self.files.remove(idx);
        }

        match self.children.binary_search_by(|a| (*a.name).cmp(dir)) {
            Ok(idx) => {
                &mut self.children[idx]
            }
            Err(idx) => {
                let dt = DirTree::new(dir);
                self.children.insert(idx, dt);
                &mut self.children[idx]
            }
        }
    }

    fn navigate<P: AsRef<Path>>(&self, path: P) -> Option<&DirTree> {
        let path = path.as_ref();

        path.components()
            .skip_while(|r| match r {
                Component::Prefix(_) => panic!("invalid path prefix in ZIP path"),
                Component::RootDir => true,
                _ => false,
            })
            .fold(Some(self), |acc, a| acc.and_then(|acc| acc.subdir(a.as_os_str().to_str().unwrap())))
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