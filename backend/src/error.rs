#[derive(Debug)]
pub enum RCE {
    NetworkFailedToStart,
    BadTileType,
    BadMapFormat,
    /// There was a problem reading the map file
    MapFileRead,
}

pub trait ErrTo<T> {
    fn to(self, to: RCE) -> Result<T, RCE>;
}

impl<A, _B> ErrTo<A> for Result<A, _B> {
    fn to(self, to: RCE) -> Result<A, RCE> {
        self.ok().ok_or(to)
    }
}

impl<T> ErrTo<T> for Option<T> {
    fn to(self, to: RCE) -> Result<T, RCE> {
        self.ok_or(to)
    }
}
