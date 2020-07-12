use crate::error::*;
use crate::protocol::payload::{S2CPayload, C2SPayload, Pieces};


/// Tells a client what their id is
#[derive(Debug)]
pub struct YourIDPayload {
    pub client_id: u32,
}
impl YourIDPayload {
    pub fn new(client_id: u32) -> Self {
        Self {
            client_id,
        }
    }
}
impl S2CPayload for YourIDPayload {
    fn encode(&self) -> String {
        let mut lines = lines!();
        let client_id = self.client_id.to_string();
        lines.push(&client_id);

        lines.join("\n")
    }
}

/// A 'ping' when in server->client direction,
/// A 'pong' (response to an earlier 'ping') when in client->server direction.
#[derive(Debug)]
pub struct PingPongPayload {
    pub id: u32,
}
impl S2CPayload for PingPongPayload {
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
