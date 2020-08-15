macro_rules! console_log {
    ($($t:tt)*) => {
        crate::bindings::log(&format_args!($($t)*).to_string())
    };
}
