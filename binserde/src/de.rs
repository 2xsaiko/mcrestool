use std::io::Read;

use crate::dedup::DedupContext;
use crate::serde::Mode;
use crate::Result;

pub trait BinDeserialize<'de>: Sized {
    fn deserialize<D: BinDeserializer<'de>>(deserializer: D) -> Result<Self>;

    fn deserialize_in_place<D: BinDeserializer<'de>>(&mut self, deserializer: D) -> Result<()> {
        *self = Self::deserialize(deserializer)?;
        Ok(())
    }
}

pub trait BinDeserializeOwned: for<'de> BinDeserialize<'de> {}
impl<T> BinDeserializeOwned for T where T: for<'de> BinDeserialize<'de> {}

pub trait BinDeserializer<'de>: Sized {
    type Pipe: Read;

    fn pipe(&mut self) -> &mut Self::Pipe;

    fn dedup(&self) -> &'de DedupContext;

    fn mode(&self) -> Mode;

    fn with_mode(self, mode: Mode) -> WithMode<Self> {
        WithMode {
            deserializer: self,
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

    fn disable_dedup(self) -> WithMode<Self> {
        self.change_mode(|mode| mode.use_dedup = false)
    }
}

impl<'de, T> BinDeserializer<'de> for &mut T
where
    T: BinDeserializer<'de>,
{
    type Pipe = T::Pipe;

    fn pipe(&mut self) -> &mut Self::Pipe {
        (**self).pipe()
    }

    fn dedup(&self) -> &'de DedupContext {
        (**self).dedup()
    }

    fn mode(&self) -> Mode {
        (**self).mode()
    }
}

pub struct BinDeserializerBase<'de, R> {
    pipe: R,
    dedup: &'de DedupContext,
}

impl<'de, R> BinDeserializerBase<'de, R> {
    pub fn new(pipe: R, dedup: &'de DedupContext) -> Self {
        BinDeserializerBase { pipe, dedup }
    }
}

impl<'de, R> BinDeserializer<'de> for BinDeserializerBase<'de, R>
where
    R: Read,
{
    type Pipe = R;

    fn pipe(&mut self) -> &mut Self::Pipe {
        &mut self.pipe
    }

    fn dedup(&self) -> &'de DedupContext {
        self.dedup
    }

    fn mode(&self) -> Mode {
        Mode::default()
    }
}

pub struct WithMode<D> {
    deserializer: D,
    mode: Mode,
}

impl<'de, D> BinDeserializer<'de> for WithMode<D>
where
    D: BinDeserializer<'de>,
{
    type Pipe = D::Pipe;

    fn pipe(&mut self) -> &mut Self::Pipe {
        self.deserializer.pipe()
    }

    fn dedup(&self) -> &'de DedupContext {
        self.deserializer.dedup()
    }

    fn mode(&self) -> Mode {
        self.mode
    }
}
