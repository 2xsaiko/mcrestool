use std::io::{Read, Write};

use crate::de::{BinDeserializer, BinDeserializerBase};
use crate::serde::UsizeLen;
use crate::serdeimpl::serialize_iter;
use crate::Result;
use crate::{BinDeserialize, BinSerializer, BinSerializerBase, Mode};

const DEDUP_MODE: Mode = Mode {
    usize_len: UsizeLen::Variable,
    dedup_idx: UsizeLen::Variable,
    fixed_size_use_varint: false,
    use_dedup: false,
};

pub struct DedupContext {
    strings: Vec<(String, usize)>,
    by_index: Vec<usize>,
}

impl DedupContext {
    pub fn new() -> Self {
        DedupContext {
            strings: Vec::new(),
            by_index: Vec::new(),
        }
    }

    pub fn put_str(&mut self, s: &str) -> usize {
        match self.strings.binary_search_by(|el| (*el.0).cmp(s)) {
            Ok(idx) => self.strings[idx].1,
            Err(idx) => {
                let l = self.strings.len();
                self.strings.insert(idx, (s.to_string(), l));
                l
            }
        }
    }

    pub fn get_str(&self, idx: usize) -> Option<&str> {
        self.by_index.get(idx).map(|el| &*self.strings[*el].0)
    }

    pub fn write_to<W: Write>(&self, pipe: W) -> Result<()> {
        let ser = BinSerializerBase::new(pipe).with_mode(DEDUP_MODE);

        let mut by_index: Vec<_> = self.strings.iter().collect();
        by_index.sort_unstable_by_key(|el| el.1);
        serialize_iter(by_index.into_iter().map(|el| &el.0), ser)?;

        Ok(())
    }

    pub fn read_from<R: Read>(pipe: R) -> Result<Self> {
        let empty = DedupContext::new();
        let de = BinDeserializerBase::new(pipe, &empty).with_mode(DEDUP_MODE);

        let by_index: Vec<String> = Vec::deserialize(de)?;
        let mut strings: Vec<_> = by_index
            .into_iter()
            .enumerate()
            .map(|(idx, s)| (s, idx))
            .collect();
        strings.sort_unstable_by(|a, b| a.0.cmp(&b.0));
        let mut by_index = vec![0; strings.len()];
        for (idx, el) in strings.iter().enumerate() {
            by_index[el.1] = idx;
        }
        Ok(DedupContext { strings, by_index })
    }
}
