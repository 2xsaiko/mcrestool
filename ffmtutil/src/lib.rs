#![allow(incomplete_features)]
#![feature(const_generics)]
#![feature(trace_macros)]

use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::io;
use std::io::{Cursor, Read, Write};
use std::num::TryFromIntError;
use std::string::FromUtf8Error;

use thiserror::Error;

use de::BinDeserializeOwned;
pub use de::{BinDeserialize, BinDeserializer};
use dedup::DedupContext;
pub use ffmtutil_derive::{member_to_ident, BinSerialize};
pub use ser::{BinSerialize, BinSerializer};
pub use serde::Mode;

use crate::de::BinDeserializerBase;
use crate::ser::{BinSerializerBase, PrescanSerializer};

pub mod de;
pub mod dedup;
mod mac;
pub mod ser;
pub mod serde;
mod serdeimpl;
pub mod try_iter;
mod varint;
mod write_ext;

pub fn serialize<T>(value: &T) -> Result<Vec<u8>>
where
    T: BinSerialize + ?Sized,
{
    serialize_with(value, Mode::default())
}

pub fn serialize_into<W, T>(pipe: W, value: &T) -> Result<()>
where
    W: Write,
    T: BinSerialize + ?Sized,
{
    serialize_with_into(pipe, value, Mode::default())
}

pub fn serialize_with<T>(value: &T, mode: Mode) -> Result<Vec<u8>>
where
    T: BinSerialize + ?Sized,
{
    let mut buf = Cursor::new(Vec::new());
    serialize_with_into(&mut buf, value, mode)?;
    Ok(buf.into_inner())
}

pub fn serialize_with_into<W, T>(mut pipe: W, value: &T, mode: Mode) -> Result<()>
where
    W: Write,
    T: BinSerialize + ?Sized,
{
    if mode.use_dedup {
        let mut ps = PrescanSerializer::new().with_mode(mode);
        value.serialize(&mut ps)?;
        ps.dedup().write_to(&mut pipe)?;
    }
    let mut serializer = BinSerializerBase::new(pipe).with_mode(mode);
    value.serialize(&mut serializer)?;
    Ok(())
}

pub fn deserialize<T>(buf: &[u8]) -> Result<T>
where
    T: BinDeserializeOwned,
{
    deserialize_with(buf, Mode::default())
}

pub fn deserialize_with<T>(buf: &[u8], mode: Mode) -> Result<T>
where
    T: BinDeserializeOwned,
{
    deserialize_with_from(Cursor::new(buf), mode)
}

pub fn deserialize_from<R, T>(pipe: R) -> Result<T>
where
    R: Read,
    T: BinDeserializeOwned,
{
    deserialize_with_from(pipe, Mode::default())
}

pub fn deserialize_with_from<R, T>(mut pipe: R, mode: Mode) -> Result<T>
where
    R: Read,
    T: BinDeserializeOwned,
{
    let context = if mode.use_dedup {
        DedupContext::read_from(&mut pipe)?
    } else {
        DedupContext::new()
    };
    let deserializer = BinDeserializerBase::new(pipe, &context).with_mode(mode);
    T::deserialize(deserializer)
}

pub fn deserialize_in_place<R, T>(target: &mut T, mut pipe: R, mode: Mode) -> Result<()>
where
    R: Read,
    T: BinDeserializeOwned,
{
    let context = if mode.use_dedup {
        DedupContext::read_from(&mut pipe)?
    } else {
        DedupContext::new()
    };
    let deserializer = BinDeserializerBase::new(pipe, &context).with_mode(mode);
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

#[test]
fn serialize_inline_test() {
    #[derive(Debug, PartialEq, Eq)]
    struct Test {
        vec: Vec<Test1>,
        map_set: HashMap<String, HashSet<String>>,
    }

    #[derive(Debug, PartialEq, Eq)]
    struct Test1(String, i32);

    impl_serde_wrap! {
        struct Test { vec, map_set }
        struct Test1(0, 1);
    }

    let s = Test {
        vec: vec![
            Test1("yyyyyyyyyyyyyyyyyy".to_string(), 4),
            Test1("a".to_string(), 4),
            Test1("yyyyyyyyyyyyyyyyyy".to_string(), 4),
            Test1("ab".to_string(), 4),
            Test1("abc".to_string(), 4),
            Test1("abcd".to_string(), 4),
        ],
        map_set: vec![("a", vec!["a", "b", "c"]), ("a1", vec!["a1", "b1", "c1"])]
            .into_iter()
            .map(|el| {
                (
                    el.0.to_string(),
                    el.1.into_iter().map(|el| el.to_string()).collect(),
                )
            })
            .collect(),
    };

    {
        let mode = Mode::dedup();

        let buf = serialize_with(&s, mode).expect("failed to serialize");
        println!("{:02X?}", buf);

        let s1: Test = deserialize_with(&buf, mode).expect("failed to deserialize");

        assert_eq!(s, s1);
    }

    {
        let buf = serialize(&s).expect("failed to serialize");
        println!("{:02X?}", buf);

        let s1: Test = deserialize(&buf).expect("failed to deserialize");

        assert_eq!(s, s1);
    }
}

#[test]
fn serialize_constant_output() {
    assert_eq!(&[3, 97, 98, 99], &*serialize(&"abc").unwrap());

    assert_eq!(&[0xFF], &*serialize(&true).unwrap());
    assert_eq!(&[0x00], &*serialize(&false).unwrap());
}
