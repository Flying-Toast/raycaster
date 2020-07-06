use crate::error::*;
use crate::protocol::payload::{S2CPayload, C2SPayload, Pieces};


pub struct PingPayload {
    pub id: u32,
}
impl S2CPayload for PingPayload {
    key!("p");

    fn encode(&self) -> String {
        let mut lines = lines!();
        let id = self.id.to_string();
        lines.push(&id);

        lines.join("\n")
    }
}

pub struct PongPayload {
    pub id: u32,
}
impl C2SPayload for PongPayload {
    fn parse(pieces: &mut Pieces) -> Result<Self, RCE> {
        let id: u32 = pieces.get()?;

        Ok(Self {
            id,
        })
    }
}
