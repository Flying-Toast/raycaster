pub use common::error::ErrTo;

/// "Backend Error" - error type
#[derive(Debug)]
pub enum BKE {
    // network errors

    NetworkFailedToStart,


    // map parsing errors

    MapFileRead,
    BadMapFormat{line_num: usize},
    BadTileType{type_string: String},
}
