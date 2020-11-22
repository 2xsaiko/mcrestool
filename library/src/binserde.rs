use std::convert::TryInto;
use std::io::{Read, Write};
use std::io;
use std::num::TryFromIntError;
use std::string::FromUtf8Error;

use byteorder::{LE, ReadBytesExt, WriteBytesExt};
use thiserror::Error;

use crate::ident::{Ident, Identifier};

pub struct WriteContext {
    strings: Vec<String>,
    content: Vec<u8>,
}

impl WriteContext {
    pub fn new() -> Self {
        WriteContext {
            strings: vec![],
            content: vec![],
        }
    }

    pub fn write_dedup_str(&mut self, s: &str) -> Result<()> {
        let idx = self.put_string(s);
        self.write_u32::<LE>(idx.try_into()?)?;
        Ok(())
    }

    pub fn write_dedup_ident(&mut self, id: &Ident) -> Result<()> {
        let idx = self.put_identifier(id);
        self.write_u32::<LE>(idx.try_into()?)?;
        Ok(())
    }

    pub fn put_string(&mut self, s: &str) -> usize {
        match self.strings.binary_search_by(|el| el.as_str().cmp(s)) {
            Ok(idx) => idx,
            Err(idx) => {
                self.strings.insert(idx, s.to_string());
                idx
            }
        }
    }

    pub fn put_identifier(&mut self, id: &Ident) -> usize {
        self.put_string(id.trim().as_str())
    }

    pub fn write_to<W: Write>(&self, mut pipe: W) -> Result<()> {
        pipe.write_u16::<LE>(self.strings.len().try_into()?)?;
        for e in self.strings.iter() {
            write_str(&e, &mut pipe)?;
        }
        pipe.write(&self.content)?;
        Ok(())
    }
}

impl Write for WriteContext {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.content.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub struct ReadContext<R> {
    strings: Vec<String>,
    pipe: R,
}

impl<R: Read> ReadContext<R> {
    pub fn new(mut pipe: R) -> Result<Self> {
        let len = pipe.read_u16::<LE>()?;
        let mut strings = Vec::new();
        for _ in 0..len {
            strings.push(read_str(&mut pipe)?);
        }
        Ok(ReadContext {
            strings,
            pipe,
        })
    }

    pub fn read_dedup_str(&mut self) -> Result<&str> {
        let idx = self.read_u16::<LE>()?;
        self.strings.get(idx as usize).map(|s| s.as_str())
            .ok_or(Error::StrOutOfRange(idx as usize))
    }

    pub fn read_dedup_ident(&mut self) -> Result<&Ident> {
        self.read_dedup_str().map(Ident::new)
    }
}

impl<R: Read> Read for ReadContext<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.pipe.read(buf)
    }
}

pub fn write_str<W: Write>(s: &str, mut pipe: W) -> Result<usize> {
    pipe.write_u16::<LE>(s.len().try_into()?)?;
    pipe.write(s.as_bytes())?;
    Ok(2 + s.len())
}

pub fn read_str<R: Read>(mut pipe: R) -> Result<String> {
    let len = pipe.read_u16::<LE>()?;
    let mut buf = vec![0; len as usize];
    pipe.read_exact(&mut buf)?;
    Ok(String::from_utf8(buf)?)
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