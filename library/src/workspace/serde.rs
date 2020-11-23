use std::convert::TryInto;
use std::io::{Read, Write};
use std::io;
use std::num::TryFromIntError;
use std::string::FromUtf8Error;

use byteorder::{BE, LE, ReadBytesExt, WriteBytesExt};

use ffmtutil::{ReadContext, ReadExt, WriteContext, WriteExt};
use matryoshka::DataSource;

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

        let mut ctx = ReadContext::new(pipe)?;

        self.fst.read_from_in_place(&mut ctx)?;
        self.gd.read_from_in_place(&mut ctx)?;

        Ok(())
    }

    pub fn write_into<W: Write>(&self, mut pipe: W) -> Result<()> {
        pipe.write_u16::<BE>(MAGIC)?;
        pipe.write_u16::<LE>(VERSION)?;

        let mut ctx = WriteContext::new();

        self.fst.write_into(&mut ctx)?;
        self.gd.write_into(&mut ctx)?;

        ctx.write_to(pipe)?;

        Ok(())
    }
}

impl FsTree {
    fn read_from_in_place<R: Read>(&mut self, mut pipe: R) -> Result<()> {
        self.reset();

        for _ in 0..pipe.read_u16::<LE>()? {
            let is_dir = pipe.read_u8()? != 0;

            let path = pipe.read_str()?;
            let name = pipe.read_str()?;

            if is_dir {
                if let Err(e) = self.add_dir_with_name(path, name) {
                    eprintln!("Failed to add workspace root, skipping: {}", e);
                }
            } else {
                if let Err(e) = self.add_zip_with_name(path, name) {
                    eprintln!("Failed to add workspace root, skipping: {:?}", e);
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
                DataSource::Dir(ds) => (true, ds.root()),
                DataSource::Zip(ds) => (false, ds.zip_path()),
            };

            pipe.write_u8(if is_dir { 1 } else { 0 })?;
            let path = path.to_str().ok_or(Error::InvalidString)?;
            pipe.write_str(path)?;
            pipe.write_str(r.name())?;
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
    #[error("{0}")]
    Ffmtutil(#[from] ffmtutil::Error),
}