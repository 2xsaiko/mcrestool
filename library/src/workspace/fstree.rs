use std::borrow::Cow;
use std::cell::{Ref, RefCell, RefMut};
use std::cmp::Ordering;
use std::path::{Path, PathBuf};
use std::rc::{Rc, Weak};

use binserde::try_iter::try_iter;
use binserde::util::{serialize_iter, VecLikeIter};
use binserde::{BinDeserialize, BinDeserializer, BinSerialize, BinSerializer};
use matryoshka::DataSource;

use crate::workspace::TreeChangeDispatcher;
use crate::{get_file_type, FileType};

pub struct FsTree {
    roots: Vec<Rc<RefCell<FsTreeRoot>>>,
    dispatcher: Rc<RefCell<TreeChangeDispatcher>>,
}

impl FsTree {
    pub fn new() -> Self {
        FsTree {
            roots: vec![],
            dispatcher: Rc::new(RefCell::new(TreeChangeDispatcher::new())),
        }
    }

    pub fn add_dir<P>(&mut self, path: P) -> matryoshka::Result<()>
    where
        P: Into<PathBuf>,
    {
        let path = path.into();
        let name = path.file_name().unwrap().to_string_lossy().to_string(); // TODO give these a better default name
        self.add_dir_with_name(path, name)
    }

    pub fn add_dir_with_name<P, S>(&mut self, path: P, name: S) -> matryoshka::Result<()>
    where
        P: Into<PathBuf>,
        S: Into<String>,
    {
        let root = FsTreeRoot::new(name, DataSourceProto::Dir(path.into()));
        self.dispatcher()
            .pre_insert(&[], self.roots.len(), self.roots.len());
        self.roots.push(root.clone());
        self.dispatcher().post_insert(&[]);

        self.open(&root)?;

        Ok(())
    }

    pub fn add_zip<P>(&mut self, path: P) -> matryoshka::Result<()>
    where
        P: Into<PathBuf>,
    {
        let path = path.into();
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        self.add_zip_with_name(path, name)
    }

    pub fn add_zip_with_name<P, S>(&mut self, path: P, name: S) -> matryoshka::Result<()>
    where
        P: Into<PathBuf>,
        S: Into<String>,
    {
        let root = FsTreeRoot::new(name, DataSourceProto::Zip(path.into()));
        self.dispatcher()
            .pre_insert(&[], self.roots.len(), self.roots.len());
        self.roots.push(root.clone());
        self.dispatcher().post_insert(&[]);

        self.open(&root)?;

        Ok(())
    }

    pub fn detach(&mut self, root: &Rc<RefCell<FsTreeRoot>>) {
        if let Some(idx) = self.roots.iter().position(|r| r.as_ptr() == root.as_ptr()) {
            self.dispatcher().pre_remove(&[], idx, idx);
            self.roots.remove(idx);
            self.dispatcher().post_remove(&[]);
        }
    }

    pub fn open(&self, root: &Rc<RefCell<FsTreeRoot>>) -> matryoshka::Result<()> {
        if self
            .roots
            .iter()
            .find(|r| r.as_ptr() == root.as_ptr())
            .is_some()
        {
            let mut ref_mut = root.borrow_mut();
            ref_mut.open()?;
            ref_mut.root().borrow_mut().root = Rc::downgrade(&root);
            let root = ref_mut.root().clone();
            drop(ref_mut);
            self.refresh(&root);
        }

        Ok(())
    }

    pub fn close(&self, root: &Rc<RefCell<FsTreeRoot>>) {
        match self
            .roots
            .iter()
            .position(|el| el.as_ptr() == root.as_ptr())
        {
            None => {}
            Some(s) => {
                let d = self.dispatcher.borrow();
                let mut r = root.borrow_mut();
                let len = r.root().borrow().children().len();

                drop(r);
                d.pre_remove(&[s], 0, len - 1);
                r = root.borrow_mut();

                r.close();

                drop(r);
                d.post_remove(&[s]);
            }
        }
    }

