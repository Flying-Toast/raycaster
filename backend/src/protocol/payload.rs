use crate::error::*;
use std::str::Lines;


pub struct Pieces<'a> {
    lines: Lines<'a>,
}

impl<'a> Pieces<'a> {
    pub fn new(raw: &'a str) -> Self {
        Self { lines: raw.lines() }
    }

    pub fn get_str(&mut self) -> Result<&str, RCE> {
        match self.lines.next() {
            Some(s) => Ok(s),
            None => Err(RCE::EmptyPieces),
        }
    }
}

/// Server-to-Client payload
pub trait S2CPayload {
    fn msg_key() -> &'static str;
    fn encode(&self) -> String;
}

/// Client-to-Server payload
pub trait C2SPayload {
    fn parse(pieces: &mut Pieces) -> Result<Self, RCE> where Self: std::marker::Sized;
}
