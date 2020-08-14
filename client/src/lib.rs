use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    WebSocket, MessageEvent,
};
use common::protocol::payloads::HelloPayload;

#[wasm_bindgen(start)]
pub fn start() {
    let ws = WebSocket::new("ws://localhost:8000").unwrap();
    let ws2 = ws.clone();
    let onopen_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
        let q = HelloPayload::assemble("👋Hello from the client!", 39);
        ws2.send_with_u8_array(q.encode());
    }) as Box<dyn FnMut(MessageEvent)>);
    ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    onopen_callback.forget();
}
