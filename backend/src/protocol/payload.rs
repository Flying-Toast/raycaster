use crate::error::*;
use std::str::{Lines, FromStr};


/// Abstraction around `std::str::Lines` for parsing payloads
pub struct Pieces<'a> {
    lines: Lines<'a>,
}

impl<'a> Pieces<'a> {
    pub fn new(raw: &'a str) -> Self {
        Self { lines: raw.lines() }
    }

    /// Parse the next line into a `&str`
    pub fn get_str(&mut self) -> Result<&str, RCE> {
        match self.lines.next() {
            Some(s) => Ok(s),
            None => Err(RCE::EmptyPieces),
        }
    }

    /// Parse the next line into `T`
    pub fn generic_get<T: FromStr>(&mut self) -> Result<T, RCE> {
        T::from_str(self.get_str()?).to(RCE::PayloadDecode)
    }
}

/// server-to-client payload
pub trait S2CPayload {
    fn payload_key() -> &'static str;
    fn encode(&self) -> String;
}

/// client-to-server payload
pub trait C2SPayload {
    fn parse(pieces: &mut Pieces) -> Result<Self, RCE> where Self: std::marker::Sized;
}
