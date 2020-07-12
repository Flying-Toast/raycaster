use std::collections::HashMap;
use crate::net::{Responder, ClientID};
use crate::protocol::ClientMessage;
use crate::game::map::Map;
use crate::protocol::payloads::*;
use crate::game::client::Client;


pub struct Game {
    map: Map,
    clients: HashMap<ClientID, Client>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            map: Map::from_file("../maps/default").expect("Error reading default map"),
            clients: HashMap::new(),
        }
    }

    pub fn on_client_disconnect(&mut self, connection_id: ClientID) {
        self.clients.remove(&connection_id);
    }

    pub fn on_client_connect(&mut self, connection_id: ClientID, mut responder: Responder) {
        // tell the client their id
        responder.send(YourIDPayload::new(connection_id));

        self.clients.insert(connection_id, Client::new(responder));
    }

    pub fn on_client_message(&mut self, connection_id: ClientID, message: ClientMessage) {
        match message {
            ClientMessage::Pong(payload) => {
            },
        }
    }

    pub fn tick(&mut self, dt: u128) {
    }
}
