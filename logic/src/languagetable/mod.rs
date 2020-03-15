use std::collections::HashMap;
use std::path::Path;

use crate::datasource::{DataSource, OpenOptions};
use crate::datasource;
use crate::ffi::McrtError;
use crate::ffihelper::FfiError;

pub mod ffi;

pub struct LanguageTable {
    table: HashMap<String, HashMap<String, String>>,
    languages: Vec<String>,
    localization_keys: Vec<String>,
}

impl LanguageTable {
    pub fn new() -> Self {
        LanguageTable {
            table: Default::default(),
            languages: vec![],
            localization_keys: vec![],
        }
    }

    pub fn add_language(&mut self, name: &str) {
        if !self.languages.iter().any(|s| s == name) {
            self.languages.push(name.to_owned());
        }
    }

    pub fn add_localization_key(&mut self, key: &str) {
        if !self.localization_keys.iter().any(|s| s == key) {
            self.localization_keys.push(key.to_owned());
        }
    }

    pub fn insert(&mut self, lang: &str, key: &str, text: &str) -> Option<String> {
        self.add_language(lang);
        self.add_localization_key(key);
        self.table.entry(lang.to_owned()).or_default().insert(key.to_owned(), text.to_owned())
    }

    pub fn get(&self, lang: &str, key: &str) -> Option<&str> {
        Some(self.table.get(lang)?.get(key)?)
    }

    pub fn column_name(&self, idx: usize) -> Option<&str> {
        Some(self.languages.get(idx)?)
    }

    pub fn row_name(&self, idx: usize) -> Option<&str> {
        Some(self.localization_keys.get(idx)?)
    }

    fn replace_language(&mut self, lang: &str, map: HashMap<String, String>) -> Option<HashMap<String, String>> {
        map.keys().for_each(|el| self.add_localization_key(el));
        self.table.insert(lang.to_owned(), map)
    }

    pub fn remove(&mut self, lang: &str, key: &str) -> Option<String> {
        self.table.get_mut(lang)?.remove(key)
    }

    pub fn row_count(&self) -> usize { self.localization_keys.len() }

    pub fn column_count(&self) -> usize { self.languages.len() }

    pub fn write_to(&self, ds: &DataSource, dir: impl AsRef<Path>) -> Result<(), Error> {
        for x in ds.list_dir(&dir)? {
            ds.delete_file(dir.as_ref().join(x))?;
        }
        for (lang, map) in self.table.iter() {
            let path = dir.as_ref().join(format!("{}.json", lang));
            let mut file = ds.open(path, OpenOptions::writing(true))?;
            serde_json::to_writer_pretty(&mut file, map)?;
        }
        Ok(())
    }

    pub fn read_from(ds: &DataSource, dir: impl AsRef<Path>) -> Result<LanguageTable, Error> {
        let mut lt = LanguageTable::new();
        for x in ds.list_dir(&dir)? {
            if x.extension().unwrap().to_str() == Some("json") {
                let file_name = x.file_name().unwrap().to_string_lossy();
                let lang = &file_name[..file_name.len() - 5];
                let mut file = ds.open(&x, OpenOptions::reading())?;
                let map: HashMap<String, String> = serde_json::from_reader(&mut file)?;
                lt.replace_language(lang, map);
            }
        }
        Ok(lt)
    }
}

pub enum Error {
    IoError(datasource::Error),
    SerdeError(serde_json::Error),
}

impl From<datasource::Error> for Error {
    fn from(err: datasource::Error) -> Self {
        Error::IoError(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::SerdeError(err)
    }
}

impl FfiError for Error {
    fn kind(&self) -> McrtError {
        match self {
            Error::IoError(e) => e.kind(),
            Error::SerdeError(_) => McrtError::CorruptedFile,
        }
    }
}