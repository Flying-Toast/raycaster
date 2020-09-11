use std::fmt;
use std::thread;
use tokio::runtime::Runtime;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;
use futures_util::{StreamExt, SinkExt};
use crate::error::*;
use common::protocol::{ClientMessage, next_message_from_client};
use common::protocol::payload::{BuiltPayload, Pieces};

/// Starts the network in a new thread.
/// `server_tx` is the transmitting end of a channel for sending `NetEvent`s to the server thread.
pub fn start(server_tx: flume::Sender<NetEvent>, port: u16) {
    thread::Builder::new()
        .name("network".to_string())
        .spawn(move || {
            run(server_tx, port).unwrap();
        }).unwrap();
}

fn run(server_tx: flume::Sender<NetEvent>, port: u16) -> Result<(), BKE> {
    Runtime::new().to(BKE::NetworkFailedToStart)?.block_on(async {
        let address = format!("0.0.0.0:{}", port);
        let mut listener = TcpListener::bind(&address).await
            .to(BKE::NetworkFailedToStart)?;
        println!("Listening on {}", address);

        let mut current_id: u32 = 0;
        loop {
            match listener.accept().await {
                Ok((stream, _)) => {
                    tokio::spawn(handle_connection(stream, server_tx.clone(), current_id));
                    current_id = current_id.wrapping_add(1);
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

    // channel for the `Responder` to send things to this websocket
    let (resp_tx, resp_rx) = flume::unbounded();

    tx.send(NetEvent::Connect(ClientID(id), Responder::new(resp_tx)))
        .expect("Server channel disconnected");

    // future that waits for messages from the `Responder` and forwards them to the websocket
    let server_events = async move {
        while let Ok(event) = resp_rx.recv_async().await {
            match event {
                ServerEvent::Message(bytes) => {
                    let bytes = lz4_compress::compress(&bytes);
                    if let Err(e) = outgoing.send(Message::Binary(bytes)).await {
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

        // this future always returns Ok, so that it wont stop the try_join
        Result::<(), ()>::Ok(())
    };

    let tx2 = tx.clone();
    //future that forwards messages received from the websocket to the server thread
    let net_events = async move {
        while let Some(message) = incoming.next().await {
            if let Ok(Message::Binary(bytes)) = message {
                let bytes = match lz4_compress::decompress(&bytes) {
                    Ok(b) => b,
                    Err(_) => {
                        eprintln!("Error decompressing packet from client #{}", id);
                        break;
                    },
                };
                let mut pieces = Pieces::new(&bytes);

                loop {
                    let message;
                    match next_message_from_client(&mut pieces) {
                        None => break,
                        Some(Err(e)) => {
                            eprintln!(concat!(
                                "Error while parsing message from client #{}: {:?}\n",
                                "The (entire) packet containing the error:\n",
                                "======START======\n",
                                "{:#?}\n",
                                "=======END======="
                            ), id, e, bytes);
                            break;
                        },
                        Some(Ok(m)) => message = m,
                    }

                    tx2.send(NetEvent::Message(ClientID(id), message))
                        .expect("Server channel disconnected");
                }
            }
        }

        // stop the try_join once the websocket is closed and all pending incoming
        // messages have been sent to the game thread.
        // stopping the try_join causes server_events to be closed too so that the
        // `Receiver` cant send any more messages.
        Result::<(), ()>::Err(())
    };

    // use try_join so that when net_events returns Err (the websocket closes), server_events will be stopped too
    let _ = futures_util::try_join!(server_events, net_events);

    tx.send(NetEvent::Disconnect(ClientID(id)))
        .expect("Server channel disconnected");
}

/// ID that is unique among clients (websockets).
/// `ClientID`s are only for the server's use - clients should know nothing about the existence of `ClientID`s.
/// Clients are only told what their player's *EntityID* is.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ClientID(u32);

/// Type that is sent over a channel the server thread.
#[derive(Debug)]
pub enum NetEvent {
    /// A client connected.
    /// The first field is an id that uniquely identifies this client.
    /// The second field is a `Responder` for sending messages back to the client.
    Connect(ClientID, Responder),
    /// A message from the client.
    /// The first field is the id of the client who sent this message.
    /// The second field is the message.
    Message(ClientID, ClientMessage),
    /// A client disconnected.
    /// The field is the id of the client who disconnected.
    Disconnect(ClientID),
}

/// Sent from the server thread to the net thread
#[derive(Debug)]
enum ServerEvent {
    /// An outgoing message.
    Message(Vec<u8>),
    /// Instructs the net thread to close the websocket.
    Close,
}

pub struct Responder {
    net_tx: flume::Sender<ServerEvent>,
    queue: Vec<u8>,
}

impl Responder {
    fn new(net_tx: flume::Sender<ServerEvent>) -> Self {
        Self {
            net_tx,
            queue: Vec::new(),
        }
    }

    /// Queues a payload
    pub fn send(&mut self, payload: &BuiltPayload) {
        self.queue.extend_from_slice(payload.encode());
    }

    /// Sends all queued payloads in a single packet
    pub fn flush(&mut self) {
        // don't send an empty message for empty packets
        if self.queue.len() == 0 {
            return;
        }

        let mut queue = Vec::new();
        std::mem::swap(&mut queue, &mut self.queue);
        let _ = self.net_tx.send(ServerEvent::Message(queue));
    }

    /// Sends any queued messages then closes the connection
    pub fn close(mut self) { // take owned self so that the Responder can't be used after closed
        // Send any queued messages before closing
        self.flush();
        let _ = self.net_tx.send(ServerEvent::Close);
    }
}

impl fmt::Debug for Responder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Responder")
         .field("net_tx", &"flume::Sender<ServerEvent>")
         .finish()
    }
}
