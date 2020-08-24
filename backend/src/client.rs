use crate::net::Responder;
use common::entity::EntityID;
use common::protocol::payload::BuiltPayload;


pub struct Client {
    responder: Responder,
    player_entity: EntityID,
    pub last_processed_input: u32,
}

impl Client {
    pub fn new(responder: Responder, player_entity: EntityID) -> Self {
        Self {
            responder,
            player_entity,
            last_processed_input: 0,
        }
    }

    /// The ID of the entity that this client controls (their player)
    pub fn player_entity(&self) -> EntityID {
        self.player_entity
    }

    pub fn send(&mut self, payload: &BuiltPayload) {
        self.responder.send(payload);
    }

    pub fn flush_messages(&mut self) {
        self.responder.flush();
    }

    pub fn disconnect(self) {
        self.responder.close();
    }
}
