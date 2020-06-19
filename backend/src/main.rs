use std::sync::mpsc::channel;
use backend::net::networking::start_network;
use backend::game::start::run_game;


fn main() {
    let (tx, rx) = channel();

    run_game(rx);
    start_network("0.0.0.0:8000", tx).unwrap();
}
