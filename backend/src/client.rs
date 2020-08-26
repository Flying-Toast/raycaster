use crate::net::Responder;
use common::entity::EntityID;
use common::protocol::payload::BuiltPayload;
use common::input::ForeignInput;


pub struct Client {
    responder: Responder,
    player_entity: EntityID,
    pub last_processed_input: u32,
    pub last_acknowledged_input: u32,
    pub unbroadcast_inputs: Vec<ForeignInput>,
}

impl Client {
    pub fn new(responder: Responder, player_entity: EntityID) -> Self {
        Self {
            responder,
            player_entity,
            last_processed_input: 0,
            last_acknowledged_input: 0,
            unbroadcast_inputs: Vec::new(),
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
