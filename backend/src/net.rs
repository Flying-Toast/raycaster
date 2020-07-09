use tokio::runtime::Runtime;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;
use futures_util::StreamExt;
use crate::error::*;
use crate::protocol::{ClientMessage, next_message};
use crate::protocol::payload::{S2CPayload, Pieces};


/// Runs the network server in this thread.
/// `server_tx` is the transmitting end of a channel for sending `NetEvent`s to the server thread.
pub fn start(server_tx: flume::Sender<NetEvent>, port: u16) -> Result<(), RCE> {
    Runtime::new().to(RCE::NetworkFailedToStart)?.block_on(async {
        let mut listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await
            .to(RCE::NetworkFailedToStart)?;

        while let Ok((stream, _)) = listener.accept().await {
            //TODO: GENERATE ID
            let id = 12345;
            tokio::spawn(handle_connection(stream, server_tx.clone(), id));
        }

        Err(RCE::NetworkClosed)
    })
}

async fn handle_connection(stream: TcpStream, tx: flume::Sender<NetEvent>, id: u32) {
    let ws_stream = match accept_async(stream).await {
                        Ok(s) => s,
                        Err(_) => return,
                    };

    let (outgoing, mut incoming) = ws_stream.split();

    tx.send(NetEvent::Connect(
        id,
        Responder {
        }
    )).expect("Server channel disconnected");

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

                    tx.send(NetEvent::Message(id, message))
                        .expect("Server channel disconnected");
                }
            },
            _ => {},
        }
    }

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

#[derive(Debug)]
pub struct Responder {
}

impl Responder {
    pub fn send(&mut self, payload: impl S2CPayload) {
    }
}
