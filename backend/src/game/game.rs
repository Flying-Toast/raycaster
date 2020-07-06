use std::sync::mpsc::Receiver;
use crate::net::NetEvent;
use crate::game::map::Map;


pub struct Game {
    map: Map,
    /// The receiving end of the channel from the network thread.
    rx: Receiver<NetEvent>,
}

impl Game {
    pub fn new(rx: Receiver<NetEvent>, map: Map) -> Self {
        Self {
            map,
            rx,
        }
    }

    /// Starts the game loop.
    pub fn run(&mut self) -> ! {
        loop {

        }
    }
}
