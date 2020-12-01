#![allow(incomplete_features)]
#![feature(const_generics)]

use std::io;
use std::io::{Read, Write};
use std::num::TryFromIntError;
use std::string::FromUtf8Error;

use thiserror::Error;

use dedup::DedupContext;
use serde::{BinDeserializeOwned, BinSerialize, Mode};

pub mod dedup;
pub mod serde;
mod serdeimpl;
pub mod try_iter;
mod varint;
mod write_ext;
mod mac;

pub fn serialize<W, T>(pipe: W, value: &T, mode: &Mode) -> Result<()>
where
    W: Write,
    T: BinSerialize,
{
    // TODO write dedup context
    value.serialize(pipe, &mut DedupContext::new(), mode)
}

pub fn deserialize<R, T>(pipe: R, mode: &Mode) -> Result<T>
where
    R: Read,
    T: BinDeserializeOwned,
{
    // TODO read dedup context
    let context = DedupContext::new();
    T::deserialize(pipe, &context, mode)
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
    Other(String),
}
