use crate::net::Responder;
use crate::protocol::ClientMessage;
use crate::game::map::Map;
use crate::protocol::payloads::*;


pub struct Game {
    map: Map,
}

impl Game {
    pub fn new() -> Self {
        Self {
            map: Map::from_file("../maps/default").expect("Error reading default map"),
        }
    }

    pub fn on_client_disconnect(&mut self, connection_id: u32) {
    }

    pub fn on_client_connect(&mut self, connection_id: u32, mut responder: Responder) {
        // tell the client their id
        responder.send(YourIDPayload::new(connection_id));
    }

    pub fn on_client_message(&mut self, connection_id: u32, message: ClientMessage) {
        match message {
            ClientMessage::Pong(payload) => {
            },
        }
    }

    pub fn tick(&mut self, dt: u128) {
    }
}
