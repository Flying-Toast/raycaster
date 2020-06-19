use std::sync::mpsc::Receiver;
use crate::net::networking::NetMessage;
use crate::game::map::Map;


pub struct Game {
    map: Map,
    /// The receiving end of the channel from the network thread.
    rx: Receiver<NetMessage>,
}

impl Game {
    pub fn new(rx: Receiver<NetMessage>, map: Map) -> Self {
        Self {
            map,
            rx,
        }
    }

    /// Starts the game loop.
    pub fn run(&mut self) {

    }
}