    pub fn roots(&self) -> &[Rc<RefCell<FsTreeRoot>>] {
        &self.roots
    }

    pub fn reset(&mut self) {
        self.dispatcher().pre_remove(&[], 0, self.roots.len() - 1);
        self.roots.clear();
        self.dispatcher().post_remove(&[]);
    }

    pub fn dispatcher(&self) -> Ref<TreeChangeDispatcher> {
        self.dispatcher.borrow()
    }

    pub fn dispatcher_mut(&self) -> RefMut<TreeChangeDispatcher> {
        self.dispatcher.borrow_mut()
    }

    pub fn find_path(&self, entry: &Rc<RefCell<FsTreeEntry>>) -> Option<Vec<usize>> {
        let mut entry = entry.clone();
        loop {
            let mut vec = Vec::new();
            let e = entry.borrow();
            match e.parent() {
                None => {
                    let root = e.root().upgrade()?;
                    let idx = self
                        .roots
                        .iter()
                        .position(|el| el.as_ptr() == root.as_ptr())?;
                    vec.push(idx);
                    vec.reverse();
                    break Some(vec);
                }
                Some(parent) => {
                    let parent = parent.upgrade()?;
                    let p = parent.borrow();
                    let idx = p.index_of(&entry)?;
                    vec.push(idx);
                    drop(p);
                    drop(e);
                    entry = parent;
                }
            }
        }
    }

    pub fn refresh(&self, entry: &Rc<RefCell<FsTreeEntry>>) {
        let d = self.dispatcher.borrow();
        if let Some(mut vec) = self.find_path(entry) {
            let root = match entry.borrow().root.upgrade() {
                None => {
                    // there has to be a root since a Rc to it is stored in self
                    // otherwise, it wouldn't have belonged to this workspace
                    // anyway
                    unreachable!();
                }
                Some(root) => root,
            };

            let root = root.borrow();

            if let Some(rd) = root.data() {
                self.refresh0(entry, &d, &rd.ds, &mut vec);
            }
        }
    }

    fn refresh0(
        &self,
        entry: &Rc<RefCell<FsTreeEntry>>,
        d: &TreeChangeDispatcher,
        ds: &DataSource,
        path_buf: &mut Vec<usize>,
    ) {
        let mut e = entry.borrow_mut();

        {
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

                                drop(e);
                                d.pre_remove(&path_buf, i, i);
                                e = entry.borrow_mut();

                                e.children.remove(i);

                                drop(e);
                                d.post_remove(&path_buf);
                                e = entry.borrow_mut();
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

                        drop(e);
                        d.pre_insert(&path_buf, i, i);
                        e = entry.borrow_mut();

                        e.children.insert(
                            i,
                            Rc::new(RefCell::new(FsTreeEntry::new(path, entry.clone(), root))),
                        );

                        drop(e);
                        d.post_insert(&path_buf);
                        e = entry.borrow_mut();
                    }
                }
            } else {
                let len = e.children.len();

                if len > 0 {
                    drop(e);
                    d.pre_remove(&path_buf, 0, len - 1);
                    e = entry.borrow_mut();

                    e.children.clear();

                    drop(e);
                    d.post_remove(&path_buf);
                    e = entry.borrow_mut();
                }
            }
        }

        let v = e.children.clone();
        drop(e);

        for (idx, ch) in v.into_iter().enumerate() {
            path_buf.push(idx);
            self.refresh0(&ch, d, ds, path_buf);
            path_buf.pop();
        }
    }
}

impl<'de> BinDeserialize<'de> for FsTree {
    fn deserialize<D: BinDeserializer<'de>>(deserializer: D) -> binserde::Result<Self> {
        let mut tree = FsTree::new();
        tree.deserialize_in_place(deserializer)?;
        Ok(tree)
    }

    fn deserialize_in_place<D: BinDeserializer<'de>>(
        &mut self,
        deserializer: D,
    ) -> binserde::Result<()> {
        self.reset();

        self.roots = try_iter(VecLikeIter::new(deserializer)?, |iter| {
            iter.map(|el| Rc::new(el)).collect()
        })?;

        Ok(())
    }
}

