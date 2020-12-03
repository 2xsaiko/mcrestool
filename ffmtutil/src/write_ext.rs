use std::convert::TryInto;
use std::io::{Read, Write};

use byteorder::{ReadBytesExt, LE};

use crate::varint::{decode_min, encode_min, varint_read, varint_write};
use crate::Result;

pub trait WriteExt {
    fn write_str(&mut self, s: &str) -> Result<usize>;

    fn write_varuint(&mut self, i: u64) -> Result<usize>;

    fn write_varint(&mut self, i: i64) -> Result<usize>;

    fn write_varusize(&mut self, i: usize) -> Result<usize>;
}

impl<W: Write> WriteExt for W {
    fn write_str(&mut self, s: &str) -> Result<usize> {
        self.write_varuint(s.len().try_into()?)?;
        self.write(s.as_bytes())?;
        Ok(2 + s.len())
    }

    fn write_varuint(&mut self, i: u64) -> Result<usize> {
        Ok(varint_write(i, self)?)
    }

    fn write_varint(&mut self, i: i64) -> Result<usize> {
        self.write_varuint(encode_min(i))
    }

    fn write_varusize(&mut self, i: usize) -> Result<usize> {
        self.write_varuint(i as u64)
    }
}

pub trait ReadExt {
    fn read_str(&mut self) -> Result<String>;

    fn read_varuint(&mut self) -> Result<u64>;

    fn read_varint(&mut self) -> Result<i64>;

    fn read_varusize(&mut self) -> Result<usize>;
}

impl<R: Read> ReadExt for R {
    fn read_str(&mut self) -> Result<String> {
        let len = self.read_u16::<LE>()?;
        let mut buf = vec![0; len as usize];
        self.read_exact(&mut buf)?;
        Ok(String::from_utf8(buf)?)
    }

    fn read_varuint(&mut self) -> Result<u64> {
        Ok(varint_read(self)?)
    }

    fn read_varint(&mut self) -> Result<i64> {
        Ok(decode_min(self.read_varuint()?))
    }

    fn read_varusize(&mut self) -> Result<usize> {
        Ok(self.read_varuint()?.try_into()?)
    }
}
