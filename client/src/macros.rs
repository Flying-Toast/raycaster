macro_rules! console_log {
    ($($t:tt)*) => {
        crate::bindings::log(&format_args!($($t)*).to_string())
    };
}

macro_rules! console_error {
    ($($t:tt)*) => {
        crate::bindings::error(&format_args!($($t)*).to_string())
    };
}

macro_rules! println {
    ($($t:tt)*) => {
        compile_error!("Don't use println in wasm (use console_log)");
    }
}
