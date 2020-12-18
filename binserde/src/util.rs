use std::marker::PhantomData;

use crate::{BinDeserialize, BinDeserializer, BinSerialize, BinSerializer, Result};

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
    // TODO restore immediate serialization when we can specialize for
    //      std::iter::TrustedLen
    let items: Vec<_> = iter.collect();
    items.len().serialize(&mut serializer)?;

    for item in items {
        item.serialize(&mut serializer)?;
    }

    Ok(())
}
