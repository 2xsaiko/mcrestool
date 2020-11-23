use std::convert::TryInto;
use std::io::{Read, Write};
use std::io;
use std::num::TryFromIntError;
use std::string::FromUtf8Error;

use byteorder::{LE, ReadBytesExt, WriteBytesExt};
use thiserror::Error;

pub use dedup::{ReadContext, WriteContext};

mod dedup;

pub trait WriteExt {
    fn write_str(&mut self, s: &str) -> Result<usize>;
}

impl<W: Write> WriteExt for W {
    fn write_str(&mut self, s: &str) -> Result<usize, Error> {
        self.write_u16::<LE>(s.len().try_into()?)?;
        self.write(s.as_bytes())?;
        Ok(2 + s.len())
    }
}

pub trait ReadExt {
    fn read_str(&mut self) -> Result<String>;
}

impl<R: Read> ReadExt for R {
    fn read_str(&mut self) -> Result<String> {
        let len = self.read_u16::<LE>()?;
        let mut buf = vec![0; len as usize];
        self.read_exact(&mut buf)?;
        Ok(String::from_utf8(buf)?)
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

pub fn encode_min(num: i32) -> u32 {
    let u_num = num as u32;
    (u_num << 1 ^ (num >> 31) as u32) | u_num >> 31
}

pub fn unencode_min(num: u32) -> i32 {
    (num >> 1) as i32 ^ ((num << 31) as i32) >> 31
}

#[test]
fn test_encode_min() {
    for i in -5..5 {
        let r = encode_min(i);
        println!("{} = {:08X}", i, r);
        assert_eq!(i, unencode_min(r));
    }

    for i in i32::MAX - 5..=i32::MAX {
        let r = encode_min(i);
        println!("{} = {:08X}", i, r);
        assert_eq!(i, unencode_min(r));
    }

    for i in i32::MIN..i32::MIN + 5 {
        let r = encode_min(i);
        println!("{} = {:08X}", i, r);
        assert_eq!(i, unencode_min(r));
    }
}