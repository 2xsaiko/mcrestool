use std::io::Write;

use crate::dedup::DedupContext;
use crate::serde::Mode;
use crate::Result;

pub trait BinSerialize {
    fn serialize<S: BinSerializer>(&self, serializer: S) -> Result<()>;
}

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
