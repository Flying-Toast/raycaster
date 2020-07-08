use std::thread;
use flume::Receiver;
use crate::net::NetEvent;
use crate::server::game::Game;


/// Spawns a game thread.
/// `rx` is the receiver from the networking thread.
pub fn run_game(rx: Receiver<NetEvent>) {
    thread::spawn(move || {
        let mut game = Game::new(rx);
        game.run();
    });
}
