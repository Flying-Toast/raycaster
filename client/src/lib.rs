#[macro_use]
mod macros;
mod bindings;
mod frontend;

use wasm_bindgen::prelude::*;


#[wasm_bindgen(start)]
pub fn start() {
    let f = frontend::Frontend::new();
    f.start();
}
