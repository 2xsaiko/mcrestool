use crate::gamedata::GameData;
use std::io::Write;
pub use crate::workspace::serde::Result;

impl GameData {
    pub fn write_into<W: Write>(&self, mut pipe: W) -> Result<()> {
        // self.refs.map.
            
            Ok(())
    }
}