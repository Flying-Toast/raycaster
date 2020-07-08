use ws::{Handler, Message, Handshake, CloseCode, listen};
use crate::error::*;
use crate::protocol::{ClientMessage, next_message};
use crate::protocol::payload::{S2CPayload, Pieces};


/// Runs the network server in this thread.
/// `server_tx` is the transmitting end of a channel for sending `NetEvent`s to the server thread.
pub fn start(server_tx: flume::Sender<NetEvent>, port: u16) -> Result<(), RCE> {
    listen(format!("0.0.0.0:{}", port), |sender| {
        NetConnection::new(sender, server_tx.clone())
    }).or(Err(RCE::NetworkFailedToStart))
}

/// Type that is sent over a channel the server thread.
#[derive(Debug)]
pub enum NetEvent {
    /// A client connected.
    /// The first field is an id that uniquely identifies this client.
    /// The second field is a `Responder` for sending messages back to the client.
    Connect(u32, Responder),
    /// A message from the client.
    /// The first field is the id of the client who sent this message.
    /// The second field is the message.
    Message(u32, ClientMessage),
    /// A client disconnected.
    /// The field is the id of the client who disconnected.
    Disconnect(u32),
}

#[derive(Debug)]
pub struct Responder {
    sender: ws::Sender,
}

impl Responder {
    pub fn send(&mut self, payload: impl S2CPayload) {
        let _ = self.sender.send(payload.encode());
    }
}

struct NetConnection {
    id: u32,
    out: ws::Sender,
    /// For sending messages to the server thread.
    server: flume::Sender<NetEvent>,
    shunned: bool,
}

impl NetConnection {
    fn new(out: ws::Sender, server: flume::Sender<NetEvent>) -> Self {
        Self {
            id: out.connection_id(),
            out,
            server,
            shunned: false,
        }
    }

    fn shun(&mut self, close: bool) {
        if close {
            let _ = self.out.close(CloseCode::Normal);
        }

        if !self.shunned {
            self.server.send(NetEvent::Disconnect(self.id))
                .expect("Server channel disconnected");
        }

        self.shunned = true;
    }
}

impl Handler for NetConnection {
    fn on_open(&mut self, _: Handshake) -> ws::Result<()> {
        self.server.send(NetEvent::Connect(
            self.id,
            Responder {
                sender: self.out.clone()
            }
        )).expect("Server channel disconnected");

        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> ws::Result<()> {
        if self.shunned {
            return Ok(());
        }

        if let Message::Text(string) = msg {
            let mut pieces = Pieces::new(&string);

            loop {
                let message;
                match next_message(&mut pieces) {
                    None => break,
                    Some(Err(e)) => {
                        eprintln!("Error while parsing message from client #{}: {:?}", self.id, e);
                        eprintln!("The (entire) packet containing the bad message follows:");
                        eprintln!("=======================");
                        eprintln!("{}", string);
                        eprintln!("=======================");
                        break;
                    },
                    Some(Ok(m)) => message = m,
                }

                self.server.send(NetEvent::Message(
                    self.id,
                    message
                )).expect("Server channel disconnected");
            }
        } else {
            eprintln!("Client #{} sent a binary message - killing it", self.id);
            self.shun(true);
        }

        Ok(())
    }

    fn on_close(&mut self, _: CloseCode, _: &str) {
        self.shun(false);
    }
}
