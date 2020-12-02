#![allow(incomplete_features)]
#![feature(const_generics)]

use std::fmt::Display;
use std::io;
use std::io::{Cursor, Read, Write};
use std::num::TryFromIntError;
use std::string::FromUtf8Error;

use thiserror::Error;

use de::BinDeserializeOwned;
use dedup::DedupContext;
pub use ffmtutil_derive::member_to_ident;
use ser::BinSerialize;
use serde::Mode;

use crate::serde::{BinSerializer, BinSerializerBase};
use crate::de::BinDeserializerBase;

pub mod de;
pub mod dedup;
mod mac;
pub mod ser;
pub mod serde;
mod serdeimpl;
pub mod try_iter;
mod varint;
mod write_ext;

pub fn serialize<W, T>(mut pipe: W, value: &T, mode: &Mode) -> Result<()>
where
    W: Write,
    T: BinSerialize,
{
    let mut buf = Cursor::new(Vec::new());
    let mut serializer = BinSerializerBase::new(&mut buf);
    value.serialize(&mut serializer)?;
    serializer.dedup().write_to(&mut pipe)?;
    pipe.write_all(buf.get_ref())?;
    Ok(())
}

pub fn deserialize<R, T>(mut pipe: R, mode: &Mode) -> Result<T>
where
    R: Read,
    T: BinDeserializeOwned,
{
    let context = DedupContext::read_from(&mut pipe)?;
    let mut deserializer = BinDeserializerBase::new(pipe, &context);
    T::deserialize(deserializer)
}

pub fn deserialize_in_place<R, T>(target: &mut T, mut pipe: R, mode: &Mode) -> Result<()>
where
    R: Read,
    T: BinDeserializeOwned,
{
    let context = DedupContext::read_from(&mut pipe)?;
    let mut deserializer = BinDeserializerBase::new(pipe, &context);
    target.deserialize_in_place(deserializer)
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
    #[error("{0}")]
    Custom(String),
}

impl Error {
    pub fn custom<S: Display>(s: S) -> Self {
        Error::Custom(s.to_string())
    }
}
