#![allow(incomplete_features)]
#![feature(const_generics)]

use std::io;
use std::io::{Read, Write};
use std::num::TryFromIntError;
use std::string::FromUtf8Error;

use thiserror::Error;

use crate::dedup::DedupContext;
use crate::serde::{BinDeserialize, BinSerialize};

mod dedup;
pub mod serde;
mod serdeimpl;
pub mod try_iter;
mod varint;
mod write_ext;

pub fn serialize<W, T, M>(pipe: W, value: &T, mode: &M) -> Result<()>
where
    W: Write,
    T: BinSerialize<Mode = M>,
{
    // TODO write dedup context
    value.serialize(pipe, &mut DedupContext::new(), mode)
}

pub fn deserialize<R, T, M>(pipe: R, mode: &M) -> Result<T>
where
    R: Read,
    T: BinDeserialize<Mode = M>,
{
    // TODO read dedup context
    T::deserialize(pipe, &DedupContext::new(), mode)
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
