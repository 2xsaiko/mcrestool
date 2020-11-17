use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::ffi::OsStr;
use std::fmt;
use std::fmt::Display;
use std::io::Read;
use std::str::FromStr;

use serde::export::Formatter;

use matryoshka::{Error, OpenOptions};
use matryoshka::resfile::ResFile;

use crate::workspace::Workspace;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Identifier {
    namespace: String,
    path: String,
}

impl Identifier {
    pub fn from<N: Into<String>, P: Into<String>>(namespace: N, path: P) -> Self {
        Identifier {
            namespace: namespace.into(),
            path: path.into(),
        }
    }
}

impl FromStr for Identifier {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (namespace, path) = s.split_once(':').ok_or(())?;

        Ok(Identifier::from(namespace, path))
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.namespace, self.path)
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
enum DependencyLink {
    Language(String, String),
    Block(Identifier),
    Item(Identifier),
}

struct GameDataReferences {
    map: HashMap<DependencyLink, HashSet<DependencyLink>>,
}

impl GameDataReferences {
    pub fn insert(&mut self, key: DependencyLink, value: DependencyLink) {
        self.map.entry(key).or_default().insert(value);
    }
}

pub struct GameData {
    refs: GameDataReferences,

    blocks: HashMap<Identifier, Block>,
    items: HashMap<Identifier, Item>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum GameObjectStatus {
    /// The game object was manually added by the user.
    Manual,

    /// The game object was added as a result of automatically detected
    /// reference from another resource.
    Auto,

    /// The game object was both added by the user and automatically detected.
    Both,

    /// The game object was automatically added (see `Auto`) but was deleted by
    /// the user.
    AutoHidden,
}

pub struct GameObjectBase {
    status: GameObjectStatus,
    id: Identifier,
}

impl GameObjectBase {
    pub fn new(id: Identifier) -> Self {
        GameObjectBase {
            status: GameObjectStatus::Manual,
            id,
        }
    }

    pub fn auto(id: Identifier) -> Self {
        GameObjectBase {
            status: GameObjectStatus::Auto,
            id,
        }
    }

    pub fn mark_manual(&mut self) {
        self.status = match self.status {
            GameObjectStatus::Manual => GameObjectStatus::Manual,
            _ => GameObjectStatus::Both,
        }
    }

    pub fn try_delete(&mut self) -> bool {
        let new_status = match self.status {
            GameObjectStatus::Manual => None,
            _ => Some(GameObjectStatus::AutoHidden),
        };

        match new_status {
            None => true,
            Some(st) => {
                self.status = st;
                false
            }
        }
    }
}

pub struct Block {
    base: GameObjectBase,
}

impl Block {
    pub fn new(base: GameObjectBase) -> Self {
        Block { base }
    }
}

pub struct Item {
    base: GameObjectBase,
}

impl Item {
    pub fn new(base: GameObjectBase) -> Self {
        Item { base }
    }
}

impl GameData {
    pub fn new() -> Self {
        GameData {
            refs: GameDataReferences { map: Default::default() },
            blocks: Default::default(),
            items: Default::default(),
        }
    }

    pub fn collect_usages(&mut self, ws: &Workspace) {
        self.refs.map.clear();

        for x in ws.roots() {
            let x = x.borrow();
            let ds = x.ds();

            for entry in ds.list_dir("assets").unwrap_or_default() {
                if entry.info().is_dir() {
                    let namespace = entry.file_name().to_str().unwrap();

                    for lang_file in ds.list_dir(entry.path().join("lang")).unwrap_or_default() {
                        if lang_file.info().is_file() && lang_file.path().extension() == Some(OsStr::new("json")) {
                            let lang_name = lang_file.path().file_stem().unwrap().to_str().unwrap();
                            let dl_source = DependencyLink::Language(namespace.to_string(), lang_name.to_string());

                            // Read entire file into string to increase speed (serde-rs/json#160)
                            let mut buf = String::new();

                            let mut file = ds.open(lang_file.path(), OpenOptions::reading()).expect("could not open file");

                            if file.read_to_string(&mut buf).is_err() {
                                continue;
                            }

                            let part: HashMap<Cow<str>, Cow<str>> = match serde_json::from_str(&buf) {
                                Ok(v) => v,
                                Err(e) => {
                                    eprintln!("warning: skipping invalid language file: {}", e);
                                    continue;
                                }
                            };

                            for k in part.keys() {
                                if k.starts_with("block.") {
                                    if let Some(block_name) = k[6..].split_once('.') {
                                        let id = Identifier::from(block_name.0, block_name.1);

                                        self.refs.insert(dl_source.clone(), DependencyLink::Block(id));
                                    }
                                } else if k.starts_with("item.") {
                                    if let Some(item_name) = k[6..].split_once('.') {
                                        let id = Identifier::from(item_name.0, item_name.1);

                                        self.refs.insert(dl_source.clone(), DependencyLink::Item(id));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn create_dummies(&mut self) {
        let vs: HashSet<_> = self.refs.map.values()
            .flat_map(|v| v.iter())
            .collect();

        for entry in vs {
            match entry {
                DependencyLink::Block(id) => {
                    self.blocks.entry(id.clone()).or_insert_with(|| Block::new())
                }
                DependencyLink::Item(id) => {}
                _ => {}
            }
        }
    }
}