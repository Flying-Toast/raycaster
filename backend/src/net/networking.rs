use std::sync::mpsc;
use ws::{Handler, Message, Handshake, CloseCode, listen};
use crate::error::*;
use crate::net::protocol::{ProtocolMessage, next_message};


/// Information that gets sent to the game thread.
pub enum NetEvent {
    /// Signifies that a new websocket connection has been made.
    /// The first field is an id that uniquely identifies this websocket.
    /// The second field is a `Responder` for sending messages back to the websocket.
    Connect(u32, Responder),
    /// A message from the websocket.
    /// The first field is the id of the websocket that sent this message.
    /// The second field is the message contents.
    Message(u32, ProtocolMessage),
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
    game: mpsc::Sender<NetEvent>,
    shunned: bool,
}

impl NetConnection {
    fn shun(&mut self, close: bool) {
        if close {
            let _ = self.out.close(CloseCode::Normal);
        }

        if !self.shunned {
            let _ = self.game.send(NetEvent::Disconnect(self.out.connection_id()));
        }

        self.shunned = true;
    }
}

impl Handler for NetConnection {
    fn on_open(&mut self, _: Handshake) -> ws::Result<()> {
        let res = self.game.send(NetEvent::Connect(
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

        if let Message::Text(string) = msg {
            let mut lines = string.lines();

            loop {
                let message;
                match next_message(&mut lines) {
                    None => break,
                    Some(Err(e)) => {
                        eprintln!("Error while parsing message from websocket #{}: {:?}", self.out.connection_id(), e);
                        eprintln!("The (entire) packet containing the message with an error:");
                        eprintln!("=======================");
                        eprintln!("{}", string);
                        eprintln!("=======================");
                        break;
                    },
                    Some(Ok(m)) => message = m,
                }

                let res = self.game.send(NetEvent::Message(
                    self.out.connection_id(),
                    message
                ));

                if let Err(_) = res {
                    eprintln!("Killing websocket #{} because of closed game thread", self.out.connection_id());
                    self.shun(true);
                }
            }
        } else {
            eprintln!("Websocket #{} sent a binary message - killing it", self.out.connection_id());
            self.shun(true);
        }

        Ok(())
    }

    fn on_close(&mut self, _: CloseCode, _: &str) {
        self.shun(false);
    }
}

/// Runs the websocket server in this thread.
/// `game_tx` is the transmitting end of a channel for sending `NetEvent`s to the game thread.
pub fn start_network(host: &str, game_tx: mpsc::Sender<NetEvent>) -> Result<(), RCE> {
    listen(host, |sender| {
        NetConnection {
            out: sender,
            game: game_tx.clone(),
            shunned: false,
        }
    }).or(Err(RCE::NetworkFailedToStart))
}
