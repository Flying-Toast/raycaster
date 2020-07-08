mod net;
mod server;
mod error;
mod protocol;
mod game;

use net::start_network;
use server::run_server;


fn main() {
    let (tx, rx) = flume::unbounded();

    run_server(rx);
    start_network("0.0.0.0:8000", tx).unwrap();
}
