use crate::error::*;
use crate::protocol::payload::{S2CPayload, C2SPayload, Pieces};


/// A 'ping' when in server->client direction,
/// A 'pong' (response to an earlier 'ping') when in client->server direction.
#[derive(Debug)]
pub struct PingPongPayload {
    pub id: u32,
}
impl S2CPayload for PingPongPayload {
    key!("p");

    fn encode(&self) -> String {
        let mut lines = lines!();
        let id = self.id.to_string();
        lines.push(&id);

        lines.join("\n")
    }
}
impl C2SPayload for PingPongPayload {
    fn parse(pieces: &mut Pieces) -> Result<Self, RCE> {
        let id: u32 = pieces.get()?;

        Ok(Self {
            id,
        })
    }
}
