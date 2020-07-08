use flume::{Receiver, TryRecvError};
use std::time::{Instant, Duration};
use std::thread;
use crate::net::{NetEvent, Responder};
use crate::protocol::ClientMessage;


/// Spawns a game thread.
/// `rx` is the receiver from the networking thread.
pub fn run_server(rx: Receiver<NetEvent>) {
    thread::spawn(move || {
        let mut server = Server::new(rx);
        server.run();
    });
}

pub struct Server {
    /// The receiving end of the channel from the network thread.
    rx: Receiver<NetEvent>,
}

impl Server {
    pub fn new(rx: Receiver<NetEvent>) -> Self {
        Self {
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

    fn tick(&mut self, dt: u128) {

    }

    /// Receives and processes pending NetEvents.
    //TODO: remove 500 limit and use flume::Receiver::drain()
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

    /// Starts the main loop.
    pub fn run(&mut self) -> ! {
        const TIME_STEP_MILLIS: u64 = 100;
        let mut last_tick = Instant::now();
        loop {
            self.process_net_events();

            let now = Instant::now();
            self.tick((now - last_tick).as_millis());
            last_tick = now;

            thread::sleep(Duration::from_millis(TIME_STEP_MILLIS));
        }
    }
}
