use std::collections::{HashMap, HashSet};
use std::convert::{Infallible, TryInto};
use std::hash::Hash;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::path::{Path, PathBuf};

use byteorder::{ReadBytesExt, WriteBytesExt, LE};

use crate::serde::UsizeLen;
use crate::try_iter::try_iter;
use crate::write_ext::{ReadExt, WriteExt};
use crate::{BinDeserialize, BinDeserializer, BinSerialize, BinSerializer};
use crate::{Error, Result};

impl<T> BinSerialize for &T
where
    T: BinSerialize + ?Sized,
{
    fn serialize<S: BinSerializer>(&self, serializer: S) -> Result<()> {
        (*self).serialize(serializer)
    }
}

impl<'de> BinDeserialize<'de> for bool {
    fn deserialize<D: BinDeserializer<'de>>(mut deserializer: D) -> Result<Self> {
        let v = deserializer.pipe().read_u8()?;
        Ok(v != 0)
    }
}

impl BinSerialize for bool {
    fn serialize<S: BinSerializer>(&self, mut serializer: S) -> Result<()> {
        Ok(serializer
            .pipe()
            .write_u8(if *self { u8::MAX } else { u8::MIN })?)
    }
}

impl<'de> BinDeserialize<'de> for usize {
    fn deserialize<D: BinDeserializer<'de>>(mut deserializer: D) -> Result<Self> {
        match deserializer.mode().usize_len {
            UsizeLen::U8 => Ok(deserializer.pipe().read_u8()? as usize),
            UsizeLen::U16 => Ok(deserializer.pipe().read_u16::<LE>()? as usize),
            UsizeLen::U32 => Ok(deserializer.pipe().read_u32::<LE>()?.try_into()?),
            UsizeLen::U64 => Ok(deserializer.pipe().read_u64::<LE>()?.try_into()?),
            UsizeLen::Variable => deserializer.pipe().read_varusize(),
        }
    }
}

impl BinSerialize for usize {
    fn serialize<S: BinSerializer>(&self, mut serializer: S) -> Result<()> {
        match serializer.mode().usize_len {
            UsizeLen::U8 => serializer.pipe().write_u8((*self).try_into()?)?,
            UsizeLen::U16 => serializer.pipe().write_u8((*self).try_into()?)?,
            UsizeLen::U32 => serializer.pipe().write_u8((*self).try_into()?)?,
            UsizeLen::U64 => serializer.pipe().write_u8((*self).try_into()?)?,
            UsizeLen::Variable => {
                serializer.pipe().write_varusize(*self)?;
            }
        };

        Ok(())
    }
}

impl<'de> BinDeserialize<'de> for u8 {
    fn deserialize<D: BinDeserializer<'de>>(mut deserializer: D) -> Result<Self> {
        Ok(deserializer.pipe().read_u8()?)
    }
}

impl BinSerialize for u8 {
    fn serialize<S: BinSerializer>(&self, mut serializer: S) -> Result<()> {
        Ok(serializer.pipe().write_u8(*self)?)
    }
}

