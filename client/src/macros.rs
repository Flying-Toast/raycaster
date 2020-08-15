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
