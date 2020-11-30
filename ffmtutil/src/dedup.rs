pub struct DedupContext {
    strings: Vec<(String, usize)>,
}

impl DedupContext {
    pub fn new() -> Self {
        DedupContext {
            strings: Vec::new(),
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
        self.strings.get(idx).map(|el| &*el.0)
    }
}
