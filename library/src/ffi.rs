use std::{io, mem};
use std::cell::RefCell;
use std::fs::File;
use std::io::{Read, Write, BufReader, BufWriter};
use std::path::Path;
use std::pin::Pin;
use std::rc::Rc;

use matryoshka::{self, DataSource, dir, zip};
use matryoshka::resfile::ResFile;

use crate::{FileType, langtable, workspace};
use crate::langtable::LanguageTable;
use crate::workspace::{FsTreeEntry, Workspace, WorkspaceRoot};

macro_rules! define_wrapper {
    ($($name:ident($inner:ty);)*) => {
        $(
            pub struct $name(pub $inner);

            impl std::ops::Deref for $name {
                type Target = $inner;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl std::ops::DerefMut for $name {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }

            impl std::convert::From<$inner> for $name {
                fn from(inner: $inner) -> Self {
                    $name(inner)
                }
            }
        )*
    }
}

define_wrapper! {
    ResFilePrivate(ResFile);
    WorkspaceRootPrivate(Option<Rc<RefCell<WorkspaceRoot>>>);
    FsTreeEntryPrivate(Option<Rc<RefCell<FsTreeEntry>>>);
    DataSourcePrivate(Rc<DataSource>);
    LanguageTablePrivate(LanguageTable);
    WorkspacePrivate(Workspace);
}

pub type TreeChangeSubscriber = types::TreeChangeSubscriber;

#[cxx::bridge(namespace = "mcrtlib::ffi")]
mod types {
    pub struct Workspace {
        pub inner: Box<WorkspacePrivate>,
    }

    pub struct DataSource {
        pub inner: Box<DataSourcePrivate>,
    }

    pub struct ResFile {
        pub inner: Box<ResFilePrivate>,
    }

    pub struct WorkspaceRoot {
        pub inner: Box<WorkspaceRootPrivate>,
    }

    pub struct FsTreeEntry {
        pub inner: Box<FsTreeEntryPrivate>,
    }

    pub struct DirEntry {
        pub path: String,
        pub info: FileInfo,
    }

    pub struct FileInfo {
        pub is_file: bool,
        pub is_dir: bool,
        pub is_symlink: bool,
        pub read_only: bool,
    }

    pub struct LanguageTable {
        pub inner: Box<LanguageTablePrivate>,
    }

    pub enum FileType {
        FILETYPE_NONE,
        FILETYPE_LANGUAGE,
        FILETYPE_LANGUAGE_PART,
        FILETYPE_RECIPE,
    }

    unsafe extern "C++" {
        include!("mcrtlibd.h");

        type TreeChangeSubscriber;

        pub fn tcs_pre_insert(s: Pin<&mut TreeChangeSubscriber>, path: &Vec<usize>, start: usize, end: usize);

        pub fn tcs_post_insert(s: Pin<&mut TreeChangeSubscriber>, path: &Vec<usize>);

        pub fn tcs_pre_remove(s: Pin<&mut TreeChangeSubscriber>, path: &Vec<usize>, start: usize, end: usize);

        pub fn tcs_post_remove(s: Pin<&mut TreeChangeSubscriber>, path: &Vec<usize>);
    }

    extern "Rust" {
        type WorkspacePrivate;
        type DataSourcePrivate;
        type ResFilePrivate;
        type WorkspaceRootPrivate;
        type FsTreeEntryPrivate;
        type LanguageTablePrivate;

        fn get_file_type(ds: &DataSource, path: &str) -> FileType;

        // Workspace
        fn workspace_new() -> Workspace;

        fn workspace_from(path: &str) -> Result<Workspace>;

        fn from(self: &mut Workspace, path: &str) -> Result<()>;

        fn reset(self: &mut Workspace);

        fn add_dir(self: &mut Workspace, path: &str) -> Result<()>;

        fn add_zip(self: &mut Workspace, path: &str) -> Result<()>;

        fn detach(self: &mut Workspace, root: &WorkspaceRoot);

        fn root_count(self: &Workspace) -> usize;

        fn by_index(self: &Workspace, idx: usize) -> WorkspaceRoot;

        fn save(self: &Workspace, path: &str) -> Result<()>;

        fn fst_subscribe(self: &mut Workspace, subscriber: Pin<&mut TreeChangeSubscriber>);

        fn fst_unsubscribe(self: &mut Workspace, subscriber: Pin<&mut TreeChangeSubscriber>);

        // WorkspaceRoot
        fn tree(self: &WorkspaceRoot) -> FsTreeEntry;

        fn ds(self: &WorkspaceRoot) -> DataSource;

        fn is_null(self: &WorkspaceRoot) -> bool;

        // FsTreeEntry
        fn fstreeentry_from_ptr(ptr: usize) -> FsTreeEntry;

        fn name(self: &FsTreeEntry) -> String;

        fn file_type(self: &FsTreeEntry) -> FileType;

        fn children_count(self: &FsTreeEntry) -> usize;

        fn by_index1(self: &FsTreeEntry, idx: usize) -> FsTreeEntry;

        fn index_of(self: &FsTreeEntry, child: &FsTreeEntry) -> isize;

        fn parent(self: &FsTreeEntry) -> FsTreeEntry;

        fn root(self: &FsTreeEntry) -> WorkspaceRoot;

        fn path(self: &FsTreeEntry) -> String;

        fn is_root(self: &FsTreeEntry) -> bool;

        fn is_null1(self: &FsTreeEntry) -> bool;

        fn to_ptr(self: &FsTreeEntry) -> usize;

        // DirEntry
        fn file_name(self: &DirEntry) -> &str;

        // DataSource
        fn datasource_open(path: &str) -> Result<DataSource>;

        fn datasource_open_zip(path: &str) -> Result<DataSource>;

        fn open(self: &DataSource, path: &str, mode: &str) -> Result<ResFile>;

        fn create_dir(self: &DataSource, path: &str) -> Result<()>;

        fn create_dir_all(self: &DataSource, path: &str) -> Result<()>;

        fn delete_file(self: &DataSource, path: &str) -> Result<()>;

        fn delete_dir(self: &DataSource, path: &str) -> Result<()>;

        fn delete_dir_all(self: &DataSource, path: &str) -> Result<()>;

        fn list_dir(self: &DataSource, path: &str) -> Result<Vec<DirEntry>>;

        fn read_info(self: &DataSource, path: &str) -> Result<FileInfo>;

        fn is_file(self: &DataSource, path: &str) -> bool;

        fn is_dir(self: &DataSource, path: &str) -> bool;

        fn is_container_zip(self: &DataSource) -> bool;

        // ResFile
        fn read(self: &mut ResFile, buf: &mut [u8]) -> Result<usize>;

        fn write(self: &mut ResFile, buf: &[u8]) -> Result<usize>;

        // LanguageTable
        fn languagetable_new() -> LanguageTable;

        fn languagetable_load(ds: &DataSource, path: &str) -> Result<LanguageTable>;

        fn insert(self: &mut LanguageTable, language: &str, key: &str, value: &str);

        fn add_key(self: &mut LanguageTable, key: &str);

        fn add_language(self: &mut LanguageTable, language: &str);

        fn key_count(self: &LanguageTable) -> usize;

        fn language_count(self: &LanguageTable) -> usize;

        fn get(self: &LanguageTable, language: &str, key: &str) -> Result<String>;

        fn get_language_at(self: &LanguageTable, idx: usize) -> Result<String>;

        fn get_key_at(self: &LanguageTable, idx: usize) -> Result<String>;

        fn save1(self: &LanguageTable, ds: &DataSource, path: &str) -> Result<()>;
    }
}

pub fn tcs_pre_insert(s: *mut TreeChangeSubscriber, path: &Vec<usize>, start: usize, end: usize) {
    unsafe { types::tcs_pre_insert(Pin::new_unchecked(s.as_mut().unwrap()), path, start, end); }
}

pub fn tcs_post_insert(s: *mut TreeChangeSubscriber, path: &Vec<usize>) {
    unsafe { types::tcs_post_insert(Pin::new_unchecked(s.as_mut().unwrap()), path); }
}

pub fn tcs_pre_remove(s: *mut TreeChangeSubscriber, path: &Vec<usize>, start: usize, end: usize) {
    unsafe { types::tcs_pre_remove(Pin::new_unchecked(s.as_mut().unwrap()), path, start, end); }
}

pub fn tcs_post_remove(s: *mut TreeChangeSubscriber, path: &Vec<usize>) {
    unsafe { types::tcs_post_remove(Pin::new_unchecked(s.as_mut().unwrap()), path); }
}

fn get_file_type(ds: &types::DataSource, path: &str) -> types::FileType {
    crate::get_file_type(&ds.inner, path).into()
}

fn workspace_new() -> types::Workspace {
    types::Workspace { inner: Box::new(Workspace::new().into()) }
}

fn workspace_from(path: &str) -> workspace::Result<types::Workspace> {
    Ok(types::Workspace { inner: Box::new(Workspace::read_from(BufReader::new(File::open(path)?))?.into()) })
}

impl types::Workspace {
    fn from(&mut self, path: &str) -> workspace::Result<()> {
        self.inner.read_from_in_place(BufReader::new(File::open(path)?))?;
        Ok(())
    }

    fn reset(&mut self) {
        self.inner.reset();
    }

    fn add_dir(&mut self, path: &str) -> io::Result<()> {
        self.inner.add_dir(path)
    }

    fn add_zip(&mut self, path: &str) -> zip::Result<()> {
        self.inner.add_zip(path)
    }

    fn detach(&mut self, root: &types::WorkspaceRoot) {
        if let Some(root) = &**root.inner {
            self.inner.detach(root)
        }
    }

    fn root_count(&self) -> usize {
        let inner: &WorkspacePrivate = &self.inner;
        inner.roots().len()
    }

    fn by_index(&self, idx: usize) -> types::WorkspaceRoot {
        let inner: &WorkspacePrivate = &self.inner;
        types::WorkspaceRoot { inner: Box::new(inner.roots().get(idx).cloned().into()) }
    }

    fn save(&self, path: &str) -> workspace::Result<()> {
        self.inner.write_into(BufWriter::new(File::create(path)?))?;

        Ok(())
    }

    fn fst_subscribe(&mut self, subscriber: Pin<&mut types::TreeChangeSubscriber>) {
        self.inner.fst_dispatcher_mut().cpp_subscribe(unsafe { subscriber.get_unchecked_mut() } as *mut _);
    }

    fn fst_unsubscribe(&mut self, subscriber: Pin<&mut types::TreeChangeSubscriber>) {
        self.inner.fst_dispatcher_mut().cpp_unsubscribe(unsafe { subscriber.get_unchecked_mut() } as *mut _);
    }
}

impl types::WorkspaceRoot {
    fn null() -> Self {
        types::WorkspaceRoot { inner: Box::new(None.into()) }
    }

    fn ds(&self) -> types::DataSource {
        let inner: &Box<WorkspaceRootPrivate> = &self.inner;
        let inner = (**inner).as_ref().expect("can't get DataSource from null WorkspaceRoot");
        types::DataSource { inner: Box::new((**inner).borrow().ds().clone().into()) }
    }

    fn is_null(&self) -> bool {
        self.inner.is_none()
    }

    fn tree(&self) -> types::FsTreeEntry {
        let inner: &Box<WorkspaceRootPrivate> = &self.inner;
        types::FsTreeEntry { inner: Box::new((**inner).as_ref().map(|e| (**e).borrow().root().clone()).into()) }
    }
}

// This is unsafe! Can't put unsafe on it though because cxx won't take it
// Takes usize because cxx doesn't support pointer types yet
fn fstreeentry_from_ptr(ptr: usize) -> types::FsTreeEntry {
    let ptr = ptr as *const RefCell<FsTreeEntry>;
    if ptr.is_null() {
        types::FsTreeEntry::null()
    } else {
        let rc = unsafe { Rc::from_raw(ptr) };
        mem::forget(rc.clone()); // bump ref counter by 1 since the pointer can be used multiple times
        types::FsTreeEntry { inner: Box::new(Some(rc).into()) }
    }
}

impl types::FsTreeEntry {
    fn null() -> Self {
        types::FsTreeEntry { inner: Box::new(None.into()) }
    }

    fn name(&self) -> String {
        let inner: &Box<FsTreeEntryPrivate> = &self.inner;
        (**inner).as_ref()
            .map(|s| (**s).borrow().display_name().into())
            .unwrap_or_default()
    }

    fn file_type(&self) -> types::FileType {
        let inner: &Box<FsTreeEntryPrivate> = &self.inner;
        (**inner).as_ref().map(|s| (**s).borrow().file_type().into()).unwrap_or_default()
    }

    fn children_count(&self) -> usize {
        let inner: &Box<FsTreeEntryPrivate> = &self.inner;
        (**inner).as_ref()
            .map(|a| (**a).borrow().children().len())
            .unwrap_or_default()
    }

    fn by_index1(&self, idx: usize) -> types::FsTreeEntry {
        let inner: &Box<FsTreeEntryPrivate> = &self.inner;
        let content = (**inner).as_ref()
            .and_then(|a| (**a).borrow().children().get(idx).cloned());
        types::FsTreeEntry { inner: Box::new(content.into()) }
    }

    fn index_of(&self, child: &types::FsTreeEntry) -> isize {
        let inner: &Box<FsTreeEntryPrivate> = &self.inner;
        let ch_inner: &Box<FsTreeEntryPrivate> = &child.inner;
        (**ch_inner).as_ref().and_then(|ch_inner|
            (**inner).as_ref()
                .and_then(|a| (**a).borrow().index_of(&ch_inner)))
            .map_or(-1, |a| a as isize)
    }

    fn parent(&self) -> types::FsTreeEntry {
        let inner: &Box<FsTreeEntryPrivate> = &self.inner;
        (**inner).as_ref()
            .and_then(|e| (**e).borrow().parent().clone())
            .and_then(|s| s.upgrade())
            .map_or_else(|| types::FsTreeEntry::null(), |s| types::FsTreeEntry { inner: Box::new(Some(s).into()) })
    }

    fn root(&self) -> types::WorkspaceRoot {
        let inner: &Box<FsTreeEntryPrivate> = &self.inner;
        (**inner).as_ref()
            .map(|e| (**e).borrow().root().clone())
            .and_then(|s| s.upgrade())
            .map_or_else(|| types::WorkspaceRoot::null(), |s| types::WorkspaceRoot { inner: Box::new(Some(s).into()) })
    }

    fn path(&self) -> String {
        let inner: &Box<FsTreeEntryPrivate> = &self.inner;
        (**inner).as_ref()
            .map(|e| (**e).borrow().path().to_str().unwrap().to_string())
            .unwrap_or_default()
    }

    fn is_root(&self) -> bool {
        let inner: &Box<FsTreeEntryPrivate> = &self.inner;
        Option::as_ref(inner).map(|e| (**e).borrow().is_root()).unwrap_or(false)
    }

    fn is_null1(&self) -> bool {
        self.inner.is_none()
    }

    // Returns usize because cxx doesn't support pointer types yet
    fn to_ptr(&self) -> usize {
        let inner: &Box<FsTreeEntryPrivate> = &self.inner;
        match ***inner {
            Some(ref a) => (Rc::as_ptr(a)) as usize,
            None => 0
        }
    }
}

impl types::DirEntry {
    fn file_name(&self) -> &str {
        let p = Path::new(&self.path);
        p.file_name().unwrap().to_str().unwrap()
    }
}

fn datasource_open(path: &str) -> Result<types::DataSource, io::Error> {
    Ok(types::DataSource { inner: Box::new(Rc::new(DataSource::Dir(dir::DataSource::new(path)?)).into()) })
}

fn datasource_open_zip(path: &str) -> Result<types::DataSource, zip::Error> {
    Ok(types::DataSource { inner: Box::new(Rc::new(DataSource::Zip(zip::DataSource::new(path)?)).into()) })
}

impl types::DataSource {
    fn open(&self, path: &str, mode: &str) -> Result<types::ResFile, matryoshka::Error> {
        let mut opts = matryoshka::OpenOptions::new();

        for char in mode.chars() {
            match char {
                'r' => { opts.read(true); }
                'w' => { opts.write(true); }
                'c' => { opts.create(true); }
                'a' => { opts.append(true); }
                _ => {}
            }
        }

        Ok(types::ResFile { inner: Box::new(self.inner.open(path, opts)?.into()) })
    }

    fn create_dir(&self, path: &str) -> Result<(), matryoshka::Error> {
        self.inner.create_dir(path)
    }

    fn create_dir_all(&self, path: &str) -> Result<(), matryoshka::Error> {
        self.inner.create_dir_all(path)
    }

    fn delete_file(&self, path: &str) -> Result<(), matryoshka::Error> {
        self.inner.delete_file(path)
    }

    fn delete_dir(&self, path: &str) -> Result<(), matryoshka::Error> {
        self.inner.delete_dir(path)
    }

    fn delete_dir_all(&self, path: &str) -> Result<(), matryoshka::Error> {
        self.inner.delete_dir_all(path)
    }

    fn list_dir(&self, path: &str) -> Result<Vec<types::DirEntry>, matryoshka::Error> {
        self.inner.list_dir(path).map(|v| v.into_iter().map(|a| a.into()).collect())
    }

    fn read_info(&self, path: &str) -> Result<types::FileInfo, matryoshka::Error> {
        self.inner.read_info(path).map(|v| v.into())
    }

    fn is_file(&self, path: &str) -> bool {
        self.inner.is_file(path)
    }

    fn is_dir(&self, path: &str) -> bool {
        self.inner.is_dir(path)
    }

    fn is_container_zip(&self) -> bool {
        match &***self.inner {
            DataSource::Zip(_) => true,
            _ => false,
        }
    }
}

impl types::ResFile {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }

    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner.write(buf)
    }
}

