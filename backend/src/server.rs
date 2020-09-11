use flume::Receiver;
use std::time::{Instant, Duration};
use std::thread;
use crate::net::NetEvent;
use crate::game::Game;

/// Runs a `Server` in this thread.
/// `rx` is the receiver from the network thread.
pub fn run(rx: Receiver<NetEvent>) -> ! {
    let mut server = Server::new(rx);
    server.run();
}

struct Server {
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
        // "drain" the receiver and iterate over the drained values,
        // so that this doesnt get caught in an infinite loop if messages arrive during processing
        for event in self.rx.drain() {
            match event {
                NetEvent::Connect(id, responder) => self.game.on_client_connect(id, responder),
                NetEvent::Disconnect(id) => self.game.on_client_disconnect(id),
                NetEvent::Message(id, message) => self.game.on_client_message(id, message),
            }
        }

        if self.rx.is_disconnected() {
            panic!("Net thread disconnected");
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
