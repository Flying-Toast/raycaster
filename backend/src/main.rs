mod net;
mod game;
mod error;
mod protocol;

use net::start_network;
use game::start::run_game;


fn main() {
    let (tx, rx) = flume::unbounded();

    run_game(rx);
    start_network("0.0.0.0:8000", tx).unwrap();
}
