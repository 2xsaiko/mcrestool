use std::io::{Read, Write};

use crate::dedup::DedupContext;
use crate::Result;

pub trait BinSerialize {
    fn serialize<S: BinSerializer>(&self, serializer: S) -> Result<()>;
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

pub trait BinSerializer: Sized {
    type Pipe: Write;

    fn pipe(&mut self) -> &mut Self::Pipe;

    fn dedup(&mut self) -> &mut DedupContext;

    fn mode(&self) -> Mode;

    fn with_mode(self, mode: Mode) -> WithMode<Self> {
        WithMode {
            serializer: self,
            mode,
        }
    }

    fn change_mode<F>(self, op: F) -> WithMode<Self>
    where
        F: FnOnce(&mut Mode),
    {
        let mut new_mode = self.mode();
        op(&mut new_mode);
        self.with_mode(new_mode)
    }
}

impl<T> BinSerializer for &mut T
where
    T: BinSerializer,
{
    type Pipe = T::Pipe;

    fn pipe(&mut self) -> &mut Self::Pipe {
        (**self).pipe()
    }

    fn dedup(&mut self) -> &mut DedupContext {
        (**self).dedup()
    }

    fn mode(&self) -> Mode {
        (**self).mode()
    }
}

pub struct BinSerializerBase<W> {
    pipe: W,
    dedup: DedupContext,
}

impl<W> BinSerializerBase<W> {
    pub fn new(pipe: W) -> Self {
        BinSerializerBase {
            pipe,
            dedup: DedupContext::new(),
        }
    }

    pub fn into_pipe(self) -> W {
        self.pipe
    }
}

impl<W> BinSerializer for BinSerializerBase<W>
where
    W: Write,
{
    type Pipe = W;
    fn pipe(&mut self) -> &mut Self::Pipe {
        &mut self.pipe
    }

    fn dedup(&mut self) -> &mut DedupContext {
        &mut self.dedup
    }

    fn mode(&self) -> Mode {
        Mode::default()
    }
}

pub struct WithMode<S> {
    serializer: S,
    mode: Mode,
}

impl<S> BinSerializer for WithMode<S>
where
    S: BinSerializer,
{
    type Pipe = S::Pipe;
    fn pipe(&mut self) -> &mut Self::Pipe {
        self.serializer.pipe()
    }

    fn dedup(&mut self) -> &mut DedupContext {
        self.serializer.dedup()
    }

    fn mode(&self) -> Mode {
        self.mode
    }
}

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
