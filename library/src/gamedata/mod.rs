use std::borrow::Cow;
use std::cell::{Ref, RefCell, RefMut};
use std::collections::{HashMap, HashSet};
use std::ffi::OsStr;
use std::io::Read;
use std::rc::Rc;

use matryoshka::OpenOptions;
use mcplatfm::Identifier;

use crate::workspace::{TreeChangeDispatcher, WorkspaceRoot};

pub mod serde;

pub struct GameData {
    refs: GameDataReferences,

    blocks: HashMap<Identifier, Block>,
    items: HashMap<Identifier, Item>,

    dispatcher: Rc<RefCell<TreeChangeDispatcher>>,
}

impl GameData {
    pub fn new() -> Self {
        GameData {
            refs: GameDataReferences {
                map: Default::default(),
            },
            blocks: Default::default(),
            items: Default::default(),
            dispatcher: Rc::new(RefCell::new(TreeChangeDispatcher::new())),
        }
    }

    pub fn reset(&mut self) {
        self.refs.map.clear();
        self.items.clear();
        self.blocks.clear();
    }

    pub fn collect_usages(&mut self, roots: &[Rc<RefCell<WorkspaceRoot>>]) {
        self.refs.map.clear();

        for x in roots.iter() {
            let x = x.borrow();
            let ds = x.ds();

            for entry in ds.list_dir("assets").unwrap_or_default() {
                if entry.info().is_dir() {
                    let namespace = entry.file_name().to_str().unwrap();

                    for lang_file in ds.list_dir(entry.path().join("lang")).unwrap_or_default() {
                        if lang_file.info().is_file()
                            && lang_file.path().extension() == Some(OsStr::new("json"))
                        {
                            let lang_name = lang_file.path().file_stem().unwrap().to_str().unwrap();
                            let dl_source = DependencyLink::Language(
                                namespace.to_string(),
                                lang_name.to_string(),
                            );

                            // Read entire file into string to increase speed (serde-rs/json#160)
                            let mut buf = String::new();

                            let mut file = ds
                                .open(lang_file.path(), OpenOptions::reading())
                                .expect("could not open file");

                            if file.read_to_string(&mut buf).is_err() {
                                continue;
                            }

                            let part: HashMap<Cow<str>, Cow<str>> = match serde_json::from_str(&buf)
                            {
                                Ok(v) => v,
                                Err(e) => {
                                    eprintln!("warning: skipping invalid language file: {}", e);
                                    continue;
                                }
                            };

                            for k in part.keys() {
                                if let Some(k) = k.strip_prefix("block.") {
                                    let mut split = k.split('.');
                                    if let Some(block_name) =
                                        split.next().and_then(|a| split.next().map(|b| (a, b)))
                                    {
                                        let id =
                                            Identifier::from_components(block_name.0, block_name.1);

                                        self.refs
                                            .insert(dl_source.clone(), DependencyLink::Block(id));
                                    }
                                } else if let Some(k) = k.strip_prefix("item.") {
                                    let mut split = k.split('.');
                                    if let Some(item_name) =
                                        split.next().and_then(|a| split.next().map(|b| (a, b)))
                                    {
                                        let id =
                                            Identifier::from_components(item_name.0, item_name.1);

                                        self.refs
                                            .insert(dl_source.clone(), DependencyLink::Item(id));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn create_dummies(&mut self) {
        let vs: HashSet<_> = self.refs.map.values().flat_map(|v| v.iter()).collect();

        self.blocks.values_mut().for_each(|b| b.mark_auto(false));
        self.items.values_mut().for_each(|i| i.mark_auto(false));

        for entry in vs {
            match entry {
                DependencyLink::Block(id) => {
                    let b = self
                        .blocks
                        .entry(id.clone())
                        .or_insert_with(|| Block::new(GameObjectBase::auto(id.clone())));
                    b.mark_auto(true);
                }
                DependencyLink::Item(id) => {
                    let i = self
                        .items
                        .entry(id.clone())
                        .or_insert_with(|| Item::new(GameObjectBase::auto(id.clone())));
                    i.mark_auto(true);
                }
                _ => {}
            }
        }
    }

    pub fn dispatcher(&self) -> Ref<TreeChangeDispatcher> {
        self.dispatcher.borrow()
    }

    pub fn dispatcher_mut(&self) -> RefMut<TreeChangeDispatcher> {
        self.dispatcher.borrow_mut()
    }

    pub fn blocks(&self) -> &HashMap<Identifier, Block> {
        &self.blocks
    }

    pub fn items(&self) -> &HashMap<Identifier, Item> {
        &self.items
    }
}

struct GameDataReferences {
    map: HashMap<DependencyLink, HashSet<DependencyLink>>,
}

impl GameDataReferences {
    pub fn insert(&mut self, key: DependencyLink, value: DependencyLink) {
        self.map.entry(key).or_default().insert(value);
    }
}

ffmtutil::impl_serde_wrap! {
    struct GameData { refs, blocks, items, ..GameData::new() }
    struct GameDataReferences { map }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
enum DependencyLink {
    Language(String, String),
    Block(Identifier),
    Item(Identifier),
}

pub struct GameObjectBase {
    manual: bool,
    auto: AutoStatus,
    id: Identifier,
}

impl GameObjectBase {
    pub fn new(id: Identifier) -> Self {
        GameObjectBase {
            manual: true,
            auto: AutoStatus::No,
            id,
        }
    }

    pub fn auto(id: Identifier) -> Self {
        GameObjectBase {
            manual: false,
            auto: AutoStatus::Yes,
            id,
        }
    }

    pub fn mark_manual(&mut self, flag: bool) {
        self.manual = flag;
    }

    pub fn mark_auto(&mut self, flag: bool) {
        if !flag && self.auto == AutoStatus::Yes {
            self.auto = AutoStatus::No;
        } else if flag && self.auto == AutoStatus::No {
            self.auto = AutoStatus::Yes;
        }
    }

    pub fn marked_for_deletion(&self) -> bool {
        !self.manual && self.auto != AutoStatus::Yes
    }
}

#[derive(Debug, Eq, PartialEq)]
enum AutoStatus {
    No,
    Yes,
    Deleted,
}

pub struct Block {
    base: GameObjectBase,
}

impl Block {
    pub fn new(base: GameObjectBase) -> Self {
        Block { base }
    }

    pub fn mark_manual(&mut self, flag: bool) {
        self.base.mark_manual(flag);
    }

    pub fn mark_auto(&mut self, flag: bool) {
        self.base.mark_auto(flag);
    }

    pub fn marked_for_deletion(&self) -> bool {
        self.base.marked_for_deletion()
    }
}

ffmtutil::impl_serde_wrap! {
    struct Block { base }
}

pub struct Item {
    base: GameObjectBase,
}

impl Item {
    pub fn new(base: GameObjectBase) -> Self {
        Item { base }
    }

    pub fn mark_manual(&mut self, flag: bool) {
        self.base.mark_manual(flag);
    }

    pub fn mark_auto(&mut self, flag: bool) {
        self.base.mark_auto(flag);
    }

    pub fn marked_for_deletion(&self) -> bool {
        self.base.marked_for_deletion()
    }
}

ffmtutil::impl_serde_wrap! {
    struct Item { base }
}
