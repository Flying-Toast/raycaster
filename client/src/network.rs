use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::rc::Rc;
use std::cell::RefCell;
use common::protocol::{ServerMessage, next_message_from_server};
use common::protocol::payload::{Pieces, BuiltPayload};
use web_sys::{WebSocket, BinaryType, MessageEvent};
use js_sys::{ArrayBuffer, Uint8Array};


#[derive(Debug)]
pub enum NetworkStatus {
    Connecting,
    Connected,
    Disconnected,
}

pub struct Network {
    ws: Option<WebSocket>,
    message_queue: Rc<RefCell<Vec<ServerMessage>>>,
    onmessage_cb: Option<Closure<dyn FnMut(MessageEvent)>>,
    outgoing_queue: Vec<u8>,
}

impl Network {
    pub fn new() -> Self {
        Self {
            ws: None,
            message_queue: Rc::new(RefCell::new(Vec::new())),
            onmessage_cb: None,
            outgoing_queue: Vec::new(),
        }
    }

    pub fn status(&self) -> NetworkStatus {
        if let Some(ref ws) = self.ws {
            match ws.ready_state() {
                0 => NetworkStatus::Connecting,
                1 => NetworkStatus::Connected,
                2 | 3 => NetworkStatus::Disconnected,
                _ => unreachable!(),
            }
        } else {
            NetworkStatus::Disconnected
        }
    }

    pub fn drain_messages(&mut self) -> Vec<ServerMessage> {
        let mut queue = Vec::new();
        std::mem::swap(&mut *self.message_queue.borrow_mut(), &mut queue);

        queue
    }

    /// Queues a message
    pub fn send(&mut self, message: &BuiltPayload) {
        self.outgoing_queue.extend_from_slice(message.encode());
    }

    /// Sends all queued messages in a single packet
    pub fn flush(&mut self) {
        // don't send an empty message for empty packets
        if self.outgoing_queue.len() == 0 {
            return;
        }

        if let NetworkStatus::Connected = self.status() {
            let bytes = lz4_compress::compress(&self.outgoing_queue);
            self.ws.as_ref().unwrap().send_with_u8_array(&bytes).unwrap();
            self.outgoing_queue.clear();
        } else {
            console_error!("Tried to send to a closed websocket");
        }
    }

    pub fn connect(&mut self, host: &str, port: u16, use_tls: bool) {
        if let Some(_) = self.ws {
            panic!("Attemted to connect Network multiple times");
        }

        let protocol = if use_tls { "wss" } else { "ws" };
        let ws = WebSocket::new(&format!("{}://{}:{}", protocol, host, port)).unwrap();
        ws.set_binary_type(BinaryType::Arraybuffer);

        let queue_clone = self.message_queue.clone();

        self.onmessage_cb = Some(Closure::wrap(Box::new(move |msg: MessageEvent| {
            let message = msg.data().dyn_into::<ArrayBuffer>().unwrap();
            let bytes = Uint8Array::new(&message).to_vec();
            let bytes = match lz4_compress::decompress(&bytes) {
                Ok(b) => b,
                Err(_) => panic!("Error decompressing packet"),
            };
            let mut pieces = Pieces::new(&bytes);

            loop {
                let message;
                match next_message_from_server(&mut pieces) {
                    None => break,
                    Some(Err(e)) => {
                        panic!(concat!(
                            "Error while parsing message from server: {:?}\n",
                            "The (entire) packet containing the error:\n",
                            "======START======\n",
                            "{:#?}\n",
                            "=======END======="
                        ), e, bytes);
                    },
                    Some(Ok(m)) => message = m,
                }

                queue_clone.borrow_mut().push(message);
            }
        }) as Box<dyn FnMut(MessageEvent)>));

        ws.set_onmessage(Some(self.onmessage_cb.as_ref().unwrap().as_ref().unchecked_ref()));

        self.ws = Some(ws);
    }
}

impl Drop for Network {
    fn drop(&mut self) {
        if let Some(ref ws) = self.ws {
            if !matches!(self.status(), NetworkStatus::Disconnected) {
                let _ = ws.close();
            }
        }
    }
}
