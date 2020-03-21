use std::sync::mpsc;
use ws::{Handler, Message, Handshake, CloseCode, listen};


const SEND_FAIL_MSG: &str = "Failed to send a message to the server thread";

/// Information that gets sent to the server thread.
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
    /// For sending messages to the server thread.
    server: mpsc::Sender<NetMessage>,
    shunned: bool,
}

impl NetConnection {
    /// Ignores all future messages from the websocket and attempts to close the connection.
    fn shun(&mut self) {
        let _ = self.out.close(CloseCode::Policy);
        self.shunned = true;
        self.server.send(NetMessage::Disconnect(self.out.connection_id()))
            .expect(SEND_FAIL_MSG);
    }
}

impl Handler for NetConnection {
    fn on_open(&mut self, _: Handshake) -> ws::Result<()> {
        self.server.send(NetMessage::Connection(
            self.out.connection_id(),
            Responder {
                sender: self.out.clone()
            }
        )).expect(SEND_FAIL_MSG);

        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> ws::Result<()> {
        if self.shunned {
            return Ok(());
        }

        if let Message::Text(message) = msg {
            self.server.send(NetMessage::Message(
                self.out.connection_id(),
                message
            )).expect(SEND_FAIL_MSG);
        } else { //the client sent a binary message
            self.shun();
        }

        Ok(())
    }

    fn on_close(&mut self, _: CloseCode, _: &str) {
        self.shun();
    }
}

/// Runs the websocket network in this thread.
/// `server_tx` is the transmitting end of a channel for sending `NetMessage`s to the server thread.
pub fn start_network(host: &str, server_tx: mpsc::Sender<NetMessage>) -> Result<(), String> {
    listen(host, |sender| {
        NetConnection {
            out: sender,
            server: server_tx.clone(),
            shunned: false,
        }
    }).or(Err("Failed to start the websocket listener".to_string()))
}
