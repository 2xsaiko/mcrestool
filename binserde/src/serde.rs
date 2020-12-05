#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Mode {
    pub usize_len: UsizeLen,
    pub dedup_idx: UsizeLen,
    pub fixed_size_use_varint: bool,

    // Do not flip this on if it's off
    pub use_dedup: bool,
}

impl Default for Mode {
    fn default() -> Self {
        Mode {
            usize_len: UsizeLen::Variable,
            dedup_idx: UsizeLen::Variable,
            fixed_size_use_varint: false,
            use_dedup: false,
        }
    }
}

impl Mode {
    pub fn dedup() -> Self {
        Mode {
            use_dedup: true,
            ..Default::default()
        }
    }

    pub fn with_usize_len(mut self, usize_len: UsizeLen) -> Self {
        self.usize_len = usize_len;
        self
    }

    pub fn with_fixed_size_use_varint(mut self, enabled: bool) -> Self {
        self.fixed_size_use_varint = enabled;
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
