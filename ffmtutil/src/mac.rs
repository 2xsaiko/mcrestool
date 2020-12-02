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

    (struct $target:ident { $($(#[$meta:tt])* $el:ident),* $(, $(..$default:expr)?)? } $($more:tt)*) => {
        $crate::do_impl_serialize_wrap!(struct $target $($(META #[$meta])* FIELD $el)*);
        $crate::impl_serialize_wrap!($($more)*);
    };

    (struct $target:ident($($(#[$meta:tt])* $el:tt),* $(, $(..$default:expr)?)?); $($more:tt)*) => {
        $crate::do_impl_serialize_wrap!(struct $target $($(META #[$meta])* FIELD $el)*);
        $crate::impl_serialize_wrap!($($more)*);
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! do_impl_serialize_wrap {
    (struct $target:ident $($(META #[$meta:tt])* FIELD $el:tt)*) => {
        impl $crate::BinSerialize for $target {
            fn serialize<S: $crate::BinSerializer>(
                &self,
                mut serializer: S
            ) -> $crate::Result<()> {
                $(self.$el.serialize($crate::parse_attr_ser!(&mut serializer, $(#[$meta])*))?;)+
                Ok(())
            }
        }
    };
}

#[macro_export]
macro_rules! impl_deserialize_wrap {
    () => {};

    (struct $target:ident { $($(#[$meta:tt])* $el:ident),* $(, $(..$default:expr)?)? } $($more:tt)*) => {
        $crate::do_impl_deserialize_wrap!(struct $target $($(META #[$meta])* FIELD $el)* $($((DEFAULT $default))?)?);
        $crate::impl_deserialize_wrap!($($more)*);
    };

    (struct $target:ident($($(#[$meta:tt])* $el:tt),*); $($more:tt)*) => {
        $crate::do_impl_deserialize_wrap!(tuplestruct $target $($(META #[$meta])* FIELD $el)*);
        $crate::impl_deserialize_wrap!($($more)*);
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! do_impl_deserialize_wrap {
    (struct $target:ident $($(META #[$meta:tt])* FIELD $el:ident)* $((DEFAULT $default:expr))?) => {
        impl<'de> $crate::BinDeserialize<'de> for $target {
            fn deserialize<D: $crate::BinDeserializer<'de>>(mut deserializer: D) -> $crate::Result<Self> {
                $(let $el = $crate::BinDeserialize::deserialize($crate::parse_attr_de!(&mut deserializer, $(#[$meta])*))?;)*
                Ok($target { $($el,)* $(..$default)? })
            }

            fn deserialize_in_place<D: $crate::BinDeserializer<'de>>(
                &mut self,
                mut deserializer: D
            ) -> $crate::Result<()> {
                $($crate::BinDeserialize::deserialize_in_place(&mut self.$el, $crate::parse_attr_de!(&mut deserializer, $(#[$meta])*))?;)*
                Ok(())
            }
        }
    };

    (tuplestruct $target:ident $($(META #[$meta:tt])* FIELD $el:tt)*) => {
        impl<'de> $crate::BinDeserialize<'de> for $target {
            fn deserialize<D: $crate::BinDeserializer<'de>>(mut deserializer: D) -> $crate::Result<Self> {
                $(let $crate::member_to_ident!($el) = $crate::BinDeserialize::deserialize($crate::parse_attr_de!(&mut deserializer, $(#[$meta])*))?;)*
                Ok($target($($crate::member_to_ident!($el),)*))
            }

            fn deserialize_in_place<D: $crate::BinDeserializer<'de>>(
                &mut self,
                mut deserializer: D
            ) -> $crate::Result<()> {
                $($crate::BinDeserialize::deserialize_in_place(&mut self.$el, $crate::parse_attr_de!(&mut deserializer, $(#[$meta])*))?;)*
                Ok(())
            }
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! parse_attr_ser {
    ($expr:expr,) => { $expr };

    ($expr:expr, #[$first:tt] $(#[$more:tt])*) => {
        $crate::parse_attr_ser!($crate::parse_attr_ser!(apply #[$first], $expr), $(#[$more])*)
    };

    (apply #[no_dedup], $expr:expr) => {
        $crate::BinSerializer::disable_dedup($expr)
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! parse_attr_de {
    ($expr:expr,) => { $expr };

    ($expr:expr, #[$first:tt] $(#[$more:tt])*) => {
        $crate::parse_attr_de!($crate::parse_attr_de!(apply $first, $expr), $(#[$more])*)
    };

    (apply no_dedup, $expr:expr) => {
        $crate::BinDeserializer::disable_dedup($expr)
    }
}
