use std::sync::mpsc::{Receiver, TryRecvError};
use std::time::{Instant, Duration};
use std::thread;
use crate::net::{NetEvent, Responder};
use crate::game::map::Map;
use crate::protocol::ClientMessage;


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

    /// Called when a client disconnects
    fn on_client_disconnect(&mut self, connection_id: u32) {

    }

    /// Called when a client connects
    fn on_client_connect(&mut self, connection_id: u32, responder: Responder) {

    }

    /// Called for each message received from a client
    fn on_client_message(&mut self, connection_id: u32, message: ClientMessage) {
        match message {
            ClientMessage::Pong(payload) => {

            },
        }
    }

    fn tick(&mut self, dt_micros: u128) {

    }

    /// Receives and processes pending NetEvents.
    /// Processes a maximum of 500 events each call.
    fn process_net_events(&mut self) {
        const MAX_EVENTS: u32 = 500;
        for _ in 0..MAX_EVENTS {
            match self.rx.try_recv() {
                Ok(event) => match event {
                                 NetEvent::Connect(id, responder) => self.on_client_connect(id, responder),
                                 NetEvent::Disconnect(id) => self.on_client_disconnect(id),
                                 NetEvent::Message(id, message) => self.on_client_message(id, message),
                             },
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => panic!("Network channel disconnected"),
            }
        }
    }

    /// Starts the game loop.
    pub fn run(&mut self) -> ! {
        const MIN_INTERVAL_MICROS: u64 = 100;
        let mut last_tick = Instant::now();
        loop {
            self.process_net_events();

            let now = Instant::now();
            self.tick((now - last_tick).as_micros());
            last_tick = now;

            thread::sleep(Duration::from_micros(MIN_INTERVAL_MICROS));
        }
    }
}
