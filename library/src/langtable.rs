use std::borrow::Borrow;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::io;
use std::io::Read;
use std::ops::Deref;
use std::path::Path;
use std::rc::Rc;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use thiserror::Error;

use crate::datasource;
use crate::datasource::{DataSource, OpenOptions};

#[derive(Debug, Clone, Default)]
pub struct LanguageTablePart {
    repr: HashMap<RcString, String>,
}

#[derive(Debug, Clone)]
pub struct LanguageTable {
    repr: HashMap<RcString, LanguageTablePart>,
    languages: Vec<RcString>,
    keys: Vec<RcString>,
}

impl LanguageTable {
    pub fn new() -> Self {
        LanguageTable {
            repr: Default::default(),
            languages: vec![],
            keys: vec![],
        }
    }

    pub fn insert<L, K, V>(&mut self, language: L, key: K, value: V)
        where L: Into<RcString>,
              K: Into<RcString>,
              V: Into<String> {
        let language = language.into();
        let key = key.into();
        let value = value.into();
        self.add_language(language.clone());
        self.add_key(key.clone());
        self.repr.entry(language).or_insert_with(|| LanguageTablePart::default()).repr.insert(key, value);
    }

    pub fn add_key<S: Into<RcString>>(&mut self, key: S) -> bool {
        let key = key.into();
        if !self.keys.iter().any(|s| &**s == &*key) {
            self.keys.push(key.into());
            true
        } else {
            false
        }
    }

    pub fn add_language<S: Into<RcString>>(&mut self, lang: S) -> bool {
        let lang = lang.into();
        if !self.languages.iter().any(|s| &**s == &*lang) {
            self.languages.push(lang.into());
            true
        } else {
            false
        }
    }

    pub fn key_count(&self) -> usize { self.keys.len() }

    pub fn lang_count(&self) -> usize { self.languages.len() }

    pub fn get(&self, language: &str, key: &str) -> Option<&str> {
        self.repr.get(language)
            .and_then(|s| s.repr.get(key))
            .map(|s| &**s)
    }

    pub fn get_language_at(&self, idx: usize) -> Option<&RcString> {
        self.languages.get(idx)
    }

    pub fn get_key_at(&self, idx: usize) -> Option<&RcString> {
        self.keys.get(idx)
    }

    pub fn get_part(&self, lang: &str) -> Option<&LanguageTablePart> {
        self.repr.get(lang)
    }

    pub fn contains_language(&self, lang: &str) -> bool {
        self.languages.iter().any(|s| &**s == lang)
    }

    pub fn clear(&mut self) {
        self.repr.clear();
        self.keys.clear();
        self.languages.clear();
    }

    pub fn save<P: AsRef<Path>>(&self, ds: &DataSource, path: P) -> Result<()> {
        let empty = HashMap::new();
        let path = path.as_ref();

        for x in ds.list_dir(path)? {
            if x.path.extension() == Some(OsStr::new("json")) {
                ds.delete_file(&x.path)?;
            }
        }

        for lang in self.languages.iter() {
            let map = self.repr.get(lang).map(|p| &p.repr).unwrap_or(&empty);
            let mut path = path.join(&**lang);
            path.set_extension("json");
            let out = ds.open(path, OpenOptions::writing(true))?;
            serde_json::to_writer(out, &map)?;
        }

        Ok(())
    }

    pub fn load<P: AsRef<Path>>(ds: &DataSource, path: P) -> Result<Self> {
        let mut lt = LanguageTable::new();

        let mut dir = ds.list_dir(path)?;

        // move en_us to the beginning
        let idx = dir.iter().enumerate()
            .find(|(_, e)| e.info.is_file && e.path.file_name() == Some(OsStr::new("en_us.json")))
            .map(|(idx, _)| idx)
            .filter(|&idx| idx > 0);

        if let Some(idx) = idx {
            let en_us = dir.remove(idx);
            dir.insert(0, en_us);
        }

        let mut keys = HashMap::new();

        for entry in dir {
            println!("{:?}", entry);
            if entry.info.is_file && entry.path.extension() == Some(OsStr::new("json")) {
                println!(" - deserialize");
                let lang: RcString = entry.path.file_stem().unwrap().to_str().unwrap().into();
                let mut buf = String::new();
                ds.open(entry.path, OpenOptions::reading())?.read_to_string(&mut buf)?;
                let part: HashMap<RcString, String> = serde_json::from_str(&buf)?;

                println!(" - deduplicate");
                // deduplicate keys
                let mut dedup_keys = Vec::new();
                for k in part.keys() {
                    if !keys.contains_key(k) {
                        keys.insert(k.clone(), k.clone());
                        lt.keys.push(k.clone());
                    } else {
                        dedup_keys.push(keys.get(k).unwrap().clone());
                    }
                }
                for k in dedup_keys {
                    let v = keys.remove(&k).unwrap();
                    keys.insert(k, v);
                }

                println!(" - insert");
                lt.repr.insert(lang.clone(), LanguageTablePart { repr: part });
                lt.languages.push(lang);
            }
        }

        Ok(lt)
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("data source error")]
    DataSource(#[from] datasource::Error),
    #[error("I/O error")]
    Io(#[from] io::Error),
    #[error("serialization error")]
    Serde(#[from] serde_json::Error),
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct RcString(Rc<String>);

impl RcString {
    pub fn from(rc: Rc<String>) -> RcString { RcString(rc) }

    pub fn into_inner(self) -> Rc<String> { self.0 }
}

impl Borrow<str> for RcString {
    fn borrow(&self) -> &str { &self.0 }
}

impl Deref for RcString {
    type Target = str;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<T> From<T> for RcString
    where
        T: Into<String> {
    fn from(s: T) -> Self {
        RcString(Rc::new(s.into()))
    }
}

impl Serialize for RcString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for RcString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de> {
        Ok(String::deserialize(deserializer)?.into())
    }
}