macro_rules! impl_int {
    ($type:ty, $rm:ident, $wm:ident, $rvm:ident, $wvm:ident, $varint_type:ty) => {
        impl<'de> BinDeserialize<'de> for $type {
            fn deserialize<D: BinDeserializer<'de>>(mut deserializer: D) -> Result<Self> {
                if deserializer.mode().fixed_size_use_varint {
                    Ok(deserializer.pipe().$rvm()?.try_into()?)
                } else {
                    Ok(deserializer.pipe().$rm::<LE>()?)
                }
            }
        }

        impl BinSerialize for $type {
            fn serialize<S: BinSerializer>(&self, mut serializer: S) -> Result<()> {
                if serializer.mode().fixed_size_use_varint {
                    serializer.pipe().$wvm(*self as $varint_type)?;
                } else {
                    serializer.pipe().$wm::<LE>(*self)?;
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
    fn deserialize<D: BinDeserializer<'de>>(mut deserializer: D) -> Result<Self> {
        if deserializer.mode().use_dedup {
            let idx = usize::deserialize(
                (&mut deserializer).change_mode(|mode| mode.usize_len = mode.dedup_idx),
            )?;
            deserializer
                .dedup()
                .get_str(idx)
                .map(|s| s.to_string())
                .ok_or_else(|| Error::custom(format!("index {} not in string table", idx)))
        } else {
            Ok(String::from_utf8(Vec::deserialize(deserializer)?)?)
        }
    }
}

impl BinSerialize for String {
    fn serialize<S: BinSerializer>(&self, serializer: S) -> Result<()> {
        (**self).serialize(serializer)
    }
}

impl BinSerialize for str {
    fn serialize<S: BinSerializer>(&self, mut serializer: S) -> Result<()> {
        if serializer.mode().use_dedup {
            let pos = serializer.dedup().put_str(self);
            pos.serialize(serializer.change_mode(|mode| mode.usize_len = mode.dedup_idx))
        } else {
            self.as_bytes().serialize(serializer)
        }
    }
}

pub struct VecLikeIter<D, T> {
    deserializer: D,
    remaining: usize,
    marker: PhantomData<T>,
}

impl<'de, D, T> VecLikeIter<D, T>
where
    D: BinDeserializer<'de>,
    T: BinDeserialize<'de>,
{
    pub fn new(mut deserializer: D) -> Result<Self> {
        let len = usize::deserialize(&mut deserializer)?;
        Ok(VecLikeIter {
            deserializer,
            remaining: len,
            marker: Default::default(),
        })
    }
}

impl<'de, D, T> Iterator for VecLikeIter<D, T>
where
    D: BinDeserializer<'de>,
    T: BinDeserialize<'de>,
{
    type Item = Result<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining > 0 {
            self.remaining -= 1;
            Some(T::deserialize(&mut self.deserializer))
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<'de, D, T> ExactSizeIterator for VecLikeIter<D, T>
where
    D: BinDeserializer<'de>,
    T: BinDeserialize<'de>,
{
}

pub fn serialize_iter<I, S>(mut iter: I, mut serializer: S) -> Result<()>
where
    I: Iterator,
    I::Item: BinSerialize,
    S: BinSerializer,
{
    match iter.size_hint() {
        (min, Some(max)) if min == max => {
            // we know the exact length of the iterator so we don't need to
            // collect it first before writing to the stream
            min.serialize(&mut serializer)?;

            for _ in 0..min {
                iter.next()
                    .expect("iterator returned less elements than it said it would!")
                    .serialize(&mut serializer)?;
            }

            Ok(())
        }
        _ => {
            let items: Vec<_> = iter.collect();
            items.len().serialize(&mut serializer)?;

            for item in items {
                item.serialize(&mut serializer)?;
            }

            Ok(())
        }
    }
}

impl<'de, T> BinDeserialize<'de> for Vec<T>
where
    T: BinDeserialize<'de>,
{
    fn deserialize<D: BinDeserializer<'de>>(deserializer: D) -> Result<Self> {
        let iter = VecLikeIter::new(deserializer)?;
        try_iter(iter, |iter| iter.collect())
    }

    fn deserialize_in_place<D: BinDeserializer<'de>>(&mut self, deserializer: D) -> Result<()> {
        self.clear();
        let iter = VecLikeIter::new(deserializer)?;
        try_iter(iter, |iter| self.extend(iter))
    }
}

impl<T> BinSerialize for Vec<T>
where
    T: BinSerialize,
{
    fn serialize<S: BinSerializer>(&self, serializer: S) -> Result<()> {
        (**self).serialize(serializer)
    }
}

impl<T> BinSerialize for [T]
where
    T: BinSerialize,
{
    fn serialize<S: BinSerializer>(&self, serializer: S) -> Result<()> {
        serialize_iter(self.iter(), serializer)
    }
}

impl<T, const LEN: usize> BinSerialize for [T; LEN]
where
    T: BinSerialize,
{
    fn serialize<S: BinSerializer>(&self, mut serializer: S) -> Result<()> {
        for el in self.iter() {
            el.serialize(&mut serializer)?;
        }

        Ok(())
    }
}

impl<'de, T, const LEN: usize> BinDeserialize<'de> for [T; LEN]
where
    T: BinDeserialize<'de> + Sized,
{
    fn deserialize<D: BinDeserializer<'de>>(mut deserializer: D) -> Result<Self> {
        // this is safe since MaybeUninit<T>'s Drop is a no-op
        // TODO: https://github.com/rust-lang/rust/issues/61956
        let mut arr: [MaybeUninit<T>; LEN] =
            unsafe { std::mem::transmute_copy(&MaybeUninit::<T>::uninit()) };

        for idx in 0..LEN {
            arr[idx] = MaybeUninit::new(T::deserialize(&mut deserializer)?);
        }

        // this is safe since [MaybeUninit<T>; LEN] doesn't do anything on drop,
        // since MaybeUninit<T>'s Drop is a no-op
        Ok(unsafe { std::mem::transmute_copy(&arr) })
    }

    fn deserialize_in_place<D: BinDeserializer<'de>>(&mut self, mut deserializer: D) -> Result<()> {
        for idx in 0..LEN {
            self[idx] = T::deserialize(&mut deserializer)?;
        }

        Ok(())
    }
}

impl<K, V> BinSerialize for HashMap<K, V>
where
    K: BinSerialize,
    V: BinSerialize,
{
    fn serialize<S: BinSerializer>(&self, serializer: S) -> Result<()> {
        serialize_iter(self.iter(), serializer)
    }
}