impl BinSerialize for FsTree {
    fn serialize<S: BinSerializer>(&self, serializer: S) -> binserde::Result<()> {
        serialize_iter(self.roots.iter().map(|el| &**el), serializer)?;
        Ok(())
    }
}

#[derive(Debug, BinSerialize)]
pub struct FsTreeRoot {
    name: String,
    ds_proto: DataSourceProto,
    #[binserde(skip)]
    data: Option<OpenFsTreeRoot>,
    #[binserde(skip)]
    root: Rc<RefCell<FsTreeEntry>>,
}

impl<'de> BinDeserialize<'de> for FsTreeRoot {
    fn deserialize<D: BinDeserializer<'de>>(mut deserializer: D) -> binserde::Result<Self> {
        Ok(FsTreeRoot {
            name: String::deserialize(&mut deserializer)?,
            ds_proto: DataSourceProto::deserialize(&mut deserializer)?,
            data: None,
            root: Rc::new(RefCell::new(Default::default())),
        })
    }
}

#[derive(Debug, BinSerialize, BinDeserialize)]
pub enum DataSourceProto {
    Dir(PathBuf),
    Zip(PathBuf),
}

impl DataSourceProto {
    fn open(&self) -> matryoshka::Result<DataSource> {
        match self {
            DataSourceProto::Dir(path) => Ok(DataSource::new_dir(path)?),
            DataSourceProto::Zip(path) => Ok(DataSource::new_zip(path)?),
        }
    }
}

#[derive(Debug)]
pub struct OpenFsTreeRoot {
    ds: Rc<DataSource>,
}

impl FsTreeRoot {
    fn new<S: Into<String>>(name: S, ds_proto: DataSourceProto) -> Rc<RefCell<Self>> {
        let fst = Rc::new(RefCell::new(FsTreeRoot {
            name: name.into(),
            ds_proto,
            data: None,
            root: Rc::new(RefCell::new(FsTreeEntry::new_top_level())),
        }));

        fst.borrow_mut().root().borrow_mut().root = Rc::downgrade(&fst);

        fst
    }

    fn close(&mut self) {
        if self.data.is_some() {
            self.data = None;
            self.root.borrow_mut().children.clear();
        }
    }

    fn open(&mut self) -> matryoshka::Result<&mut OpenFsTreeRoot> {
        match self.data {
            None => {
                let source = self.ds_proto.open()?;
                self.data = Some(OpenFsTreeRoot {
                    ds: Rc::new(source),
                });
                Ok(self.data.as_mut().unwrap())
            }
            Some(ref mut el) => Ok(el),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn proto(&self) -> &DataSourceProto {
        &self.ds_proto
    }

    pub fn data(&self) -> Option<&OpenFsTreeRoot> {
        self.data.as_ref()
    }

    pub fn data_mut(&mut self) -> Option<&mut OpenFsTreeRoot> {
        self.data.as_mut()
    }

    pub fn root(&self) -> &Rc<RefCell<FsTreeEntry>> {
        &self.root
    }
}

impl OpenFsTreeRoot {
    pub fn ds(&self) -> &Rc<DataSource> {
        &self.ds
    }
}

#[derive(Debug)]
pub struct FsTreeEntry {
    path: PathBuf,
    file_type: Option<FileType>,
    children: Vec<Rc<RefCell<FsTreeEntry>>>,
    parent: Option<Weak<RefCell<FsTreeEntry>>>,
    root: Weak<RefCell<FsTreeRoot>>,
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
        root: Weak<RefCell<FsTreeRoot>>,
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

    pub fn root(&self) -> &Weak<RefCell<FsTreeRoot>> {
        &self.root
    }

    pub fn path(&self) -> &Path {
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
}

impl Default for FsTreeEntry {
    fn default() -> Self {
        FsTreeEntry::new_top_level()
    }
}
