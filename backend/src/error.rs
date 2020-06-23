#[derive(Debug)]
pub enum RCE {
    NetworkFailedToStart,
    BadTileType,
    BadMapFormat,
    /// There was a problem reading the map file
    MapFileRead,
    PayloadDecode,
    BadClientMessageType,
    EmptyPieces,
}

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
