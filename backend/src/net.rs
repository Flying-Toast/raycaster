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
    let (resp_tx, mut resp_rx) = flume::unbounded();

    tx.send(NetEvent::Connect(ClientID(id), Responder::new(resp_tx)))
        .expect("Server channel disconnected");

    // future that waits for messages from the `Responder` and forwards them to the websocket
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

        // this future always returns Ok, so that it wont stop the try_join
        Result::<(), ()>::Ok(())
    };

    let tx2 = tx.clone();
    //future that forwards messages received from the websocket to the server thread
    let net_events = async move {
        while let Some(message) = incoming.next().await {
            if let Ok(Message::Text(string)) = message {
                let mut pieces = Pieces::new(&string);

                loop {
                    let message;
                    match next_message(&mut pieces) {
                        None => break,
                        Some(Err(e)) => {
                            eprintln!("Error while parsing message from client #{}: {:?}", id, e);
                            eprintln!("The (entire) packet containing the bad message follows:");
                            eprintln!("=========START=========");
                            eprintln!("{}", string);
                            eprintln!("==========END==========");
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
    Message(String),
    /// Instructs the net thread to close the websocket.
    Close,
}

pub struct Responder {
    net_tx: flume::Sender<ServerEvent>,
    queue: Vec<String>,
}

impl Responder {
    fn new(net_tx: flume::Sender<ServerEvent>) -> Self {
        Self {
            net_tx,
            queue: Vec::new(),
        }
    }

    /// Queues a payload
    pub fn send(&mut self, payload: impl S2CPayload) {
        self.queue.push(payload.encode().build());
    }

    /// Sends all queued payloads in a single packet
    pub fn flush(&mut self) {
        // don't send an empty string for empty packets
        if self.queue.len() == 0 {
            return;
        }

        let _ = self.net_tx.send(ServerEvent::Message(self.queue.join("\n")));
        self.queue.clear();
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
