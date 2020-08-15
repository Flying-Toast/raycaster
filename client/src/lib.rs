#[macro_use]
mod macros;
mod bindings;

use wasm_bindgen::prelude::*;


#[wasm_bindgen(start)]
pub fn start() {
    console_log!("HELLO HELLO HELLO HELLO");
    console_error!("oopsie");
}
