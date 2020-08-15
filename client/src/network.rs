use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::rc::Rc;
use std::cell::RefCell;
use common::protocol::{ServerMessage, next_message_from_server};
use common::protocol::payload::Pieces;
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
    onmessage_cb: Rc<RefCell<Option<Closure<dyn FnMut(MessageEvent)>>>>,
}

impl Network {
    pub fn new() -> Self {
        Self {
            ws: None,
            message_queue: Rc::new(RefCell::new(Vec::new())),
            onmessage_cb: Rc::new(RefCell::new(None)),
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
        self.message_queue.borrow_mut().drain(..).collect()
    }

    pub fn connect(&mut self, host: &str, port: u16, use_tls: bool) {
        if let Some(_) = self.ws {
            panic!("Attemted to connect Network multiple times");
        }

        let protocol = if use_tls { "wss" } else { "ws" };
        let ws = WebSocket::new(&format!("{}://{}:{}", protocol, host, port)).unwrap();
        ws.set_binary_type(BinaryType::Arraybuffer);

        let queue_clone = self.message_queue.clone();

        *self.onmessage_cb.borrow_mut() = Some(Closure::wrap(Box::new(move |msg: MessageEvent| {
            let message = msg.data().dyn_into::<ArrayBuffer>().unwrap();
            let bytes = Uint8Array::new(&message).to_vec();
            let mut pieces = Pieces::new(&bytes);

            loop {
                let message;
                match next_message_from_server(&mut pieces) {
                    None => break,
                    Some(Err(e)) => {
                        console_error!("Error while parsing message from server: {:?}", e);
                        console_error!("The (entire) packet containing the error:");
                        console_error!("======START======");
                        console_error!("{:#?}", bytes);
                        console_error!("=======END=======");
                        break;
                    },
                    Some(Ok(m)) => message = m,
                }

                queue_clone.borrow_mut().push(message);
            }
        }) as Box<dyn FnMut(MessageEvent)>));

        ws.set_onmessage(Some(self.onmessage_cb.borrow().as_ref().unwrap().as_ref().unchecked_ref()));

        self.ws = Some(ws);
    }
}
