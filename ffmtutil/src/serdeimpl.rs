use std::collections::{HashMap, HashSet};
use std::convert::{Infallible, TryInto};
use std::hash::Hash;
use std::io::{Read, Write};
use std::marker::PhantomData;
use std::mem::MaybeUninit;

use byteorder::{ReadBytesExt, WriteBytesExt, LE};

use crate::dedup::DedupContext;
use crate::serde::{BinDeserialize, BinSerialize};
use crate::try_iter::try_iter;
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

impl<T, M> BinSerialize for &T
where
    T: BinSerialize<Mode = M>,
{
    type Mode = M;

    fn serialize<W: Write>(
        &self,
        mut pipe: W,
        dedup: &mut DedupContext,
        mode: &Self::Mode,
    ) -> Result<()> {
        (*self).serialize(pipe, dedup, mode)
    }
}

impl<'de> BinDeserialize<'de> for usize {
    type Mode = Mode;

    fn deserialize<R: Read>(
        mut pipe: R,
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
        mut pipe: W,
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
        mut pipe: R,
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
        mut pipe: W,
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
                mut pipe: R,
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
                mut pipe: W,
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
        mut pipe: R,
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
        mut pipe: W,
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
        mut pipe: W,
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

pub struct VecLikeIter<'de, R, T, M> {
    pipe: R,
    remaining: usize,
    dedup: &'de DedupContext,
    mode: M,
    marker: PhantomData<T>,
}

impl<'de, R, T> VecLikeIter<'de, R, T, Mode>
where
    R: Read,
    T: BinDeserialize<'de>,
{
    pub fn new(mut pipe: R, dedup: &'de DedupContext, mode: Mode) -> Result<Self> {
        let len = usize::deserialize(&mut pipe, dedup, &mode)?;
        Ok(VecLikeIter {
            pipe,
            remaining: len,
            dedup,
            mode,
            marker: Default::default(),
        })
    }
}

impl<'de, R, T, M> Iterator for VecLikeIter<'de, R, T, M>
where
    R: Read,
    T: BinDeserialize<'de, Mode = M>,
{
    type Item = Result<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining > 0 {
            self.remaining -= 1;
            Some(T::deserialize(&mut self.pipe, self.dedup, &self.mode))
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<'de, T> BinDeserialize<'de> for Vec<T>
where
    T: BinDeserialize<'de, Mode = Mode>,
{
    type Mode = Mode;

    fn deserialize<R: Read>(pipe: R, dedup: &'de DedupContext, mode: &Self::Mode) -> Result<Self> {
        let iter = VecLikeIter::new(pipe, dedup, *mode)?;
        try_iter(iter, |iter| iter.collect())
    }
}

impl<T> BinSerialize for Vec<T>
where
    T: BinSerialize<Mode = Mode>,
{
    type Mode = Mode;

    fn serialize<W: Write>(
        &self,
        pipe: W,
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
        mut pipe: W,
        dedup: &mut DedupContext,
        mode: &Self::Mode,
    ) -> Result<()> {
        self.len().serialize(&mut pipe, dedup, mode)?;

        for item in self.iter() {
            item.serialize(&mut pipe, dedup, mode)?;
        }

        Ok(())
    }
}

impl<T, M, const LEN: usize> BinSerialize for [T; LEN]
where
    T: BinSerialize<Mode = M>,
{
    type Mode = M;

    fn serialize<W: Write>(
        &self,
        mut pipe: W,
        dedup: &mut DedupContext,
        mode: &Self::Mode,
    ) -> Result<()> {
        for el in self.iter() {
            el.serialize(&mut pipe, dedup, mode)?;
        }

        Ok(())
    }
}

impl<'de, T, M, const LEN: usize> BinDeserialize<'de> for [T; LEN]
where
    T: BinDeserialize<'de, Mode = M> + Sized,
{
    type Mode = M;

    fn deserialize<R: Read>(
        mut pipe: R,
        dedup: &'de DedupContext,
        mode: &Self::Mode,
    ) -> Result<Self> {
        // this is safe since MaybeUninit<T>'s Drop is a no-op
        // TODO: https://github.com/rust-lang/rust/issues/61956
        let mut arr: [MaybeUninit<T>; LEN] =
            unsafe { std::mem::transmute_copy(&MaybeUninit::<T>::uninit()) };

        for idx in 0..LEN {
            arr[idx] = MaybeUninit::new(T::deserialize(&mut pipe, dedup, mode)?);
        }

        // this is safe since [MaybeUninit<T>; LEN] doesn't do anything on drop,
        // since MaybeUninit<T>'s Drop is a no-op
        Ok(unsafe { std::mem::transmute_copy(&arr) })
    }
}

impl<K, V> BinSerialize for HashMap<K, V>
where
    K: BinSerialize<Mode = Mode>,
    V: BinSerialize<Mode = Mode>,
{
    type Mode = Mode;

    fn serialize<W: Write>(
        &self,
        mut pipe: W,
        dedup: &mut DedupContext,
        mode: &Self::Mode,
    ) -> Result<()> {
        self.len().serialize(&mut pipe, dedup, mode)?;

        for el in self.iter() {
            el.serialize(&mut pipe, dedup, mode)?;
        }

        Ok(())
    }
}

impl<'de, K, V> BinDeserialize<'de> for HashMap<K, V>
where
    K: BinDeserialize<'de, Mode = Mode> + Eq + Hash,
    V: BinDeserialize<'de, Mode = Mode>,
{
    type Mode = Mode;

    fn deserialize<R: Read>(
        mut pipe: R,
        dedup: &'de DedupContext,
        mode: &Self::Mode,
    ) -> Result<Self> {
        let iter = VecLikeIter::new(pipe, dedup, *mode)?;
        try_iter(iter, |iter| iter.collect())
    }
}

impl<T> BinSerialize for HashSet<T>
where
    T: BinSerialize<Mode = Mode>,
{
    type Mode = Mode;

    fn serialize<W: Write>(
        &self,
        mut pipe: W,
        dedup: &mut DedupContext,
        mode: &Self::Mode,
    ) -> Result<()> {
        self.len().serialize(&mut pipe, dedup, mode)?;

        for el in self.iter() {
            el.serialize(&mut pipe, dedup, mode)?;
        }

        Ok(())
    }
}

impl<'de, T> BinDeserialize<'de> for HashSet<T>
where
    T: BinDeserialize<'de, Mode = Mode> + Hash + Eq,
{
    type Mode = Mode;

    fn deserialize<R: Read>(pipe: R, dedup: &'de DedupContext, mode: &Self::Mode) -> Result<Self> {
        let iter = VecLikeIter::new(pipe, dedup, *mode)?;
        try_iter(iter, |iter| iter.collect())
    }
}

macro_rules! impl_tuple {
    ($($tp:ident)+) => {
        impl<$($tp),+, Mode> BinSerialize for ($($tp),+)
        where
            $($tp: BinSerialize<Mode = Mode>),+
        {
            type Mode = Mode;

            #[allow(non_snake_case)]
            fn serialize<W: Write>(
                &self,
                mut pipe: W,
                dedup: &mut DedupContext,
                mode: &Self::Mode,
            ) -> Result<()> {
                let ($(ref $tp),+) = *self;
                $($tp.serialize(&mut pipe, dedup, mode)?;)+
                Ok(())
            }
        }

        impl<'de, $($tp),+, Mode> BinDeserialize<'de> for ($($tp),+)
        where
            $($tp: BinDeserialize<'de, Mode = Mode>),+
        {
            type Mode = Mode;

            fn deserialize<R: Read>(
                mut pipe: R,
                dedup: &'de DedupContext,
                mode: &Self::Mode,
            ) -> Result<Self> {
                Ok((
                    $($tp::deserialize(&mut pipe, dedup, mode)?),+
                ))
            }
        }
    }
}

impl_tuple! { A B }
impl_tuple! { A B C }
impl_tuple! { A B C D }
impl_tuple! { A B C D E }
impl_tuple! { A B C D E F }
impl_tuple! { A B C D E F G }
impl_tuple! { A B C D E F G H }
impl_tuple! { A B C D E F G H I }
impl_tuple! { A B C D E F G H I J }
impl_tuple! { A B C D E F G H I J K }
impl_tuple! { A B C D E F G H I J K L }
