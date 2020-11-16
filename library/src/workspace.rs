use std::cell::RefCell;
use std::io;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::rc::Rc;

use byteorder::{BE, LE, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::datasource::{DataSource, dir, zip};
use crate::fstree::FsTreeEntry;

const MAGIC: u32 = 0x90A7C0DE;
const VERSION: u16 = 0;

#[derive(Debug)]
pub struct Workspace {
    roots: Vec<Rc<RefCell<WorkspaceRoot>>>,
}

#[derive(Debug)]
pub struct WorkspaceRoot {
    name: String,
    ds: Rc<DataSource>,
    root: Rc<RefCell<FsTreeEntry>>,
}

#[derive(Serialize, Deserialize)]
struct Root {
    is_zip: bool,
    path: PathBuf,
}

impl Workspace {
    pub fn new() -> Self {
        Workspace {
            roots: vec![]
        }
    }

    pub fn read_from<R: Read>(mut pipe: R) -> Result<Self> {
        let magic = pipe.read_u32::<BE>()?;
        if magic != MAGIC {
            return Err(Error::MagicError(magic));
        }

        let version = pipe.read_u16::<LE>()?;
        if version > VERSION {
            return Err(Error::FileVersionError(version));
        }

        let roots: Vec<Root> = bincode::deserialize_from(&mut pipe)?;

        let mut ws = Workspace::new();

        for root in roots {
            if root.is_zip {
                if let Err(e) = ws.add_zip(root.path) {
                    eprintln!("{:?}", e);
                }
            } else {
                if let Err(e) = ws.add_dir(root.path) {
                    eprintln!("{:?}", e);
                }
            }
        }

        Ok(ws)
    }

    pub fn write_into<W: Write>(&self, mut pipe: W) -> Result<()> {
        pipe.write_u32::<BE>(MAGIC)?;
        pipe.write_u16::<LE>(VERSION)?;

        let roots: Vec<_> = self.roots.iter()
            .map(|r| match &*r.borrow().ds {
                DataSource::Dir(ds) => Root { is_zip: false, path: ds.root().to_path_buf() },
                DataSource::Zip(ds) => Root { is_zip: true, path: ds.zip_path().to_path_buf() },
            })
            .collect();

        bincode::serialize_into(&mut pipe, &roots)?;

        Ok(())
    }

    pub fn add_dir<P: Into<PathBuf>>(&mut self, path: P) -> io::Result<()> {
        let path = path.into();
        let name = path.file_name().unwrap().to_string_lossy().to_string(); // TODO give these a better default name
        let ds = DataSource::Dir(dir::DataSource::new(path)?);
        let root = WorkspaceRoot::new(name, ds);
        self.roots.push(root);
        Ok(())
    }

    pub fn add_zip<P: Into<PathBuf>>(&mut self, path: P) -> zip::Result<()> {
        let path = path.into();
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        let ds = DataSource::Zip(zip::DataSource::new(path)?);
        let root = WorkspaceRoot::new(name, ds);
        self.roots.push(root);
        Ok(())
    }

    pub fn roots(&self) -> &[Rc<RefCell<WorkspaceRoot>>] { &self.roots }
}

impl WorkspaceRoot {
    pub fn new<S: Into<String>>(name: S, ds: DataSource) -> Rc<RefCell<Self>> {
        let wsr = Rc::new(RefCell::new(WorkspaceRoot {
            name: name.into(),
            ds: Rc::new(ds),
            root: Rc::new(RefCell::new(FsTreeEntry::new_top_level())),
        }));

        let copy = wsr.clone();
        wsr.borrow_mut().root.borrow_mut().root = Rc::downgrade(&copy);
        let fstree = wsr.borrow().root.clone();
        FsTreeEntry::refresh(&fstree);
        wsr
    }

    pub fn name(&self) -> &str { &self.name }

    pub fn root(&self) -> &Rc<RefCell<FsTreeEntry>> { &self.root }

    pub fn ds(&self) -> &Rc<DataSource> { &self.ds }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid magic")]
    MagicError(u32),
    #[error("unimplemented file version")]
    FileVersionError(u16),
    #[error("I/O error")]
    Io(#[from] io::Error),
    #[error("serialization error")]
    Serde(#[from] bincode::Error),
}