#[macro_use]
mod macros;
mod frontend;
mod network;
mod game;
mod controls;
mod renderer;

use wasm_bindgen::prelude::*;


#[wasm_bindgen(start)]
pub fn start() {
    std::panic::set_hook(Box::new(|info| {
        console_error!("{}", info);
    }));

    let f = frontend::Frontend::new();
    f.start();
}
