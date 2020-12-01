use std::collections::{HashMap, HashSet};
use std::convert::{Infallible, TryInto};
use std::hash::Hash;
use std::io::{Read, Write};
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::path::{Path, PathBuf};

use byteorder::{ReadBytesExt, WriteBytesExt, LE};

use crate::dedup::DedupContext;
use crate::serde::{BinDeserialize, BinSerialize, Mode, UsizeLen};
use crate::try_iter::try_iter;
use crate::write_ext::{ReadExt, WriteExt};
use crate::{Error, Result};

impl<T> BinSerialize for &T
where
    T: BinSerialize,
{
    fn serialize<W: Write>(&self, pipe: W, dedup: &mut DedupContext, mode: &Mode) -> Result<()> {
        (*self).serialize(pipe, dedup, mode)
    }
}

impl<'de> BinDeserialize<'de> for bool {
    fn deserialize<R: Read>(mut pipe: R, _dedup: &'de DedupContext, _mode: &Mode) -> Result<Self> {
        let v = pipe.read_u8()?;
        Ok(v != 0)
    }
}

impl BinSerialize for bool {
    fn serialize<W: Write>(
        &self,
        mut pipe: W,
        _dedup: &mut DedupContext,
        _mode: &Mode,
    ) -> Result<()> {
        Ok(pipe.write_u8(if *self { u8::MAX } else { u8::MIN })?)
    }
}

