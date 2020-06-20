use std::thread;
use std::sync::mpsc::Receiver;
use crate::net::networking::NetMessage;
use crate::game::game::Game;
use crate::game::map::Map;


/// Spawns a game thread.
/// `rx` is the receiver from the networking thread.
pub fn run_game(rx: Receiver<NetMessage>) {
    let default_map = Map::from_file("../maps/default").unwrap();

    thread::spawn(move || {
        let mut game = Game::new(rx, default_map);
        game.run();
    });
}
