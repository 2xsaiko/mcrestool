use std::io;
use std::io::{Read, Write};

use thiserror::Error;

use crate::dedup::DedupContext;
use crate::Result;

pub trait BinSerialize {
    type Mode;

    fn serialize<W: Write>(
        &self,
        pipe: &mut W,
        dedup: &mut DedupContext,
        mode: &Self::Mode,
    ) -> Result<()>;
}

pub trait BinDeserialize<'de>: Sized {
    type Mode;

    fn deserialize<R: Read>(
        pipe: &mut R,
        dedup: &'de DedupContext,
        mode: &Self::Mode,
    ) -> Result<Self>;
}