impl<'de> BinDeserialize<'de> for usize {
    fn deserialize<R: Read>(mut pipe: R, _dedup: &'de DedupContext, mode: &Mode) -> Result<Self> {
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
    fn serialize<W: Write>(
        &self,
        mut pipe: W,
        _dedup: &mut DedupContext,
        mode: &Mode,
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
    fn deserialize<R: Read>(mut pipe: R, _dedup: &'de DedupContext, _mode: &Mode) -> Result<Self> {
        Ok(pipe.read_u8()?)
    }
}

impl BinSerialize for u8 {
    fn serialize<W: Write>(
        &self,
        mut pipe: W,
        _dedup: &mut DedupContext,
        _mode: &Mode,
    ) -> Result<()> {
        Ok(pipe.write_u8(*self)?)
    }
}

macro_rules! impl_int {
    ($type:ty, $rm:ident, $wm:ident, $rvm:ident, $wvm:ident, $varint_type:ty) => {
        impl<'de> BinDeserialize<'de> for $type {
            fn deserialize<R: Read>(
                mut pipe: R,
                _dedup: &'de DedupContext,
                mode: &Mode,
            ) -> Result<Self> {
                if mode.fixed_size_use_varint {
                    Ok(pipe.$rvm()?.try_into()?)
                } else {
                    Ok(pipe.$rm::<LE>()?)
                }
            }
        }

        impl BinSerialize for $type {
            fn serialize<W: Write>(
                &self,
                mut pipe: W,
                _dedup: &mut DedupContext,
                mode: &Mode,
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
    fn deserialize<R: Read>(pipe: R, dedup: &'de DedupContext, mode: &Mode) -> Result<Self> {
        Ok(String::from_utf8(Vec::deserialize(pipe, dedup, mode)?)?)
    }
}

impl BinSerialize for String {
    fn serialize<W: Write>(&self, pipe: W, dedup: &mut DedupContext, mode: &Mode) -> Result<()> {
        (**self).serialize(pipe, dedup, mode)
    }
}

impl BinSerialize for str {
    fn serialize<W: Write>(&self, pipe: W, dedup: &mut DedupContext, mode: &Mode) -> Result<()> {
        if mode.use_dedup {
            let pos = dedup.put_str(self);
            pos.serialize(pipe, dedup, &mode.with_usize_len(mode.dedup_idx))
        } else {
            self.as_bytes().serialize(pipe, dedup, mode)
        }
    }
}

pub struct VecLikeIter<'de, R, T> {
    pipe: R,
    remaining: usize,
    dedup: &'de DedupContext,
    mode: Mode,
    marker: PhantomData<T>,
}

impl<'de, R, T> VecLikeIter<'de, R, T>
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

impl<'de, R, T> Iterator for VecLikeIter<'de, R, T>
where
    R: Read,
    T: BinDeserialize<'de>,
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

pub fn serialize_iter<I, W>(
    mut iter: I,
    mut pipe: W,
    dedup: &mut DedupContext,
    mode: &Mode,
) -> Result<()>
where
    I: Iterator,
    I::Item: BinSerialize,
    W: Write,
{
    let (min, max) = iter.size_hint();
    if Some(min) == max {
        min.serialize(&mut pipe, dedup, mode)?;

        for _ in 0..min {
            iter.next()
                .expect("iterator returned less elements than it said it would!")
                .serialize(&mut pipe, dedup, mode)?;
        }

        Ok(())
    } else {
        let items: Vec<_> = iter.collect();
        items.serialize(pipe, dedup, mode)
    }
}

impl<'de, T> BinDeserialize<'de> for Vec<T>
where
    T: BinDeserialize<'de>,
{
    fn deserialize<R: Read>(pipe: R, dedup: &'de DedupContext, mode: &Mode) -> Result<Self> {
        let iter = VecLikeIter::new(pipe, dedup, *mode)?;
        try_iter(iter, |iter| iter.collect())
    }

    fn deserialize_in_place<R: Read>(
        &mut self,
        pipe: R,
        dedup: &'de DedupContext,
        mode: &Mode,
    ) -> Result<()> {
        self.clear();
        let iter = VecLikeIter::new(pipe, dedup, *mode)?;
        try_iter(iter, |iter| self.extend(iter))
    }
}

impl<T> BinSerialize for Vec<T>
where
    T: BinSerialize,
{
    fn serialize<W: Write>(&self, pipe: W, dedup: &mut DedupContext, mode: &Mode) -> Result<()> {
        (**self).serialize(pipe, dedup, mode)
    }
}

impl<T> BinSerialize for [T]
where
    T: BinSerialize,
{
    fn serialize<W: Write>(
        &self,
        mut pipe: W,
        dedup: &mut DedupContext,
        mode: &Mode,
    ) -> Result<()> {
        self.len().serialize(&mut pipe, dedup, mode)?;

        for item in self.iter() {
            item.serialize(&mut pipe, dedup, mode)?;
        }

        Ok(())
    }
}

impl<T, const LEN: usize> BinSerialize for [T; LEN]
where
    T: BinSerialize,
{
    fn serialize<W: Write>(
        &self,
        mut pipe: W,
        dedup: &mut DedupContext,
        mode: &Mode,
    ) -> Result<()> {
        for el in self.iter() {
            el.serialize(&mut pipe, dedup, mode)?;
        }

        Ok(())
    }
}

impl<'de, T, const LEN: usize> BinDeserialize<'de> for [T; LEN]
where
    T: BinDeserialize<'de> + Sized,
{
    fn deserialize<R: Read>(mut pipe: R, dedup: &'de DedupContext, mode: &Mode) -> Result<Self> {
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

    fn deserialize_in_place<R: Read>(
        &mut self,
        mut pipe: R,
        dedup: &'de DedupContext,
        mode: &Mode,
    ) -> Result<()> {
        for idx in 0..LEN {
            self[idx] = T::deserialize(&mut pipe, dedup, mode)?;
        }

        Ok(())
    }
}

impl<K, V> BinSerialize for HashMap<K, V>
where
    K: BinSerialize,
    V: BinSerialize,
{
    fn serialize<W: Write>(
        &self,
        mut pipe: W,
        dedup: &mut DedupContext,
        mode: &Mode,
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
    K: BinDeserialize<'de> + Eq + Hash,
    V: BinDeserialize<'de>,
{
    fn deserialize<R: Read>(pipe: R, dedup: &'de DedupContext, mode: &Mode) -> Result<Self> {
        let iter = VecLikeIter::new(pipe, dedup, *mode)?;
        try_iter(iter, |iter| iter.collect())
    }

    fn deserialize_in_place<R: Read>(
        &mut self,
        pipe: R,
        dedup: &'de DedupContext,
        mode: &Mode,
    ) -> Result<()> {
        self.clear();
        let iter = VecLikeIter::new(pipe, dedup, *mode)?;
        try_iter(iter, |iter| self.extend(iter))
    }
}

impl<T> BinSerialize for HashSet<T>
where
    T: BinSerialize,
{
    fn serialize<W: Write>(
        &self,
        mut pipe: W,
        dedup: &mut DedupContext,
        mode: &Mode,
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
    T: BinDeserialize<'de> + Hash + Eq,
{
    fn deserialize<R: Read>(pipe: R, dedup: &'de DedupContext, mode: &Mode) -> Result<Self> {
        let iter = VecLikeIter::new(pipe, dedup, *mode)?;
        try_iter(iter, |iter| iter.collect())
    }

    fn deserialize_in_place<R: Read>(
        &mut self,
        pipe: R,
        dedup: &'de DedupContext,
        mode: &Mode,
    ) -> Result<()> {
        self.clear();
        let iter = VecLikeIter::new(pipe, dedup, *mode)?;
        try_iter(iter, |iter| self.extend(iter))
    }
}

impl BinSerialize for () {
    fn serialize<W: Write>(&self, _pipe: W, _dedup: &mut DedupContext, _mode: &Mode) -> Result<(), Error> {
        Ok(())
    }
}

impl<'de> BinDeserialize<'de> for () {
    fn deserialize<R: Read>(_pipe: R, _dedup: &'de DedupContext, _mode: &Mode) -> Result<Self, Error> {
        Ok(())
    }

    fn deserialize_in_place<R: Read>(&mut self, _pipe: R, _dedup: &'de DedupContext, _mode: &Mode) -> Result<(), Error> {
        Ok(())
    }
}

macro_rules! impl_tuple {
    ($($tp:ident)+) => {
        impl<$($tp),+> BinSerialize for ($($tp),+)
        where
            $($tp: BinSerialize),+
        {
            #[allow(non_snake_case)]
            fn serialize<W: Write>(
                &self,
                mut pipe: W,
                dedup: &mut DedupContext,
                mode: &Mode,
            ) -> Result<()> {
                let ($(ref $tp),+) = *self;
                $($tp.serialize(&mut pipe, dedup, mode)?;)+
                Ok(())
            }
        }

        impl<'de, $($tp),+> BinDeserialize<'de> for ($($tp),+)
        where
            $($tp: BinDeserialize<'de>),+
        {
            fn deserialize<R: Read>(
                mut pipe: R,
                dedup: &'de DedupContext,
                mode: &Mode,
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

impl BinSerialize for Path {
    fn serialize<W: Write>(&self, pipe: W, dedup: &mut DedupContext, mode: &Mode) -> Result<()> {
        match self.to_str() {
            None => Err(Error::custom(
                "invalid characters for UTF-8 conversion in string",
            )),
            Some(s) => s.serialize(pipe, dedup, mode),
        }
    }
}

impl BinSerialize for PathBuf {
    fn serialize<W: Write>(&self, pipe: W, dedup: &mut DedupContext, mode: &Mode) -> Result<()> {
        self.as_path().serialize(pipe, dedup, mode)
    }
}

impl<'de> BinDeserialize<'de> for PathBuf {
    fn deserialize<R: Read>(pipe: R, dedup: &'de DedupContext, mode: &Mode) -> Result<Self> {
        Ok(PathBuf::from(String::deserialize(pipe, dedup, mode)?))
    }
}
