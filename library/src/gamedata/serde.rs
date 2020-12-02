use std::io::Read;

use byteorder::{ReadBytesExt, WriteBytesExt};

use ffmtutil::de::BinDeserializer;
use ffmtutil::dedup::DedupContext;
use ffmtutil::serde::{BinDeserialize, BinSerialize, BinSerializer, Mode};
use ffmtutil::{Error, Result};

use crate::gamedata::*;

impl<'de> BinDeserialize<'de> for DependencyLink {
    fn deserialize<D: BinDeserializer<'de>>(mut deserializer: D) -> Result<Self, Error> {
        let typ = u8::deserialize(&mut deserializer)?;
        match typ {
            0 => {
                let namespace = String::deserialize(&mut deserializer)?;
                let lang_name = String::deserialize(&mut deserializer)?;
                Ok(DependencyLink::Language(namespace, lang_name))
            }
            1 => Ok(DependencyLink::Block(Identifier::deserialize(
                &mut deserializer,
            )?)),
            2 => Ok(DependencyLink::Item(Identifier::deserialize(
                &mut deserializer,
            )?)),
            _ => Err(Error::custom(format!(
                "invalid dependency link type: {}",
                typ
            ))),
        }
    }
}

impl BinSerialize for DependencyLink {
    fn serialize<S: BinSerializer>(&self, mut serializer: S) -> Result<(), Error> {
        match self {
            DependencyLink::Language(namespace, lang_name) => {
                0u8.serialize(&mut serializer)?;
                namespace.serialize(&mut serializer)?;
                lang_name.serialize(&mut serializer)?;
            }
            DependencyLink::Block(id) => {
                1u8.serialize(&mut serializer)?;
                id.serialize(&mut serializer)?;
            }
            DependencyLink::Item(id) => {
                2u8.serialize(&mut serializer)?;
                id.serialize(&mut serializer)?;
            }
        }
        Ok(())
    }
}

impl<'de> BinDeserialize<'de> for GameObjectBase {
    fn deserialize<D: BinDeserializer<'de>>(mut deserializer: D) -> Result<Self, Error> {
        let id = Identifier::deserialize(&mut deserializer)?;
        let bits = deserializer.pipe().read_u8()?;
        let manual = bits & 1 != 0;
        let auto = if bits & 2 != 0 {
            if bits & 4 != 0 {
                AutoStatus::Deleted
            } else {
                AutoStatus::Yes
            }
        } else {
            AutoStatus::No
        };

        Ok(GameObjectBase { manual, auto, id })
    }
}

impl BinSerialize for GameObjectBase {
    fn serialize<S: BinSerializer>(&self, mut serializer: S) -> Result<()> {
        self.id.serialize(&mut serializer)?;
        let mut bits = 0;

        if self.manual {
            bits |= 1;
        }

        bits |= match self.auto {
            AutoStatus::No => 0,
            AutoStatus::Yes => 2,
            AutoStatus::Deleted => 5,
        };

        serializer.pipe().write_u8(bits)?;
        Ok(())
    }
}
