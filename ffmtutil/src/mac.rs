#[macro_export]
macro_rules! impl_serde_wrap {
    ($($token:tt)*) => {
        $crate::impl_serialize_wrap!($($token)*);
        $crate::impl_deserialize_wrap!($($token)*);
    }
}

#[macro_export]
macro_rules! impl_serialize_wrap {
    (struct $target:ident { $($el:ident),* } $($more:tt)*) => {
        $crate::do_impl_serialize_wrap!(struct $target, $($el),*);
        $crate::impl_serialize_wrap!($($more)*);
    };

    (struct $target:ident($($el:ident),*); $($more:tt)*) => {
        $crate::do_impl_serialize_wrap!(tuplestruct $target, $($el),*);
        $crate::impl_serialize_wrap!($($more)*);
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! do_impl_serialize_wrap {
    (struct $target:ident, $($el:ident),+) => {
        impl $crate::serde::BinSerialize for $target {
            fn serialize<W: std::io::Write>(
                &self,
                mut pipe: W,
                dedup: &mut $crate::dedup::DedupContext,
                mode: &$crate::serde::Mode,
            ) -> $crate::Result<()> {
                $(self.$el.serialize(&mut pipe, dedup, mode)?;)+
                Ok(())
            }
        }
    }
}

#[macro_export]
macro_rules! impl_deserialize_wrap {
    (struct $target:ident { $($el:ident),* } $($more:tt)*) => {
        $crate::do_impl_deserialize_wrap!(struct $target, $($el),*);
        $crate::impl_deserialize_wrap!($($more)*);
    };

    (struct $target:ident($($el:ident),*); $($more:tt)*) => {
        $crate::do_impl_deserialize_wrap!(tuplestruct $target, $($el),*);
        $crate::impl_deserialize_wrap!($($more)*);
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! do_impl_deserialize_wrap {
    (struct $target:ident, $($el:ident),+) => {
        impl<'de> $crate::serde::BinDeserialize<'de> for $target {
            fn deserialize<R: Read>(mut pipe: R, dedup: &'de DedupContext, mode: &Mode) -> Result<Self> {
                $(let $el = $el::deserialize(&mut pipe, dedup, mode);)*
                $target { $($el),* }
            }
        }
    }
}
