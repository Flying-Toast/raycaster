use flume::Receiver;
use std::time::{Instant, Duration};
use std::thread;
use crate::net::NetEvent;
use crate::game::Game;


/// Runs a `Server` in a new thread.
/// `rx` is the receiver from the networking thread.
pub fn run(rx: Receiver<NetEvent>) {
    thread::Builder::new()
        .name("server".to_string())
        .spawn(move || {
            let mut server = Server::new(rx);
            server.run();
        }).unwrap();
}

pub struct Server {
    /// The receiving end of the channel from the network thread.
    rx: Receiver<NetEvent>,
    game: Game,
}

impl Server {
    pub fn new(rx: Receiver<NetEvent>) -> Self {
        Self {
            game: Game::new(),
            rx,
        }
    }

    /// Receives and processes pending NetEvents.
    fn process_net_events(&mut self) {
        for event in self.rx.drain() {
            match event {
                NetEvent::Connect(id, responder) => self.game.on_client_connect(id, responder),
                NetEvent::Disconnect(id) => self.game.on_client_disconnect(id),
                NetEvent::Message(id, message) => self.game.on_client_message(id, message),
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
            self.game.tick((now - last_tick).as_millis());
            last_tick = now;

            thread::sleep(Duration::from_millis(TIME_STEP_MILLIS));
        }
    }
}
