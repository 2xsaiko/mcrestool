use std::convert::{Infallible, TryInto};
use std::io::{Read, Write};
use std::mem::MaybeUninit;

use byteorder::{ReadBytesExt, WriteBytesExt, LE};

use crate::dedup::DedupContext;
use crate::serde::{BinDeserialize, BinSerialize};
use crate::{serde, ReadExt};
use crate::{Result, WriteExt};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Mode {
    pub usize_len: UsizeLen,
    pub dedup_idx: UsizeLen,
    pub fixed_size_use_varint: bool,
    pub use_dedup: bool,
}

impl Default for Mode {
    fn default() -> Self {
        Mode {
            usize_len: UsizeLen::Variable,
            dedup_idx: UsizeLen::Variable,
            fixed_size_use_varint: false,
            use_dedup: true,
        }
    }
}

impl Mode {
    pub fn with_usize_len(mut self, usize_len: UsizeLen) -> Self {
        self.usize_len = usize_len;
        self
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum UsizeLen {
    U8,
    U16,
    U32,
    U64,
    Variable,
}

impl<'de> BinDeserialize<'de> for usize {
    type Mode = Mode;

    fn deserialize<R: Read>(
        pipe: &mut R,
        _dedup: &'de DedupContext,
        mode: &Self::Mode,
    ) -> Result<Self> {
        match mode.usize_len {
            UsizeLen::U8 => Ok(pipe.read_u8()? as usize),
            UsizeLen::U16 => Ok(pipe.read_u16::<LE>()? as usize),
            UsizeLen::U32 => Ok(pipe.read_u32::<LE>()?.try_into()?),
            UsizeLen::U64 => Ok(pipe.read_u64::<LE>()?.try_into()?),
            UsizeLen::Variable => pipe.read_varusize(),
        }
    }
}

impl BinSerialize for usize {
    type Mode = Mode;

    fn serialize<W: Write>(
        &self,
        pipe: &mut W,
        _dedup: &mut DedupContext,
        mode: &Self::Mode,
    ) -> Result<()> {
        match mode.usize_len {
            UsizeLen::U8 => pipe.write_u8((*self).try_into()?)?,
            UsizeLen::U16 => pipe.write_u8((*self).try_into()?)?,
            UsizeLen::U32 => pipe.write_u8((*self).try_into()?)?,
            UsizeLen::U64 => pipe.write_u8((*self).try_into()?)?,
            UsizeLen::Variable => {
                pipe.write_varusize(*self)?;
            }
        };

        Ok(())
    }
}

impl<'de> BinDeserialize<'de> for u8 {
    type Mode = Mode;

    fn deserialize<R: Read>(
        pipe: &mut R,
        dedup: &'de DedupContext,
        mode: &Self::Mode,
    ) -> Result<Self> {
        Ok(pipe.read_u8()?)
    }
}

impl BinSerialize for u8 {
    type Mode = Mode;

    fn serialize<W: Write>(
        &self,
        pipe: &mut W,
        dedup: &mut DedupContext,
        mode: &Self::Mode,
    ) -> Result<()> {
        Ok(pipe.write_u8(*self)?)
    }
}

macro_rules! impl_int {
    ($type:ty, $rm:ident, $wm:ident, $rvm:ident, $wvm:ident, $varint_type:ty) => {
        impl<'de> BinDeserialize<'de> for $type {
            type Mode = Mode;

            fn deserialize<R: Read>(
                pipe: &mut R,
                _dedup: &'de DedupContext,
                mode: &Self::Mode,
            ) -> Result<Self> {
                if mode.fixed_size_use_varint {
                    Ok(pipe.$rvm()?.try_into()?)
                } else {
                    Ok(pipe.$rm::<LE>()?)
                }
            }
        }

        impl BinSerialize for $type {
            type Mode = Mode;

            fn serialize<W: Write>(
                &self,
                pipe: &mut W,
                _dedup: &mut DedupContext,
                mode: &Self::Mode,
            ) -> Result<()> {
                if mode.fixed_size_use_varint {
                    pipe.$wvm(*self as $varint_type)?;
                } else {
                    pipe.$wm::<LE>(*self)?;
                }

                Ok(())
            }
        }
    };
}

impl From<std::convert::Infallible> for crate::Error {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}

impl_int!(u16, read_u16, write_u16, read_varuint, write_varuint, u64);
impl_int!(u32, read_u32, write_u32, read_varuint, write_varuint, u64);
impl_int!(u64, read_u64, write_u64, read_varuint, write_varuint, u64);
impl_int!(i16, read_i16, write_i16, read_varint, write_varint, i64);
impl_int!(i32, read_i32, write_i32, read_varint, write_varint, i64);
impl_int!(i64, read_i64, write_i64, read_varint, write_varint, i64);

impl<'de> BinDeserialize<'de> for String {
    type Mode = Mode;

    fn deserialize<R: Read>(
        pipe: &mut R,
        dedup: &'de DedupContext,
        mode: &Self::Mode,
    ) -> Result<Self> {
        Ok(String::from_utf8(Vec::deserialize(pipe, dedup, mode)?)?)
    }
}

impl BinSerialize for String {
    type Mode = Mode;

    fn serialize<W: Write>(
        &self,
        pipe: &mut W,
        dedup: &mut DedupContext,
        mode: &Self::Mode,
    ) -> Result<()> {
        (**self).serialize(pipe, dedup, mode)
    }
}

impl BinSerialize for str {
    type Mode = Mode;

    fn serialize<W: Write>(
        &self,
        pipe: &mut W,
        dedup: &mut DedupContext,
        mode: &Self::Mode,
    ) -> Result<()> {
        if mode.use_dedup {
            let pos = dedup.put_str(self);
            pos.serialize(pipe, dedup, &mode.with_usize_len(mode.dedup_idx))
        } else {
            self.as_bytes().serialize(pipe, dedup, mode)
        }
    }
}

impl<'de, T> BinDeserialize<'de> for Vec<T>
where
    T: BinDeserialize<'de, Mode = Mode>,
{
    type Mode = Mode;

    fn deserialize<R: Read>(
        pipe: &mut R,
        dedup: &'de DedupContext,
        mode: &Self::Mode,
    ) -> Result<Self> {
        let len = usize::deserialize(pipe, dedup, mode)?;
        let mut vec = Vec::with_capacity(len);

        for _ in 0..len {
            let el = T::deserialize(pipe, dedup, mode)?;
            vec.push(el);
        }

        Ok(vec)
    }
}

impl<T> BinSerialize for Vec<T>
where
    T: BinSerialize<Mode = Mode>,
{
    type Mode = Mode;

    fn serialize<W: Write>(
        &self,
        pipe: &mut W,
        dedup: &mut DedupContext,
        mode: &Self::Mode,
    ) -> Result<()> {
        (**self).serialize(pipe, dedup, mode)
    }
}

impl<T> BinSerialize for [T]
where
    T: BinSerialize<Mode = Mode>,
{
    type Mode = Mode;

    fn serialize<W: Write>(
        &self,
        pipe: &mut W,
        dedup: &mut DedupContext,
        mode: &Self::Mode,
    ) -> Result<()> {
        self.len().serialize(pipe, dedup, mode)?;

        for item in self.iter() {
            item.serialize(pipe, dedup, mode)?;
        }

        Ok(())
    }
}

impl<const LEN: usize, T, M> BinSerialize for [T; LEN]
where
    T: BinSerialize<Mode = M>,
{
    type Mode = M;

    fn serialize<W: Write>(
        &self,
        pipe: &mut W,
        dedup: &mut DedupContext,
        mode: &Self::Mode,
    ) -> Result<()> {
        for el in self.iter() {
            el.serialize(pipe, dedup, mode)?;
        }

        Ok(())
    }
}

// TODO: https://github.com/rust-lang/rust/issues/61956

// impl<'de, const LEN: usize, T, M> BinDeserialize<'de> for [T; LEN]
// where
//     T: BinDeserialize<'de, Mode = M> + Sized,
// {
//     type Mode = M;
//
//     fn deserialize<R: Read>(
//         pipe: &mut R,
//         dedup: &'de DedupContext,
//         mode: &Self::Mode,
//     ) -> Result<Self> {
//         let mut arr: [MaybeUninit<T>; LEN] =
//             unsafe { std::mem::transmute(MaybeUninit::<T>::uninit()) };
//
//         for idx in 0..LEN {
//             arr[idx] = MaybeUninit::new(T::deserialize(pipe, dedup, mode)?);
//         }
//
//         Ok(unsafe { std::mem::transmute(arr) })
//     }
// }
