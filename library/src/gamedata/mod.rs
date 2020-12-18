use std::borrow::Cow;
use std::cell::{Ref, RefCell, RefMut};
use std::collections::{HashMap, HashSet};
use std::ffi::OsStr;
use std::io::Read;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use binserde::{BinDeserialize, BinSerialize};
use matryoshka::OpenOptions;
use mcplatfm::{Ident, Identifier};

use crate::workspace::{FsTreeRoot, TreeChangeDispatcher};

pub mod serde;

#[derive(BinDeserialize, BinSerialize)]
pub struct GameData {
    refs: GameDataReferences,

    blocks: Registry<Block>,
    items: Registry<Item>,

    #[binserde(skip)]
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

    pub fn collect_usages(&mut self, roots: &[Rc<RefCell<FsTreeRoot>>]) {
        self.refs.map.clear();

        for x in roots.iter() {
            let x = x.borrow();
            let ds = match x.data() {
                None => continue,
                Some(data) => data.ds(),
            };

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

        self.blocks.iter_mut().for_each(|b| b.mark_auto(false));
        self.items.iter_mut().for_each(|i| i.mark_auto(false));

        for entry in vs {
            match entry {
                DependencyLink::Block(id) => {
                    match self
                        .blocks
                        .register(Block::new(GameObjectBase::auto(id.clone())))
                    {
                        Ok(_) => {}
                        Err(b) => {
                            b.mark_auto(true);
                        }
                    }
                }
                DependencyLink::Item(id) => {
                    match self
                        .items
                        .register(Item::new(GameObjectBase::auto(id.clone())))
                    {
                        Ok(_) => {}
                        Err(i) => {
                            i.mark_auto(true);
                        }
                    }
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

    pub fn blocks(&self) -> &Registry<Block> {
        &self.blocks
    }

    pub fn items(&self) -> &Registry<Item> {
        &self.items
    }

    pub fn get_block(&self, id: &Ident) -> Option<&Block> {
        self.blocks.by_id(id)
    }
}

#[derive(BinSerialize, BinDeserialize)]
pub struct Registry<T> {
    inner: Vec<T>,
}

impl<T> Registry<T> {
    pub fn new() -> Self {
        Registry { inner: Vec::new() }
    }
}

impl<T> Default for Registry<T> {
    fn default() -> Self {
        Registry::new()
    }
}

impl<T> Registry<T>
where
    T: GameObject,
{
    pub fn register(&mut self, object: T) -> Result<&mut T, &mut T> {
        let id = object.base().id();

        match self.find(id) {
            Ok(idx) => {
                // this id already exists
                Err(&mut self.inner[idx])
            }
            Err(idx) => {
                self.inner.insert(idx, object);
                Ok(&mut self.inner[idx])
            }
        }
    }

    pub fn deregister(&mut self, id: &Ident) -> Result<T, ()> {
        match self.find(id) {
            Ok(idx) => Ok(self.inner.remove(idx)),
            Err(_) => Err(()),
        }
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    pub fn ids(&self) -> impl Iterator<Item = &Ident> {
        self.inner.iter().map(|el| el.base().id())
    }

    pub fn by_id(&self, id: &Ident) -> Option<&T> {
        let idx = self.find(id).ok()?;
        Some(&self.inner[idx])
    }

    pub fn by_id_mut(&mut self, id: &Ident) -> Option<&mut T> {
        let idx = self.find(id).ok()?;
        Some(&mut self.inner[idx])
    }

    pub fn contains(&self, id: &Ident) -> bool {
        self.find(id).is_ok()
    }

    fn find(&self, id: &Ident) -> Result<usize, usize> {
        self.inner.binary_search_by_key(&id, |el| el.base().id())
    }
}

impl<T> Deref for Registry<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for Registry<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[derive(BinSerialize, BinDeserialize)]
struct GameDataReferences {
    map: HashMap<DependencyLink, HashSet<DependencyLink>>,
}

impl GameDataReferences {
    pub fn insert(&mut self, key: DependencyLink, value: DependencyLink) {
        self.map.entry(key).or_default().insert(value);
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, BinDeserialize, BinSerialize)]
enum DependencyLink {
    Language(String, String),
    Block(Identifier),
    Item(Identifier),
}

pub trait GameObject {
    fn base(&self) -> &GameObjectBase;

    fn base_mut(&mut self) -> &mut GameObjectBase;
}

macro_rules! impl_game_object {
    ($t:ty, $field:ident) => {
        impl GameObject for $t {
            fn base(&self) -> &GameObjectBase {
                &self.$field
            }

            fn base_mut(&mut self) -> &mut GameObjectBase {
                &mut self.$field
            }
        }
    };
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

    pub fn id(&self) -> &Ident {
        &self.id
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

#[derive(BinSerialize, BinDeserialize)]
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

impl_game_object!(Block, base);

#[derive(BinSerialize, BinDeserialize)]
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

impl_game_object!(Item, base);
