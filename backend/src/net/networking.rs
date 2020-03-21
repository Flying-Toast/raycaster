use std::sync::mpsc;
use ws::{Handler, Message, Handshake, CloseCode, listen};


/// Information that gets sent to the game thread.
pub enum NetMessage {
    /// Signifies that a new websocket connection has been made.
    /// The first field is an id that uniquely identifies this websocket.
    /// The second field is a `Responder` for sending messages back to the websocket.
    Connection(u32, Responder),
    /// A message from the websocket.
    /// The first field is the id of the websocket that sent this message.
    /// The second field is the message contents.
    Message(u32, String),
    /// Signifies that a websocket has disconnected.
    /// The field is the id of the websocket that disconnected.
    Disconnect(u32),
}

pub struct Responder {
    sender: ws::Sender,
}

impl Responder {
    pub fn send(&mut self, message: String) {
        let _ = self.sender.send(message);
    }
}

struct NetConnection {
    out: ws::Sender,
    /// For sending messages to the game thread.
    game: mpsc::Sender<NetMessage>,
    shunned: bool,
}

impl NetConnection {
    fn shun(&mut self, close: bool) {
        if close {
            let _ = self.out.close(CloseCode::Normal);
        }

        if !self.shunned {
            let _ = self.game.send(NetMessage::Disconnect(self.out.connection_id()));
        }

        self.shunned = true;
    }
}

impl Handler for NetConnection {
    fn on_open(&mut self, _: Handshake) -> ws::Result<()> {
        let res = self.game.send(NetMessage::Connection(
            self.out.connection_id(),
            Responder {
                sender: self.out.clone()
            }
        ));

        if let Err(_) = res {
            eprintln!("Closing websocket connection due to closed game thread");
            self.shun(true);
        }

        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> ws::Result<()> {
        if self.shunned {
            return Ok(());
        }

        if let Message::Text(message) = msg {
            let res = self.game.send(NetMessage::Message(
                self.out.connection_id(),
                message
            ));

            if let Err(_) = res {
                eprintln!("Closing websocket connection due to closed game thread");
                self.shun(true);
            }
        } else { //the client sent a binary message
            self.shun(true);
        }

        Ok(())
    }

    fn on_close(&mut self, _: CloseCode, _: &str) {
        self.shun(false);
    }
}

/// Runs the websocket server in this thread.
/// `game_tx` is the transmitting end of a channel for sending `NetMessage`s to the game thread.
pub fn start_network(host: &str, game_tx: mpsc::Sender<NetMessage>) -> Result<(), String> {
    listen(host, |sender| {
        NetConnection {
            out: sender,
            game: game_tx.clone(),
            shunned: false,
        }
    }).or(Err("Failed to start websocket server".to_string()))
}
