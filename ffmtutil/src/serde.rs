use std::io::{Read, Write};

use crate::dedup::DedupContext;
use crate::Result;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Mode {
    pub usize_len: UsizeLen,
    pub dedup_idx: UsizeLen,
    pub fixed_size_use_varint: bool,
    pub use_dedup: bool,
}

impl Default for Mode {
    fn default() -> Self {
        Mode {
            usize_len: UsizeLen::Variable,
            dedup_idx: UsizeLen::Variable,
            fixed_size_use_varint: false,
            use_dedup: true,
        }
    }
}

impl Mode {
    pub fn with_usize_len(mut self, usize_len: UsizeLen) -> Self {
        self.usize_len = usize_len;
        self
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum UsizeLen {
    U8,
    U16,
    U32,
    U64,
    Variable,
}

pub trait BinSerialize {
    fn serialize<W: Write>(
        &self,
        pipe: W,
        dedup: &mut DedupContext,
        mode: &Mode,
    ) -> Result<()>;
}

pub trait BinDeserialize<'de>: Sized {
    fn deserialize<R: Read>(pipe: R, dedup: &'de DedupContext, mode: &Mode) -> Result<Self>;

    fn deserialize_in_place<R: Read>(
        &mut self,
        pipe: R,
        dedup: &'de DedupContext,
        mode: &Mode,
    ) -> Result<()> {
        *self = Self::deserialize(pipe, dedup, mode)?;
        Ok(())
    }
}

pub trait BinDeserializeOwned: for<'de> BinDeserialize<'de> {}
impl<T> BinDeserializeOwned for T where T: for<'de> BinDeserialize<'de> {}
