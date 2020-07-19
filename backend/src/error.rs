/// Error type
#[derive(Debug)]
pub enum RCE {
    // network errors
    NetworkFailedToStart,

    // protocol errors
    BadClientMessageType,
    /// Tried to parse a type which requires more bytes than
    /// there are left in the packet.
    NotEnoughBytes,
    /// Invalid string (the bytes were not valid UTF-8)
    BadString,

    // map parsing errors
    MapFileRead,
    BadMapFormat{line_num: usize},
    BadTileType{type_string: String},
}

/// Converts `Result<T, _>` to `Result<T, RCE>`. Also implemented on `Option`.
pub trait ErrTo<T> {
    fn to(self, to: RCE) -> Result<T, RCE>;
}

impl<A, _B> ErrTo<A> for Result<A, _B> {
    fn to(self, to: RCE) -> Result<A, RCE> {
        match self {
            Ok(x) => Ok(x),
            Err(_) => Err(to),
        }
    }
}

impl<T> ErrTo<T> for Option<T> {
    fn to(self, to: RCE) -> Result<T, RCE> {
        match self {
            Some(x) => Ok(x),
            None => Err(to),
        }
    }
}
