use std::borrow::Cow;
use std::cell::{Ref, RefCell, RefMut};
use std::cmp::Ordering;
use std::io;
use std::path::PathBuf;
use std::rc::{Rc, Weak};

use matryoshka::{dir, zip, DataSource};

use crate::workspace::{TreeChangeDispatcher, WorkspaceRoot};
use crate::{get_file_type, FileType};

pub struct FsTree {
    roots: Vec<Rc<RefCell<WorkspaceRoot>>>,
    dispatcher: Rc<RefCell<TreeChangeDispatcher>>,
}

impl FsTree {
    pub fn new() -> Self {
        FsTree {
            roots: vec![],
            dispatcher: Rc::new(RefCell::new(TreeChangeDispatcher::new())),
        }
    }

    pub fn add_dir<P>(&mut self, path: P) -> io::Result<()>
    where
        P: Into<PathBuf>,
    {
        let path = path.into();
        let name = path.file_name().unwrap().to_string_lossy().to_string(); // TODO give these a better default name
        self.add_dir_with_name(path, name)
    }

    pub fn add_dir_with_name<P, S>(&mut self, path: P, name: S) -> io::Result<()>
    where
        P: Into<PathBuf>,
        S: Into<String>,
    {
        let path = path.into();
        let ds = DataSource::Dir(dir::DataSource::new(path)?);
        let root = WorkspaceRoot::new(name, ds);
        self.dispatcher()
            .pre_insert(&vec![], self.roots.len(), self.roots.len());
        self.roots.push(root);
        self.dispatcher().post_insert(&vec![]);

        Ok(())
    }

    pub fn add_zip<P>(&mut self, path: P) -> zip::Result<()>
    where
        P: Into<PathBuf>,
    {
        let path = path.into();
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        self.add_zip_with_name(path, name)
    }

    pub fn add_zip_with_name<P, S>(&mut self, path: P, name: S) -> zip::Result<()>
    where
        P: Into<PathBuf>,
        S: Into<String>,
    {
        let path = path.into();
        let ds = DataSource::Zip(zip::DataSource::new(path)?);
        let root = WorkspaceRoot::new(name, ds);
        self.dispatcher()
            .pre_insert(&vec![], self.roots.len(), self.roots.len());
        self.roots.push(root);
        self.dispatcher().post_insert(&vec![]);

        Ok(())
    }

    pub fn detach(&mut self, root: &Rc<RefCell<WorkspaceRoot>>) {
        if let Some(idx) = self.roots.iter().position(|r| r.as_ptr() == root.as_ptr()) {
            self.dispatcher().pre_remove(&vec![], idx, idx);
            self.roots.remove(idx);
            self.dispatcher().post_remove(&vec![]);
        }
    }

    pub fn roots(&self) -> &[Rc<RefCell<WorkspaceRoot>>] {
        &self.roots
    }

    pub fn reset(&mut self) {
        self.dispatcher().pre_remove(&vec![], 0, self.roots.len());
        self.roots.clear();
        self.dispatcher().post_remove(&vec![]);
    }

    pub fn dispatcher(&self) -> Ref<TreeChangeDispatcher> {
        self.dispatcher.borrow()
    }

    pub fn dispatcher_mut(&self) -> RefMut<TreeChangeDispatcher> {
        self.dispatcher.borrow_mut()
    }
}

#[derive(Debug)]
pub struct FsTreeEntry {
    path: PathBuf,
    file_type: Option<FileType>,
    children: Vec<Rc<RefCell<FsTreeEntry>>>,
    parent: Option<Weak<RefCell<FsTreeEntry>>>,
    pub(super) root: Weak<RefCell<WorkspaceRoot>>,
    is_top_level: bool,
}

impl FsTreeEntry {
    pub(crate) fn new_top_level() -> Self {
        FsTreeEntry {
            path: "/".into(),
            file_type: None,
            children: Default::default(),
            parent: None,
            root: Weak::new(),
            is_top_level: true,
        }
    }

    fn new<P: Into<PathBuf>>(
        path: P,
        parent: Rc<RefCell<FsTreeEntry>>,
        root: Weak<RefCell<WorkspaceRoot>>,
    ) -> Self {
        FsTreeEntry {
            path: path.into(),
            file_type: None,
            children: Default::default(),
            parent: Some(Rc::downgrade(&parent)),
            root,
            is_top_level: false,
        }
    }

    pub fn display_name(&self) -> Cow<str> {
        if self.is_top_level {
            match self.root.upgrade() {
                None => {
                    println!("fstree entry's root is gone!?");
                    "???".into()
                }
                Some(r) => r.borrow().name().to_string().into(),
            }
        } else {
            self.path.file_name().unwrap().to_string_lossy()
        }
    }

    pub fn file_type(&self) -> Option<FileType> {
        self.file_type
    }

    pub fn is_root(&self) -> bool {
        self.is_top_level
    }

    pub fn parent(&self) -> &Option<Weak<RefCell<FsTreeEntry>>> {
        &self.parent
    }

    pub fn root(&self) -> &Weak<RefCell<WorkspaceRoot>> {
        &self.root
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn children(&self) -> &[Rc<RefCell<FsTreeEntry>>] {
        &self.children
    }

    pub fn index_of(&self, child: &Rc<RefCell<FsTreeEntry>>) -> Option<usize> {
        self.children
            .iter()
            .position(|a| a.as_ptr() == child.as_ptr())
    }

    pub fn refresh(entry: &Rc<RefCell<Self>>) {
        let mut e = (*entry).borrow_mut();

        let root = match e.root.upgrade() {
            None => {
                println!("fstree entry's root is gone!?");
                return;
            }
            Some(r) => r,
        };

        let mut changed = false;

        {
            let root = (*root).borrow_mut();
            let ds = root.ds();

            e.file_type = get_file_type(ds, &e.path);

            if ds.is_dir(&e.path) {
                let list = match ds.list_dir(&e.path) {
                    Ok(vec) => vec,
                    Err(e) => {
                        eprintln!("failed to list directory contents: {:?}", e);
                        Vec::new()
                    }
                };

                for (i, dir_entry) in list.into_iter().enumerate() {
                    let mut found = false;

                    while e.children.len() > i {
                        let ch = e.children[i].borrow();
                        match (*ch.path).cmp(dir_entry.path()) {
                            Ordering::Less => {
                                drop(ch);
                                e.children.remove(i);
                                changed = true;
                            }
                            Ordering::Equal => {
                                found = true;
                                break;
                            }
                            Ordering::Greater => {
                                break;
                            }
                        }
                    }

                    if !found {
                        let path = dir_entry.path();
                        let root = e.root.clone();
                        e.children.insert(
                            i,
                            Rc::new(RefCell::new(FsTreeEntry::new(path, entry.clone(), root))),
                        );
                        changed = true;
                    }
                }
            } else {
                changed = !e.children.is_empty();
                e.children.clear();
            }
        }

        if changed {
            // TODO events/signals?
            // emit children_changed();
        }

        for ch in e.children.iter() {
            FsTreeEntry::refresh(ch);
        }
    }
}
