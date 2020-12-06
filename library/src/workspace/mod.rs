use std::cell::{Ref, RefCell, RefMut};

use std::path::PathBuf;
use std::rc::{Rc, Weak};

use thiserror::Error;

use binserde::{BinDeserialize, BinSerialize};
use fstree::FsTree;
pub use fstree::{DataSourceProto, FsTreeEntry, FsTreeRoot};


use crate::ffi;
#[cfg(feature = "cpp")]
use crate::ffi::TreeChangeSubscriber as CppTreeChangeSubscriber;
use crate::gamedata::GameData;

pub use self::serde::Result;

mod fstree;
pub mod serde;

#[derive(BinDeserialize, BinSerialize)]
pub struct Workspace {
    #[binserde(no_dedup)]
    fst: FsTree,
    gd: GameData,
}

impl Workspace {
    pub fn new() -> Self {
        Workspace {
            fst: FsTree::new(),
            gd: GameData::new(),
        }
    }

    pub fn add_dir<P: Into<PathBuf>>(&mut self, path: P) -> matryoshka::Result<()> {
        self.fst.add_dir(path)?;
        self.update_refs();
        Ok(())
    }

    pub fn add_zip<P: Into<PathBuf>>(&mut self, path: P) -> matryoshka::Result<()> {
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

    pub fn detach(&mut self, root: &Rc<RefCell<FsTreeRoot>>) {
        self.fst.detach(root);
        self.update_refs();
    }

    pub fn roots(&self) -> &[Rc<RefCell<FsTreeRoot>>] {
        self.fst.roots()
    }

    pub fn fs_tree(&self) -> &FsTree {
        &self.fst
    }

    pub fn game_data(&self) -> &GameData {
        &self.gd
    }

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
}

#[derive(Default)]
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
        let idx = self
            .subscribers
            .iter()
            .position(|el| el.ptr_eq(&Rc::downgrade(ptr)));

        if let Some(idx) = idx {
            self.subscribers.remove(idx);
        }
    }

    #[cfg(feature = "cpp")]
    pub fn cpp_subscribe(&mut self, ptr: *mut CppTreeChangeSubscriber) {
        if ptr.is_null() {
            return;
        }

        self.cpp_subscribers.push(ptr);
    }

    #[cfg(feature = "cpp")]
    pub fn cpp_unsubscribe(&mut self, ptr: *mut CppTreeChangeSubscriber) {
        if ptr.is_null() {
            return;
        }

        let idx = self.cpp_subscribers.iter().position(|&el| el == ptr);

        if let Some(idx) = idx {
            self.cpp_subscribers.remove(idx);
        }
    }

    fn pre_insert(&self, path: &[usize], start: usize, end: usize) {
        self.subscribers
            .iter()
            .filter_map(Weak::upgrade)
            .for_each(|l| l.pre_insert(path, start, end));

        #[cfg(feature = "cpp")]
        {
            let path = path.to_vec();
            self.cpp_subscribers
                .iter()
                .for_each(|&l| ffi::tcs_pre_insert(l, &path, start, end));
        }
    }

    fn post_insert(&self, path: &[usize]) {
        self.subscribers
            .iter()
            .filter_map(Weak::upgrade)
            .for_each(|l| l.post_insert(path));

        #[cfg(feature = "cpp")]
        {
            let path = path.to_vec();
            self.cpp_subscribers
                .iter()
                .for_each(|&l| ffi::tcs_post_insert(l, &path));
        }
    }

    fn pre_remove(&self, path: &[usize], start: usize, end: usize) {
        self.subscribers
            .iter()
            .filter_map(Weak::upgrade)
            .for_each(|l| l.pre_remove(path, start, end));

        #[cfg(feature = "cpp")]
        {
            let path = path.to_vec();
            self.cpp_subscribers
                .iter()
                .for_each(|&l| ffi::tcs_pre_remove(l, &path, start, end));
        }
    }

    fn post_remove(&self, path: &[usize]) {
        self.subscribers
            .iter()
            .filter_map(Weak::upgrade)
            .for_each(|l| l.post_remove(path));

        #[cfg(feature = "cpp")]
        {
            let path = path.to_vec();
            self.cpp_subscribers
                .iter()
                .for_each(|&l| ffi::tcs_post_remove(l, &path));
        }
    }
}

pub trait TreeChangeSubscriber {
    fn pre_insert(&self, path: &[usize], start: usize, end: usize);

    fn post_insert(&self, path: &[usize]);

    fn pre_remove(&self, path: &[usize], start: usize, end: usize);

    fn post_remove(&self, path: &[usize]);
}