fn languagetable_new() -> types::LanguageTable {
    types::LanguageTable { inner: Box::new(LanguageTable::new().into()) }
}

fn languagetable_load(ds: &types::DataSource, path: &str) -> langtable::Result<types::LanguageTable> {
    Ok(types::LanguageTable { inner: Box::new(LanguageTable::load(&ds.inner, path)?.into()) })
}

impl types::LanguageTable {
    fn insert(&mut self, language: &str, key: &str, value: &str) {
        self.inner.insert(language, key, value);
    }

    fn add_key(&mut self, key: &str) {
        self.inner.add_key(key);
    }

    fn add_language(&mut self, language: &str) {
        self.inner.add_language(language);
    }

    fn key_count(&self) -> usize {
        self.inner.key_count()
    }

    fn language_count(&self) -> usize {
        self.inner.lang_count()
    }

    fn get(&self, language: &str, key: &str) -> Result<String, &'static str> {
        self.inner.get(language, key).map(|s| s.to_string()).ok_or("entry not found in table")
    }

    fn get_language_at(&self, idx: usize) -> Result<String, &'static str> {
        self.inner.get_language_at(idx).map(|s| s.to_string()).ok_or("language not found in table")
    }

    fn get_key_at(&self, idx: usize) -> Result<String, &'static str> {
        self.inner.get_key_at(idx).map(|s| s.to_string()).ok_or("key not found in table")
    }

    fn save1(&self, ds: &types::DataSource, path: &str) -> langtable::Result<()> {
        self.inner.save(&ds.inner, path)
    }
}

