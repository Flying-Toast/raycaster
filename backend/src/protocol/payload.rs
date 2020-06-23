use crate::error::*;
use std::str::Lines;


/// Server-to-Client payload
pub trait S2CPayload {
    fn msg_key() -> &'static str;
    fn encode(&self) -> String;
}

/// Client-to-Server payload
pub trait C2SPayload {
    fn parse(lines: &mut Lines) -> Result<Self, RCE> where Self: std::marker::Sized;
}
