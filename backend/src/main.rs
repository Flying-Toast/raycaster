use std::sync::mpsc::channel;
use backend::net::networking::start_network;


fn main() {
    let (tx, rx) = channel();

    start_network("0.0.0.0:8000", tx).expect("Failed to start network");
}
