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
