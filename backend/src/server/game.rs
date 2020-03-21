use std::sync::mpsc::Receiver;
use crate::net::networking::NetMessage;
use crate::server::map::Map;


pub struct Game {
    map: Map,
    /// The receiving end of the channel from the network thread.
    rx: Receiver<NetMessage>,
}

impl Game {
    pub fn new(rx: Receiver<NetMessage>) -> Self {
        Self {
            map: Map::from_file("../maps/default.map").expect("Failed to load map"),
            rx,
        }
    }

    /// Starts the game loop.
    pub fn run(&mut self) {

    }
}
