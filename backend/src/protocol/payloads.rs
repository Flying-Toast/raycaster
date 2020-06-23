use crate::protocol::types::{GameMode};
use crate::error::*;
use std::str::FromStr;
use crate::protocol::payload::{S2CPayload, C2SPayload, Pieces};


pub struct NewGamePayload {
    pub map_name: String,
    pub gamemode: GameMode,
}
impl S2CPayload for NewGamePayload {
    msg_key!("ng");

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
        let gamemode = GameMode::from_str(pieces.get_str()?).to(E)?;

        Ok(Self {
            map_name,
            gamemode,
        })
    }
}
