mod net;
mod server;
mod error;
mod game;
mod client;


fn main() {
    let (tx, rx) = flume::unbounded();

    net::start(tx, 8000);
    server::run(rx);
}
