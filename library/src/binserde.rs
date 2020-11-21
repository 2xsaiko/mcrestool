use std::convert::TryInto;
use std::io::{Read, Write};
use std::io;
use std::num::TryFromIntError;
use std::string::FromUtf8Error;

use byteorder::{LE, ReadBytesExt, WriteBytesExt};
use thiserror::Error;

use crate::ident::{Ident, Identifier};

pub struct Context {
    strings: Vec<String>,
    identifiers: Vec<Identifier>,
    content: Vec<u8>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            strings: vec![],
            identifiers: vec![],
            content: vec![],
        }
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

    pub fn put_identifier(&mut self, id: &Identifier) -> usize {
        match self.identifiers.binary_search(id) {
            Ok(idx) => idx,
            Err(idx) => {
                self.identifiers.insert(idx, id.clone());
                idx
            }
        }
    }

    pub fn write_to<W: Write>(&self, mut pipe: W) -> Result<()> {
        for e in self.strings.iter() {
            write_str(&e, &mut pipe)?;
        }
        pipe.write(&self.content)?;
        Ok(())
    }
}

impl Write for Context {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.content.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
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

pub fn write_identifier<W: Write>(id: &Ident, mut pipe: W) -> Result<usize> {
    let is_minecraft_namespace = id.namespace() == "minecraft";
    let namespace_len = if is_minecraft_namespace {
        u16::MAX
    } else {
        let namespace_len = id.namespace().len().try_into()?;
        assert!(namespace_len < u16::MAX);
        namespace_len
    };
    let path_len = id.path().len().try_into()?;
    pipe.write_u16::<LE>(namespace_len)?;
    pipe.write_u16::<LE>(path_len)?;
    if !is_minecraft_namespace {
        pipe.write(id.namespace().as_bytes())?;
    }
    pipe.write(id.path().as_bytes())?;
    Ok(4 + if is_minecraft_namespace { 0 } else { id.namespace().len() } + id.path().len())
}

pub fn read_identifier<R: Read>(mut pipe: R) -> Result<Identifier> {
    let namespace_len = pipe.read_u16::<LE>()?;
    let path_len = pipe.read_u16::<LE>()?;
    let namespace = if namespace_len == u16::MAX {
        "minecraft".to_string()
    } else {
        let mut buf = vec![0; namespace_len as usize];
        pipe.read_exact(&mut buf)?;
        String::from_utf8(buf)?
    };
    let path = {
        let mut buf = vec![0; path_len as usize];
        pipe.read_exact(&mut buf)?;
        String::from_utf8(buf)?
    };
    Ok(Identifier::from_components(&namespace, &path))
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
}