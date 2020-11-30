use std::convert::TryInto;
use std::io::Read;

use byteorder::{ReadBytesExt, WriteBytesExt, LE};

use ffmtutil::{Error, ReadContext, Result, WriteContext};

use crate::gamedata::*;

impl GameData {
    pub fn write_into(&self, ctx: &mut WriteContext) -> Result<()> {
        ctx.write_u32::<LE>(self.refs.map.len().try_into()?)?;
        for (source, targets) in self.refs.map.iter() {
            source.write_into(ctx)?;
            ctx.write_u32::<LE>(targets.len().try_into()?)?;
            for target in targets.iter() {
                target.write_into(ctx)?;
            }
        }

        ctx.write_u32::<LE>(self.blocks.len().try_into()?)?;
        for (_, block) in self.blocks.iter() {
            block.base.write_into(ctx)?;
        }

        ctx.write_u32::<LE>(self.items.len().try_into()?)?;
        for (_, items) in self.items.iter() {
            items.base.write_into(ctx)?;
        }

        Ok(())
    }

    pub fn read_from_in_place<R: Read>(&mut self, ctx: &mut ReadContext<R>) -> Result<()> {
        self.reset();

        for _ in 0..ctx.read_u32::<LE>()? {
            let source = DependencyLink::read_from(ctx)?;
            let map = self.refs.map.entry(source).or_default();
            for _ in 0..ctx.read_u32::<LE>()? {
                let target = DependencyLink::read_from(ctx)?;
                map.insert(target);
            }
        }

        for _ in 0..ctx.read_u32::<LE>()? {
            let base = GameObjectBase::read_from(ctx)?;
            let block = Block::new(base);
            self.blocks.insert(block.base.id.clone(), block);
        }

        for _ in 0..ctx.read_u32::<LE>()? {
            let base = GameObjectBase::read_from(ctx)?;
            let item = Item::new(base);
            self.items.insert(item.base.id.clone(), item);
        }

        Ok(())
    }
}

impl DependencyLink {
    fn read_from<R: Read>(ctx: &mut ReadContext<R>) -> Result<DependencyLink> {
        let typ = ctx.read_u8()?;
        match typ {
            0 => {
                let namespace = ctx.read_dedup_str()?.to_string();
                let lang_name = ctx.read_dedup_str()?.to_string();
                Ok(DependencyLink::Language(namespace, lang_name))
            }
            1 => Ok(DependencyLink::Block(
                ctx.read_dedup_ident()?.to_identifier(),
            )),
            2 => Ok(DependencyLink::Item(
                ctx.read_dedup_ident()?.to_identifier(),
            )),
            _ => Err(Error::Other(format!(
                "invalid dependency link type: {}",
                typ
            ))),
        }
    }

    fn write_into(&self, ctx: &mut WriteContext) -> Result<()> {
        match self {
            DependencyLink::Language(namespace, lang_name) => {
                ctx.write_u8(0)?;
                ctx.write_dedup_str(namespace)?;
                ctx.write_dedup_str(lang_name)?;
            }
            DependencyLink::Block(id) => {
                ctx.write_u8(1)?;
                ctx.write_dedup_ident(&id)?;
            }
            DependencyLink::Item(id) => {
                ctx.write_u8(2)?;
                ctx.write_dedup_ident(&id)?;
            }
        }
        Ok(())
    }
}

impl GameObjectBase {
    fn read_from<R: Read>(ctx: &mut ReadContext<R>) -> Result<GameObjectBase> {
        let id = ctx.read_dedup_ident()?.to_identifier();
        let bits = ctx.read_u8()?;
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

    fn write_into(&self, ctx: &mut WriteContext) -> Result<()> {
        ctx.write_dedup_ident(&self.id)?;
        let mut bits = 0;

        if self.manual {
            bits |= 1;
        }

        bits |= match self.auto {
            AutoStatus::No => 0,
            AutoStatus::Yes => 2,
            AutoStatus::Deleted => 5,
        };

        ctx.write_u8(bits)?;

        Ok(())
    }
}
