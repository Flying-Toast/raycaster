use crate::error::*;
use crate::protocol::payload::{S2CPayload, C2SPayload, Pieces, PayloadBuilder};
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
    fn encode(&self) -> PayloadBuilder {
        let mut builder = builder!();
        builder.add_str(&self.entity_id.to_string());

        builder
    }
}

/// A 'ping' when in server->client direction,
/// A 'pong' (response to an earlier 'ping') when in client->server direction.
#[derive(Debug)]
pub struct PingPongPayload {
    pub id: u32,
}
impl S2CPayload for PingPongPayload {
    fn encode(&self) -> PayloadBuilder {
        let mut builder = builder!();
        builder.add_str(&self.id.to_string());

        builder
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