impl From<matryoshka::DirEntry> for types::DirEntry {
    fn from(e: matryoshka::DirEntry) -> Self {
        types::DirEntry {
            path: e.path().to_str().expect("invalid characters in path for UTF-8 string").to_string(),
            info: e.info().into(),
        }
    }
}

impl From<matryoshka::FileInfo> for types::FileInfo {
    fn from(fi: matryoshka::FileInfo) -> Self {
        types::FileInfo {
            is_file: fi.is_file(),
            is_dir: fi.is_dir(),
            is_symlink: fi.is_symlink(),
            read_only: fi.read_only(),
        }
    }
}

impl From<crate::FileType> for types::FileType {
    fn from(t: crate::FileType) -> Self {
        match t {
            crate::FileType::Language => types::FileType::FILETYPE_LANGUAGE,
            crate::FileType::LanguagePart => types::FileType::FILETYPE_LANGUAGE_PART,
            crate::FileType::Recipe => types::FileType::FILETYPE_RECIPE,
        }
    }
}

impl From<Option<crate::FileType>> for types::FileType {
    fn from(t: Option<FileType>) -> Self {
        match t {
            None => types::FileType::FILETYPE_NONE,
            Some(t) => t.into(),
        }
    }
}

impl Default for types::FileType {
    fn default() -> Self { types::FileType::FILETYPE_NONE }
}