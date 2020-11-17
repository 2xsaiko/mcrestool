use std::fmt;
use std::fmt::Display;

use serde::export::Formatter;
use std::collections::HashMap;

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Identifier {
    namespace: String,
    path: String,
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.namespace, self.path)
    }
}

enum DependencyLink {
    LanguageTable,
    Block(Identifier),
}

struct GameDataReferences {
    map: HashMap<DependencyLink, Vec<DependencyLink>>,
}

pub struct GameData {
    refs: GameDataReferences,

    blocks: HashMap<Identifier, Block>,
}

pub enum GameObjectStatus {
    Manual,
    Auto,
    AutoHidden,
}

pub struct GameObjectBase {
    status: GameObjectStatus,
}

pub struct Block {
    base: GameObjectBase,
}

impl GameData {
    fn collect_usages(&mut self) {
        unimplemented!()
    }

    fn create_dummies(&mut self) {
        unimplemented!()
    }
}