use crate::net::Responder;
use crate::protocol::ClientMessage;


pub struct Game {
}

impl Game {
    pub fn new() -> Self {
        Self {
        }
    }

    /// Called when a client disconnects
    pub fn on_client_disconnect(&mut self, connection_id: u32) {
    }

    /// Called when a client connects
    pub fn on_client_connect(&mut self, connection_id: u32, responder: Responder) {
    }

    /// Called for each message received from a client
    pub fn on_client_message(&mut self, connection_id: u32, message: ClientMessage) {
        match message {
            ClientMessage::Pong(payload) => {
            },
        }
    }

    pub fn tick(&mut self, dt: u128) {
    }
}
