/// Error type
#[derive(Debug)]
pub enum RCE {
    // network errors
    NetworkFailedToStart,

    // protocol errors
    BadClientMessageType,
    /// An error when trying to parse a line from a payload string.
    /// `attempted_parse_type` is the name of the type that couldn't be parsed.
    /// `packet_line_num` is the line number within the packet that caused the parsing error.
    PayloadParse{attempted_parse_type: &'static str, packet_line_num: usize},
    EmptyPieces,

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
