use crate::protocol::types::{GameMode};
use crate::error::*;
use std::str::{Lines, FromStr};
use crate::protocol::payload::{S2CPayload, C2SPayload};


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
    fn parse(lines: &mut Lines) -> Result<Self, RCE> {
        const E: RCE = RCE::ProtocolDecode;
        let map_name = lines.next().to(E)?.to_string();
        let gamemode = GameMode::from_str(lines.next().to(E)?).to(E)?;

        Ok(Self {
            map_name,
            gamemode,
        })
    }
}
