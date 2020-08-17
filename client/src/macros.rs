macro_rules! console_log {
    ($($t:tt)*) => {
        web_sys::console::log_1(&wasm_bindgen::JsValue::from(&format_args!($($t)*).to_string()))
    };
}

macro_rules! console_error {
    ($($t:tt)*) => {
        web_sys::console::error_1(&wasm_bindgen::JsValue::from(&format_args!($($t)*).to_string()))
    };
}

#[allow(unused_macros)]
macro_rules! println {
    ($($t:tt)*) => {
        compile_error!("Don't use println in wasm (use console_log)");
    }
}
