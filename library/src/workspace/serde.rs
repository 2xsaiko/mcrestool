use std::convert::TryInto;
use std::io;
use std::io::{Read, Write};
use std::num::TryFromIntError;

use byteorder::{ReadBytesExt, WriteBytesExt, BE, LE};

use ffmtutil::dedup::DedupContext;
use ffmtutil::serde::{BinSerialize, Mode, BinDeserialize};
use matryoshka::DataSource;

use crate::workspace::fstree::FsTree;
use crate::workspace::{Error, Workspace, WorkspaceRoot};

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

ffmtutil::impl_serde_wrap! {
    struct Workspace { fst, gd }
}

impl<'de> BinDeserialize<'de> for FsTree {
    fn deserialize<R: Read>(pipe: R, dedup: &'de DedupContext, mode: &Mode) -> ffmtutil::Result<Self> {
        let mut tree = FsTree::new();
        tree.deserialize_in_place(pipe, dedup, mode)?;
        Ok(tree)
    }

    fn deserialize_in_place<R: Read>(&mut self, mut pipe: R, dedup: &'de DedupContext, mode: &Mode) -> ffmtutil::Result<()> {
        self.reset();

        for _ in 0..u16::deserialize(&mut pipe, dedup, mode)? {
            let is_dir = bool::deserialize(&mut pipe, dedup, mode)?;

            let path = String::deserialize(&mut pipe, dedup, mode)?;
            let name = String::deserialize(&mut pipe, dedup, mode)?;

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
}

impl BinSerialize for FsTree {
    fn serialize<W: Write>(
        &self,
        mut pipe: W,
        dedup: &mut DedupContext,
        mode: &Mode,
    ) -> ffmtutil::Result<()> {
        pipe.write_u16::<LE>(self.roots().len().try_into()?)?;
        for r in self.roots() {
            let r = r.borrow();
            let (is_dir, path) = match &*r.ds {
                DataSource::Dir(ds) => (true, ds.root()),
                DataSource::Zip(ds) => (false, ds.zip_path()),
            };

            is_dir.serialize(&mut pipe, dedup, mode)?;
            let path = path.to_str().ok_or(Error::InvalidString)?;
            path.serialize(&mut pipe, dedup, mode)?;
            r.name().serialize(&mut pipe, dedup, mode)?;

            Ok(())

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
