use std::collections::HashMap;
use crate::net::{Responder, ClientID};
use crate::protocol::ClientMessage;
use crate::game::map::Map;
use crate::protocol::payloads::*;
use crate::game::client::Client;
use crate::game::entity::{Entity, EntityID};


pub struct Game {
    map: Map,
    clients: HashMap<ClientID, Client>,
    entities: HashMap<EntityID, Entity>,
    next_entity_id: u32,
}

impl Game {
    pub fn new() -> Self {
        Self {
            map: Map::from_file("../maps/default").expect("Error reading default map"),
            clients: HashMap::new(),
            entities: HashMap::new(),
            next_entity_id: 0,
        }
    }

    pub fn on_client_disconnect(&mut self, connection_id: ClientID) {
        if let Some(client) = self.clients.get(&connection_id) {
            // remove the client's player
            self.entities.remove(&client.player_entity);
        }

        // remove the client itself
        self.clients.remove(&connection_id);
    }

    pub fn on_client_connect(&mut self, connection_id: ClientID, mut responder: Responder) {
        // tell the client their id
        responder.send(YourIDPayload::new(connection_id));

        let ent_id = self.gen_entity_id();
        self.entities.insert(ent_id, Entity::new());
        self.clients.insert(connection_id, Client::new(responder, ent_id));
    }

    pub fn on_client_message(&mut self, connection_id: ClientID, message: ClientMessage) {
        match message {
            ClientMessage::Pong(payload) => {
            },
        }
    }

    pub fn tick(&mut self, dt: u128) {
        self.send_queued_messages();
    }

    /// Flushes the outgoing message queues of all clients
    fn send_queued_messages(&mut self) {
        for client in self.clients.values_mut() {
            client.responder.flush();
        }
    }

    fn gen_entity_id(&mut self) -> EntityID {
        self.next_entity_id = self.next_entity_id.wrapping_add(1);

        EntityID::new(self.next_entity_id)
    }
}
