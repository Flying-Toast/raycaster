use std::thread;
use std::sync::mpsc::Receiver;
use crate::net::networking::NetMessage;
use crate::server::game::Game;


/// Spawns a thread that runs a game server.
/// `rx` is the receiver from the networking thread.
pub fn run_game(rx: Receiver<NetMessage>) {
    thread::spawn(move || {
        let mut game = Game::new(rx);
        game.run();
    });
}
