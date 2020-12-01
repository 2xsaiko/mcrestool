#[macro_export]
macro_rules! impl_serde_wrap {
    ($($token:tt)*) => {
        $crate::impl_serialize_wrap!($($token)*);
        $crate::impl_deserialize_wrap!($($token)*);
    }
}

#[macro_export]
macro_rules! impl_serialize_wrap {
    () => {};

    (struct $target:ident { $($el:ident),* $(, $(..$default:expr)?)? } $($more:tt)*) => {
        $crate::do_impl_serialize_wrap!(struct $target $($el)*);
        $crate::impl_serialize_wrap!($($more)*);
    };

    (struct $target:ident($($el:tt),* $(, $(..$default:expr)?)?); $($more:tt)*) => {
        $crate::do_impl_serialize_wrap!(struct $target $($el)*);
        $crate::impl_serialize_wrap!($($more)*);
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! do_impl_serialize_wrap {
    (struct $target:ident $($el:tt)*) => {
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
    };
}

#[macro_export]
macro_rules! impl_deserialize_wrap {
    () => {};

    (struct $target:ident { $($el:ident),* $(, $(..$default:expr)?)? } $($more:tt)*) => {
        $crate::do_impl_deserialize_wrap!(struct $target $($el)* $($((default $default))?)?);
        $crate::impl_deserialize_wrap!($($more)*);
    };

    (struct $target:ident($($el:tt),*); $($more:tt)*) => {
        $crate::do_impl_deserialize_wrap!(tuplestruct $target $($el)*);
        $crate::impl_deserialize_wrap!($($more)*);
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! do_impl_deserialize_wrap {
    (struct $target:ident $($el:ident)* $((default $default:expr))?) => {
        impl<'de> $crate::serde::BinDeserialize<'de> for $target {
            fn deserialize<R: Read>(
                mut pipe: R,
                dedup: &'de $crate::dedup::DedupContext,
                mode: &$crate::serde::Mode,
            ) -> $crate::Result<Self> {
                $(let $el = $crate::serde::BinDeserialize::deserialize(&mut pipe, dedup, mode)?;)*
                Ok($target { $($el,)* $(..$default)? })
            }

            fn deserialize_in_place<R: Read>(
                &mut self,
                mut pipe: R,
                dedup: &'de $crate::dedup::DedupContext,
                mode: &$crate::serde::Mode,
            ) -> $crate::Result<()> {
                $($crate::serde::BinDeserialize::deserialize_in_place(&mut self.$el, &mut pipe, dedup, mode)?;)*
                Ok(())
            }
        }
    };

    (tuplestruct $target:ident $($el:tt)*) => {
        impl<'de> $crate::serde::BinDeserialize<'de> for $target {
            fn deserialize<R: Read>(
                mut pipe: R,
                dedup: &'de $crate::dedup::DedupContext,
                mode: &$crate::serde::Mode,
            ) -> $crate::Result<Self> {
                $(let $crate::member_to_ident!($el) = $crate::serde::BinDeserialize::deserialize(&mut pipe, dedup, mode)?;)*
                Ok($target($($crate::member_to_ident!($el),)*))
            }

            fn deserialize_in_place<R: Read>(
                &mut self,
                mut pipe: R,
                dedup: &'de $crate::dedup::DedupContext,
                mode: &$crate::serde::Mode,
            ) -> $crate::Result<()> {
                $($crate::serde::BinDeserialize::deserialize_in_place(&mut self.$el, &mut pipe, dedup, mode)?;)*
                Ok(())
            }
        }
    };
}
