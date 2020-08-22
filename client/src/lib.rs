#[macro_use]
mod macros;
mod frontend;
mod network;
mod game;

use wasm_bindgen::prelude::*;


fn panic_hook(info: &std::panic::PanicInfo) {
    console_error!("{}", info);
}

#[wasm_bindgen(start)]
pub fn start() {
    std::panic::set_hook(Box::new(panic_hook));

    let f = frontend::Frontend::new();
    f.start();
}