impl<'de, K, V> BinDeserialize<'de> for HashMap<K, V>
where
    K: BinDeserialize<'de> + Eq + Hash,
    V: BinDeserialize<'de>,
{
    fn deserialize<D: BinDeserializer<'de>>(deserializer: D) -> Result<Self> {
        let iter = VecLikeIter::new(deserializer)?;
        try_iter(iter, |iter| iter.collect())
    }

    fn deserialize_in_place<D: BinDeserializer<'de>>(&mut self, deserializer: D) -> Result<()> {
        self.clear();
        let iter = VecLikeIter::new(deserializer)?;
        try_iter(iter, |iter| self.extend(iter))
    }
}

impl<T> BinSerialize for HashSet<T>
where
    T: BinSerialize,
{
    fn serialize<S: BinSerializer>(&self, serializer: S) -> Result<()> {
        serialize_iter(self.iter(), serializer)
    }
}

impl<'de, T> BinDeserialize<'de> for HashSet<T>
where
    T: BinDeserialize<'de> + Hash + Eq,
{
    fn deserialize<D: BinDeserializer<'de>>(deserializer: D) -> Result<Self> {
        let iter = VecLikeIter::new(deserializer)?;
        try_iter(iter, |iter| iter.collect())
    }

    fn deserialize_in_place<D: BinDeserializer<'de>>(&mut self, deserializer: D) -> Result<()> {
        self.clear();
        let iter = VecLikeIter::new(deserializer)?;
        try_iter(iter, |iter| self.extend(iter))
    }
}

impl BinSerialize for () {
    fn serialize<S: BinSerializer>(&self, _serializer: S) -> Result<(), Error> {
        Ok(())
    }
}

impl<'de> BinDeserialize<'de> for () {
    fn deserialize<D: BinDeserializer<'de>>(_deserializer: D) -> Result<Self, Error> {
        Ok(())
    }

    fn deserialize_in_place<D: BinDeserializer<'de>>(
        &mut self,
        _deserializer: D,
    ) -> Result<(), Error> {
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
            fn serialize<S: BinSerializer>(&self, mut serializer: S) -> Result<()> {
                let ($(ref $tp),+) = *self;
                $($tp.serialize(&mut serializer)?;)+
                Ok(())
            }
        }

        impl<'de, $($tp),+> BinDeserialize<'de> for ($($tp),+)
        where
            $($tp: BinDeserialize<'de>),+
        {
            fn deserialize<De: BinDeserializer<'de>>(
                mut deserializer: De
            ) -> Result<Self> {
                Ok((
                    $($tp::deserialize(&mut deserializer)?),+
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
    fn serialize<S: BinSerializer>(&self, serializer: S) -> Result<()> {
        match self.to_str() {
            None => Err(Error::custom(
                "invalid characters for UTF-8 conversion in string",
            )),
            Some(s) => s.serialize(serializer),
        }
    }
}

impl BinSerialize for PathBuf {
    fn serialize<S: BinSerializer>(&self, serializer: S) -> Result<()> {
        self.as_path().serialize(serializer)
    }
}

impl<'de> BinDeserialize<'de> for PathBuf {
    fn deserialize<D: BinDeserializer<'de>>(deserializer: D) -> Result<Self> {
        Ok(PathBuf::from(String::deserialize(deserializer)?))
    }
}

impl<T> BinSerialize for Option<T>
where
    T: BinSerialize,
{
    fn serialize<S: BinSerializer>(&self, mut serializer: S) -> Result<()> {
        match self {
            None => 0u8.serialize(&mut serializer),
            Some(v) => {
                1u8.serialize(&mut serializer)?;
                v.serialize(&mut serializer)
            }
        }
    }
}

impl<'de, T> BinDeserialize<'de> for Option<T>
where
    T: BinDeserialize<'de>,
{
    fn deserialize<D: BinDeserializer<'de>>(mut deserializer: D) -> Result<Self> {
        let variant = u8::deserialize(&mut deserializer)?;
        Ok(match variant {
            0 => None,
            1 => Some(T::deserialize(deserializer)?),
            x @ _ => Err(Error::custom(format!("invalid enum variant index {}", x)))?,
        })
    }
}

impl<T, R> BinSerialize for Result<T, R>
where
    T: BinSerialize,
    R: BinSerialize,
{
    fn serialize<S: BinSerializer>(&self, mut serializer: S) -> Result<()> {
        match self {
            Ok(v) => {
                0u8.serialize(&mut serializer)?;
                v.serialize(&mut serializer)
            }
            Err(v) => {
                1u8.serialize(&mut serializer)?;
                v.serialize(&mut serializer)
            }
        }
    }
}

impl<'de, T, R> BinDeserialize<'de> for Result<T, R>
where
    T: BinDeserialize<'de>,
    R: BinDeserialize<'de>,
{
    fn deserialize<D: BinDeserializer<'de>>(mut deserializer: D) -> Result<Self> {
        let variant = u8::deserialize(&mut deserializer)?;
        Ok(match variant {
            0 => Ok(T::deserialize(deserializer)?),
            1 => Err(R::deserialize(deserializer)?),
            x @ _ => Err(Error::custom(format!("invalid enum variant index {}", x)))?,
        })
    }
}
