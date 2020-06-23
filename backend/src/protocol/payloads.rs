use crate::protocol::types::{GameMode};
use crate::error::*;
use crate::protocol::payload::{S2CPayload, C2SPayload, Pieces};


pub struct NewGamePayload {
    pub map_name: String,
    pub gamemode: GameMode,
}
impl S2CPayload for NewGamePayload {
    key!("ng");

    fn encode(&self) -> String {
        let mut lines = lines!();
        lines.push(&self.map_name[..]);
        lines.push(self.gamemode.as_ref());

        lines.join("\n")
    }
}
impl C2SPayload for NewGamePayload {
    fn parse(pieces: &mut Pieces) -> Result<Self, RCE> {
        const E: RCE = RCE::PayloadDecode;
        let map_name = pieces.get_str()?.to_string();
        let gamemode: GameMode = pieces.generic_get()?;

        Ok(Self {
            map_name,
            gamemode,
        })
    }
}
