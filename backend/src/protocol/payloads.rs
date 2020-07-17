use crate::error::*;
use crate::protocol::payload::{S2CPayload, C2SPayload, Pieces};
use crate::game::entity::EntityID;


/// Tells a client what their player entity's id is
#[derive(Debug)]
pub struct YourIDPayload {
    pub entity_id: EntityID,
}
impl YourIDPayload {
    pub fn new(entity_id: EntityID) -> Self {
        Self {
            entity_id,
        }
    }
}
impl S2CPayload for YourIDPayload {
    fn encode(&self) -> String {
        let mut lines = lines!();
        let entity_id = self.entity_id.to_string();
        lines.push(&entity_id);

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
        let id = pieces.get_u32()?;

        Ok(Self {
            id,
        })
    }
}
