pub use common::error::ErrTo;

/// "Backend Error" - error type
#[derive(Debug)]
pub enum BKE {
    NetworkFailedToStart,
}
