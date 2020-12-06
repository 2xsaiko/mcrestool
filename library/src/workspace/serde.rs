
use std::io;
use std::io::{Read, Write};
use std::num::TryFromIntError;

use byteorder::{ReadBytesExt, WriteBytesExt, BE, LE};

use binserde::Mode;

use crate::workspace::{Error, Workspace};

pub const MAGIC: u16 = 0x3B1C;
pub const VERSION: u16 = 1;
pub const MIN_VERSION: u16 = 1;

impl Workspace {
    pub fn read_from<R: Read>(pipe: R) -> Result<Self> {
        let mut ws = Workspace::new();
        ws.read_from_in_place(pipe)?;
        Ok(ws)
    }

    pub fn read_from_in_place<R: Read>(&mut self, mut pipe: R) -> Result<()> {
        let magic = pipe.read_u16::<BE>()?;
        if magic != MAGIC {
            return Err(Error::MagicError(magic));
        }

        let version = pipe.read_u16::<LE>()?;
        if version < MIN_VERSION || version > VERSION {
            return Err(Error::FileVersionError(version));
        }

        self.gd.reset();
        self.fst.reset();

        binserde::deserialize_in_place(self, pipe, Mode::dedup())?;

        Ok(())
    }

    pub fn write_into<W: Write>(&self, mut pipe: W) -> Result<()> {
        pipe.write_u16::<BE>(MAGIC)?;
        pipe.write_u16::<LE>(VERSION)?;

        binserde::serialize_with_into(pipe, self, Mode::dedup())?;

        Ok(())
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid magic: {0:02X}")]
    MagicError(u16),
    #[error("unimplemented file version: {0}")]
    FileVersionError(u16),
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("string too long")]
    TryFromInt(#[from] TryFromIntError),
    #[error("invalid string")]
    InvalidString,
    #[error("{0}")]
    BinSerde(#[from] binserde::Error),
}
