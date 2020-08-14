mod net;
mod server;
mod protocol;
mod game;


fn main() {
    let (tx, rx) = flume::unbounded();

    server::run(rx);
    net::start(tx, 8000).unwrap();
}
