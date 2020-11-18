use std::cell::{Ref, RefCell, RefMut};
use std::io;
use std::io::{Read, Write};
#[cfg(not(feature = "cpp"))]
use std::marker::PhantomData;
use std::path::PathBuf;
use std::rc::{Rc, Weak};

use byteorder::{BE, LE, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub use fstree::FsTreeEntry;
use matryoshka::{DataSource, dir, zip};

use crate::ffi;
#[cfg(feature = "cpp")]
use crate::ffi::TreeChangeSubscriber as CppTreeChangeSubscriber;
use crate::gamedata::GameData;
use crate::workspace::fstree::FsTree;

mod fstree;

pub const MAGIC: u16 = 0x3B1C;
pub const VERSION: u16 = 0;

pub struct Workspace {
    fst: FsTree,
    gd: GameData,
}

#[derive(Serialize, Deserialize)]
struct Root {
    is_zip: bool,
    path: PathBuf,
}

impl Workspace {
    pub fn new() -> Self {
        Workspace {
            fst: FsTree::new(),
            gd: GameData::new(),
        }
    }

    pub fn add_dir<P: Into<PathBuf>>(&mut self, path: P) -> io::Result<()> {
        self.fst.add_dir(path)?;
        self.update_refs();
        Ok(())
    }

    pub fn add_zip<P: Into<PathBuf>>(&mut self, path: P) -> zip::Result<()> {
        self.fst.add_zip(path)?;
        self.update_refs();
        Ok(())
    }

    pub fn update_refs(&mut self) {
        self.gd.collect_usages(self.fst.roots());
        self.gd.create_dummies();

        let mut blocks: Vec<_> = self.gd.blocks().keys().collect();
        blocks.sort();
        print!("Blocks: ");
        blocks.iter().for_each(|id| print!("{} ", id));
        println!();

        let mut items: Vec<_> = self.gd.items().keys().collect();
        items.sort();
        print!("Items: ");
        items.iter().for_each(|id| print!("{} ", id));
        println!();
    }

    pub fn detach(&mut self, root: &Rc<RefCell<WorkspaceRoot>>) {
        self.fst.detach(root);
        self.update_refs();
    }

    pub fn roots(&self) -> &[Rc<RefCell<WorkspaceRoot>>] { self.fst.roots() }

    pub fn fs_tree(&self) -> &FsTree { &self.fst }

    pub fn game_data(&self) -> &GameData { &self.gd }

    pub fn fst_dispatcher(&self) -> Ref<TreeChangeDispatcher> {
        self.fst.dispatcher()
    }

    pub fn fst_dispatcher_mut(&self) -> RefMut<TreeChangeDispatcher> {
        self.fst.dispatcher_mut()
    }

    pub fn gd_dispatcher(&self) -> Ref<TreeChangeDispatcher> {
        self.gd.dispatcher()
    }

    pub fn gd_dispatcher_mut(&self) -> RefMut<TreeChangeDispatcher> {
        self.gd.dispatcher_mut()
    }

    pub fn reset(&mut self) {
        self.fst.reset();
        self.update_refs();
    }

    pub fn read_from<R: Read>(pipe: R) -> Result<Self> {
        let mut ws = Workspace::new();
        ws.read_from_in_place(pipe)?;
        Ok(ws)
    }

    pub fn read_from_in_place<R: Read>(&mut self, mut pipe: R) -> Result<()> {
        self.fst.reset();

        let magic = pipe.read_u16::<BE>()?;
        if magic != MAGIC {
            return Err(Error::MagicError(magic));
        }

        let version = pipe.read_u16::<LE>()?;
        if version > VERSION {
            return Err(Error::FileVersionError(version));
        }

        let roots: Vec<Root> = bincode::deserialize_from(&mut pipe)?;

        for root in roots {
            if root.is_zip {
                if let Err(e) = self.fst.add_zip(root.path) {
                    eprintln!("{:?}", e);
                }
            } else {
                if let Err(e) = self.fst.add_dir(root.path) {
                    eprintln!("{:?}", e);
                }
            }
        }

        self.update_refs();

        Ok(())
    }

    pub fn write_into<W: Write>(&self, mut pipe: W) -> Result<()> {
        pipe.write_u16::<BE>(MAGIC)?;
        pipe.write_u16::<LE>(VERSION)?;

        let roots: Vec<_> = self.roots().iter()
            .map(|r| match &*r.borrow().ds {
                DataSource::Dir(ds) => Root { is_zip: false, path: ds.root().to_path_buf() },
                DataSource::Zip(ds) => Root { is_zip: true, path: ds.zip_path().to_path_buf() },
            })
            .collect();

        bincode::serialize_into(&mut pipe, &roots)?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct WorkspaceRoot {
    name: String,
    ds: Rc<DataSource>,
    root: Rc<RefCell<FsTreeEntry>>,
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

pub struct TreeChangeDispatcher {
    subscribers: Vec<Weak<dyn TreeChangeSubscriber>>,

    #[cfg(feature = "cpp")]
    cpp_subscribers: Vec<*mut CppTreeChangeSubscriber>,
}

impl TreeChangeDispatcher {
    pub fn new() -> Self {
        TreeChangeDispatcher {
            subscribers: vec![],

            #[cfg(feature = "cpp")]
            cpp_subscribers: vec![],
        }
    }

    pub fn subscribe(&mut self, ptr: &Rc<dyn TreeChangeSubscriber>) {
        self.subscribers.push(Rc::downgrade(ptr));
    }

    pub fn unsubscribe(&mut self, ptr: &Rc<dyn TreeChangeSubscriber>) {
        let idx = self.subscribers.iter().position(|el| el.ptr_eq(&Rc::downgrade(ptr)));

        if let Some(idx) = idx {
            self.subscribers.remove(idx);
        }
    }

    #[cfg(feature = "cpp")]
    pub fn cpp_subscribe(&mut self, ptr: *mut CppTreeChangeSubscriber) {
        if ptr.is_null() { return; }

        self.cpp_subscribers.push(ptr);
    }

    #[cfg(feature = "cpp")]
    pub fn cpp_unsubscribe(&mut self, ptr: *mut CppTreeChangeSubscriber) {
        if ptr.is_null() { return; }

        let idx = self.cpp_subscribers.iter().position(|&el| el == ptr);

        if let Some(idx) = idx {
            self.cpp_subscribers.remove(idx);
        }
    }

    fn pre_insert(&self, path: &Vec<usize>, start: usize, end: usize) {
        self.subscribers.iter()
            .filter_map(Weak::upgrade)
            .for_each(|l| l.pre_insert(path, start, end));

        #[cfg(feature = "cpp")]
            self.cpp_subscribers.iter().for_each(|&l| ffi::tcs_pre_insert(l, path, start, end));
    }

    fn post_insert(&self, path: &Vec<usize>) {
        self.subscribers.iter()
            .filter_map(Weak::upgrade)
            .for_each(|l| l.post_insert(path));

        #[cfg(feature = "cpp")]
            self.cpp_subscribers.iter().for_each(|&l| ffi::tcs_post_insert(l, path));
    }

    fn pre_remove(&self, path: &Vec<usize>, start: usize, end: usize) {
        self.subscribers.iter()
            .filter_map(Weak::upgrade)
            .for_each(|l| l.pre_remove(path, start, end));

        #[cfg(feature = "cpp")]
            self.cpp_subscribers.iter().for_each(|&l| ffi::tcs_pre_remove(l, path, start, end));
    }

    fn post_remove(&self, path: &Vec<usize>) {
        self.subscribers.iter()
            .filter_map(Weak::upgrade)
            .for_each(|l| l.post_remove(path));

        #[cfg(feature = "cpp")]
            self.cpp_subscribers.iter().for_each(|&l| ffi::tcs_post_remove(l, path));
    }
}

pub trait TreeChangeSubscriber {
    fn pre_insert(&self, path: &Vec<usize>, start: usize, end: usize);

    fn post_insert(&self, path: &Vec<usize>);

    fn pre_remove(&self, path: &Vec<usize>, start: usize, end: usize);

    fn post_remove(&self, path: &Vec<usize>);
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid magic")]
    MagicError(u16),
    #[error("unimplemented file version")]
    FileVersionError(u16),
    #[error("I/O error")]
    Io(#[from] io::Error),
    #[error("serialization error")]
    Serde(#[from] bincode::Error),
}