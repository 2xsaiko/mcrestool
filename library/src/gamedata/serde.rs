use byteorder::{ReadBytesExt, WriteBytesExt};

use ffmtutil::de::BinDeserializer;
use ffmtutil::{BinDeserialize, BinSerialize, BinSerializer};
use ffmtutil::{Error, Result};

use crate::gamedata::*;

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
