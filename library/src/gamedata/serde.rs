use crate::binserde::{Result, WriteContext, ReadContext};
use crate::gamedata::{GameData, DependencyLink, GameObjectBase, AutoStatus};
use byteorder::{WriteBytesExt, LE, ReadBytesExt};
use std::convert::TryInto;
use std::io::Read;

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

        }

        Ok(())
    }
}

impl DependencyLink {
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