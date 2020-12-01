
use std::io::{Read, Write};

use byteorder::{ReadBytesExt, WriteBytesExt};

use ffmtutil::serde::{BinDeserialize, BinSerialize, Mode};
use ffmtutil::{Error,  Result};

use crate::gamedata::*;
use ffmtutil::dedup::DedupContext;

impl <'de> BinDeserialize<'de> for DependencyLink {
    fn deserialize<R: Read>(mut pipe: R, dedup: &'de DedupContext, mode: &Mode) -> Result<Self, Error> {
        let typ = u8::deserialize(&mut pipe, dedup, mode)?;
        match typ {
            0 => {
                let namespace = String::deserialize(&mut pipe, dedup, mode)?;
                let lang_name = String::deserialize(&mut pipe, dedup, mode)?;
                Ok(DependencyLink::Language(namespace, lang_name))
            }
            1 => Ok(DependencyLink::Block(
                Identifier::deserialize(&mut pipe, dedup, mode)?,
            )),
            2 => Ok(DependencyLink::Item(
                Identifier::deserialize(&mut pipe, dedup, mode)?,
            )),
            _ => Err(Error::custom(format!(
                "invalid dependency link type: {}",
                typ
            ))),
        }
    }
}

impl BinSerialize for DependencyLink {
    fn serialize<W: Write>(
        &self,
        mut pipe: W,
        dedup: &mut DedupContext,
        mode: &Mode,
    ) -> Result<(), Error> {
        match self {
            DependencyLink::Language(namespace, lang_name) => {
                0u8.serialize(&mut pipe, dedup, mode)?;
                namespace.serialize(&mut pipe, dedup, mode)?;
                lang_name.serialize(&mut pipe, dedup, mode)?;
            }
            DependencyLink::Block(id) => {
                1u8.serialize(&mut pipe, dedup, mode)?;
                id.serialize(&mut pipe, dedup, mode)?;
            }
            DependencyLink::Item(id) => {
                2u8.serialize(&mut pipe, dedup, mode)?;
                id.serialize(&mut pipe, dedup, mode)?;
            }
        }
        Ok(())
    }
}

impl<'de> BinDeserialize<'de> for GameObjectBase {
    fn deserialize<R: Read>(
        mut pipe: R,
        dedup: &'de DedupContext,
        mode: &Mode,
    ) -> Result<Self, Error> {
        let id = Identifier::deserialize(&mut pipe, dedup, mode)?;
        let bits = pipe.read_u8()?;
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
    fn serialize<W: Write>(
        &self,
        mut pipe: W,
        dedup: &mut DedupContext,
        mode: &Mode,
    ) -> Result<()> {
        self.id.serialize(&mut pipe, dedup, mode)?;
        let mut bits = 0;

        if self.manual {
            bits |= 1;
        }

        bits |= match self.auto {
            AutoStatus::No => 0,
            AutoStatus::Yes => 2,
            AutoStatus::Deleted => 5,
        };

        pipe.write_u8(bits)?;
        Ok(())
    }
}
