use std::fmt;
use tokio::runtime::Runtime;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;
use futures_util::{StreamExt, SinkExt};
use crate::error::*;
use crate::protocol::{ClientMessage, next_message};
use crate::protocol::payload::{S2CPayload, Pieces};


/// Runs the network server in this thread.
/// `server_tx` is the transmitting end of a channel for sending `NetEvent`s to the server thread.
pub fn start(server_tx: flume::Sender<NetEvent>, port: u16) -> Result<(), RCE> {
    Runtime::new().to(RCE::NetworkFailedToStart)?.block_on(async {
        let address = format!("0.0.0.0:{}", port);
        let mut listener = TcpListener::bind(&address).await
            .to(RCE::NetworkFailedToStart)?;
        eprintln!("Listening on {}", address);

        let mut current_id: u32 = 0;
        loop {
            match listener.accept().await {
                Ok((stream, _)) => {
                    tokio::spawn(handle_connection(stream, server_tx.clone(), current_id));
                    current_id = current_id.overflowing_add(1).0;
                },
                Err(e) => eprintln!("Error accepting client (would be #{}): {}", current_id + 1, e),
            }
        }
    })
}

async fn handle_connection(stream: TcpStream, tx: flume::Sender<NetEvent>, id: u32) {
    let ws_stream = match accept_async(stream).await {
                        Ok(s) => s,
                        Err(e) => {
                            eprintln!("Error with websocket handshake (client #{}): {}", id, e);
                            return;
                        },
                    };

    let (mut outgoing, mut incoming) = ws_stream.split();

    let (resp_tx, mut resp_rx) = flume::unbounded();

    tx.send(NetEvent::Connect(id, Responder::new(resp_tx)))
        .expect("Server channel disconnected");

    let server_events = async move {
        while let Ok(event) = resp_rx.recv_async().await {
            match event {
                ServerEvent::Message(string) => {
                    if let Err(e) = outgoing.send(Message::Text(string)).await {
                        eprintln!("Error sending to client #{}: {} - disconnecting", id, e);
                        let _ = outgoing.close().await;
                        return Ok(());
                    }
                },
                ServerEvent::Close => {
                    let _ = outgoing.close().await;
                    return Ok(());
                },
            }
        }

        eprintln!("Client #{}'s `Responder` was dropped without explicitly disconnecting - disconnecting now", id);
        let _ = outgoing.close().await;

        Result::<(), ()>::Ok(())
    };

    let tx2 = tx.clone();
    let net_events = async move {
        while let Some(message) = incoming.next().await {
            match message {
                Ok(Message::Text(string)) => {
                    let mut pieces = Pieces::new(&string);

                    loop {
                        let message;
                        match next_message(&mut pieces) {
                            None => break,
                            Some(Err(e)) => {
                                eprintln!("Error while parsing message from client #{}: {:?}", id, e);
                                eprintln!("The (entire) packet containing the bad message follows:");
                                eprintln!("=======================");
                                eprintln!("{}", string);
                                eprintln!("=======================");
                                break;
                            },
                            Some(Ok(m)) => message = m,
                        }

                        tx2.send(NetEvent::Message(id, message))
                            .expect("Server channel disconnected");
                    }
                },
                _ => {},
            }
        }

        Result::<(), ()>::Err(())
    };

    let _ = futures_util::try_join!(server_events, net_events);

    tx.send(NetEvent::Disconnect(id))
        .expect("Server channel disconnected");
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

/// Sent from the server thread to the net thread
#[derive(Debug)]
enum ServerEvent {
    /// An outgoing message.
    Message(String),
    /// Instructs the net thread to close the websocket.
    Close,
}

pub struct Responder {
    net_tx: flume::Sender<ServerEvent>,
}

impl Responder {
    pub fn send(&mut self, payload: impl S2CPayload) -> Result<(), RCE> {
        self.net_tx.send(ServerEvent::Message(payload.encode())).to(RCE::NetworkSend)
    }

    pub fn close(&mut self) {
        let _ = self.net_tx.send(ServerEvent::Close);
    }

    fn new(net_tx: flume::Sender<ServerEvent>) -> Self {
        Self {
            net_tx,
        }
    }
}

impl fmt::Debug for Responder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Responder")
         .field("net_tx", &"flume::Sender<ServerEvent>")
         .finish()
    }
}
