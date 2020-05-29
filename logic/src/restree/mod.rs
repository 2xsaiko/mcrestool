use std::ffi::CString;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::path::PathBuf;
use crate::datasource::DataSource;

pub mod ffi;

pub struct FileTreeRoot<'a> {
    ds: &'a DataSource,

}

pub struct FileTree {
    name: String,
    ft_path: FileTreePath,
    path: Option<PathBuf>,
    file_type: FileType,
    children: Vec<FileTree>,
}

#[repr(u8)]
pub enum FileType {
    None,
    Language,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct FileTreePath(Vec<String>);

impl Default for FileTreePath {
    fn default() -> Self { FileTreePath(vec![]) }
}

impl Display for FileTreePath {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for x in &self.0 {
            write!(f, "/{}", x)?;
        }
        Ok(())
    }
}

impl FileTreePath {
    pub fn is_root(&self) -> bool {
        self.0.is_empty()
    }

    pub fn parent(&self) -> Option<FileTreePath> {
        if !self.is_root() {
            Some(FileTreePath(self.0[..self.0.len() - 1].to_vec()))
        } else {
            None
        }
    }

    pub fn child(&self, name: &str) -> FileTreePath {
        let mut vec = self.0.clone();
        vec.push(name.to_owned());
        FileTreePath(vec)
    }

    pub fn full_path(&self) -> String {
        format!("{}", self)
    }

    pub fn path(&self) -> Option<&str> {
        self.0.last().map(|s| &**s)
    }
}
