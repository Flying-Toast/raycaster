#[macro_use]
mod macros;
mod bindings;
mod frontend;

use wasm_bindgen::prelude::*;


#[wasm_bindgen(start)]
pub fn start() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let f = frontend::Frontend::new();
    f.start();
}
