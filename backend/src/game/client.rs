use crate::net::Responder;
use common::entity::EntityID;


pub struct Client {
    pub responder: Responder,
    /// The ID of the entity that this client controls (their player)
    pub player_entity: EntityID,
}

impl Client {
    pub fn new(responder: Responder, player_entity: EntityID) -> Self {
        Self {
            responder,
            player_entity,
        }
    }
}
