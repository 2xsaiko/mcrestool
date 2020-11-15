use std::borrow::Cow;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::path::PathBuf;
use std::rc::{Rc, Weak};

use crate::{FileType, get_file_type};
use crate::workspace::WorkspaceRoot;

#[derive(Debug)]
pub struct FsTreeEntry {
    path: PathBuf,
    file_type: Option<FileType>,
    children: Vec<Rc<RefCell<FsTreeEntry>>>,
    parent: Option<Weak<RefCell<FsTreeEntry>>>,
    pub(crate) root: Weak<RefCell<WorkspaceRoot>>,
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

    fn new<P: Into<PathBuf>>(path: P, parent: Rc<RefCell<FsTreeEntry>>, root: Weak<RefCell<WorkspaceRoot>>) -> Self {
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
                Some(r) => r.borrow().name().to_string().into()
            }
        } else {
            self.path.file_name().unwrap().to_string_lossy()
        }
    }

    pub fn file_type(&self) -> Option<FileType> { self.file_type }

    pub fn is_root(&self) -> bool { self.is_top_level }

    pub fn parent(&self) -> &Option<Weak<RefCell<FsTreeEntry>>> { &self.parent }

    pub fn root(&self) -> &Weak<RefCell<WorkspaceRoot>> { &self.root }

    pub fn path(&self) -> &PathBuf { &self.path }

    pub fn children(&self) -> &[Rc<RefCell<FsTreeEntry>>] { &self.children }

    pub fn index_of(&self, child: &Rc<RefCell<FsTreeEntry>>) -> Option<usize> {
        self.children.iter().enumerate()
            .find(|(_, a)| a.as_ptr() == child.as_ptr())
            .map(|(idx, _)| idx)
    }

    pub fn refresh(entry: &Rc<RefCell<Self>>) {
        let mut e = (*entry).borrow_mut();

        let root = match e.root.upgrade() {
            None => {
                println!("fstree entry's root is gone!?");
                return;
            }
            Some(r) => r
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
                        match ch.path.cmp(&dir_entry.path) {
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
                        let path = dir_entry.path;
                        let root = e.root.clone();
                        e.children.insert(i, Rc::new(RefCell::new(FsTreeEntry::new(path, entry.clone(), root))));
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