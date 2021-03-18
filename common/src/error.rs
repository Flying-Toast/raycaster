/// "Common Error" - error type
#[derive(Debug)]
pub enum CME {
    BadClientMessageType{payload_key: u8},
    BadServerMessageType{payload_key: u8},
    /// Tried to parse a type which requires more bytes than
    /// there are left in the packet.
    /// `requested` is how many bytes the read operation requested,
    /// `available` is how many bytes there were available.
    NotEnoughBytes{requested: usize, available: usize},
    /// Invalid string (the bytes were not valid UTF-8)
    BadString{bytes: Vec<u8>},
    BadMapFormat{line_num: usize},
    BadTileType{type_string: String},
    /// The parsed `Map` does not have walls all around the edges
    NonClosedMap,
    /// All maps need to be at least 3x3
    MapTooSmall,
}

/// Converts `Result<T, _>` to `Result<T, E>`. Also implemented on `Option`.
pub trait ErrTo<T, E> {
    fn to(self, to: E) -> Result<T, E>;
}

impl<A, E, _B> ErrTo<A, E> for Result<A, _B> {
    fn to(self, to: E) -> Result<A, E> {
        match self {
            Ok(x) => Ok(x),
            Err(_) => Err(to),
        }
    }
}

impl<T, E> ErrTo<T, E> for Option<T> {
    fn to(self, to: E) -> Result<T, E> {
        match self {
            Some(x) => Ok(x),
            None => Err(to),
        }
    }
}
