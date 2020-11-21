use std::convert::TryInto;
use std::io::{Read, Write};
use std::io;
use std::num::TryFromIntError;
use std::string::FromUtf8Error;

use byteorder::{BE, LE, ReadBytesExt, WriteBytesExt};

use matryoshka::DataSource;

use crate::binserde::{self, read_str, write_str};
use crate::workspace::{Error, Workspace};
use crate::workspace::fstree::FsTree;

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
        self.fst.reset();

        let magic = pipe.read_u16::<BE>()?;
        if magic != MAGIC {
            return Err(Error::MagicError(magic));
        }

        let version = pipe.read_u16::<LE>()?;
        if version < MIN_VERSION || version > VERSION {
            return Err(Error::FileVersionError(version));
        }

        self.fst.read_from_in_place(&mut pipe)?;
        self.update_refs();

        Ok(())
    }

    pub fn write_into<W: Write>(&self, mut pipe: W) -> Result<()> {
        pipe.write_u16::<BE>(MAGIC)?;
        pipe.write_u16::<LE>(VERSION)?;

        self.fst.write_into(&mut pipe)?;
        self.gd.write_into(&mut pipe)?;

        Ok(())
    }
}

impl FsTree {
    fn read_from_in_place<R: Read>(&mut self, mut pipe: R) -> Result<()> {
        for _ in 0..pipe.read_u16::<LE>()? {
            let is_dir = pipe.read_u8()? != 0;

            let path = read_str(&mut pipe)?;
            let name = read_str(&mut pipe)?;

            if is_dir {
                if let Err(_) = self.add_dir_with_name(path, name) {
                    eprintln!("Failed to add workspace root, skipping");
                }
            } else {
                if let Err(_) = self.add_zip_with_name(path, name) {
                    eprintln!("Failed to add workspace root, skipping");
                }
            }
        }
        Ok(())
    }

    fn write_into<W: Write>(&self, mut pipe: W) -> Result<()> {
        pipe.write_u16::<LE>(self.roots().len().try_into()?)?;
        for r in self.roots() {
            let r = r.borrow();
            let (is_dir, path) = match &*r.ds {
                DataSource::Dir(ds) => (false, ds.root()),
                DataSource::Zip(ds) => (true, ds.zip_path()),
            };

            pipe.write_u8(if is_dir { 1 } else { 0 })?;
            let path = path.to_str().ok_or(Error::InvalidString)?;
            write_str(path, &mut pipe)?;
            write_str(r.name(), &mut pipe)?;
        }

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
    #[error("invalid UTF-8 string")]
    InvalidUtf8(#[from] FromUtf8Error),
}

impl From<binserde::Error> for Error {
    fn from(e: binserde::Error) -> Self {
        match e {
            binserde::Error::Io(e) => Error::Io(e),
            binserde::Error::TryFromInt(e) => Error::TryFromInt(e),
            binserde::Error::InvalidUtf8(e) => Error::InvalidUtf8(e),
        }
    }
}