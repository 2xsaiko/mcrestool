use std::convert::TryInto;
use std::io::{Read, Write};
use std::io;
use std::num::TryFromIntError;
use std::string::FromUtf8Error;

use byteorder::{LE, ReadBytesExt, WriteBytesExt};
use thiserror::Error;

pub use dedup::{ReadContext, WriteContext};

mod dedup;
mod varint;

pub trait WriteExt {
    fn write_str(&mut self, s: &str) -> Result<usize>;

    fn write_varuint(&mut self, i: u64) -> Result<usize>;

    fn write_varint(&mut self, i: i64) -> Result<usize>;
}

impl<W: Write> WriteExt for W {
    fn write_str(&mut self, s: &str) -> Result<usize, Error> {
        self.write_u16::<LE>(s.len().try_into()?)?;
        self.write(s.as_bytes())?;
        Ok(2 + s.len())
    }

    fn write_varuint(&mut self, i: u64) -> Result<usize, Error> {
        unimplemented!()
    }

    fn write_varint(&mut self, i: i64) -> Result<usize, Error> {
        self.write_varuint(encode_min(i))
    }
}

pub trait ReadExt {
    fn read_str(&mut self) -> Result<String>;

    fn read_varuint(&mut self) -> Result<u64>;

    fn read_varint(&mut self) -> Result<i64>;
}

impl<R: Read> ReadExt for R {
    fn read_str(&mut self) -> Result<String> {
        let len = self.read_u16::<LE>()?;
        let mut buf = vec![0; len as usize];
        self.read_exact(&mut buf)?;
        Ok(String::from_utf8(buf)?)
    }

    fn read_varuint(&mut self) -> Result<u64, Error> {
        unimplemented!()
    }

    fn read_varint(&mut self) -> Result<i64, Error> {
        Ok(decode_min(self.read_varuint()?))
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("string too long")]
    TryFromInt(#[from] TryFromIntError),
    #[error("invalid UTF-8 string")]
    InvalidUtf8(#[from] FromUtf8Error),
    #[error("indexed string out of range: {0}")]
    StrOutOfRange(usize),
}