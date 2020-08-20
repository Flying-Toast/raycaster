mod net;
mod server;
mod error;
mod game;
mod client;


fn main() {
    let (tx, rx) = flume::unbounded();

    server::run(rx);
    net::start(tx, 8000).unwrap();
}
