use crate::error::*;
use std::str::{Lines, FromStr};


/// Abstraction around `std::str::Lines` for parsing payloads
pub struct Pieces<'a> {
    lines: Lines<'a>,
    current_line: usize,
}

impl<'a> Pieces<'a> {
    pub fn new(raw: &'a str) -> Self {
        Self {
            lines: raw.lines(),
            current_line: 0,
        }
    }

    /// Parse the next line into a `&str`
    pub fn get_str(&mut self) -> Result<&str, RCE> {
        match self.lines.next() {
            Some(s) => {
                self.current_line += 1;
                Ok(s)
            },
            None => Err(RCE::EmptyPieces),
        }
    }

    /// Parse the next line into `T`
    pub fn get<T: FromStr>(&mut self) -> Result<T, RCE> {
        let line = self.get_str()?;
        T::from_str(line).to(RCE::PayloadParse {
                                    attempted_parse_type: std::any::type_name::<T>(),
                                    packet_line_num: self.current_line,
                                  })
    }
}

/// server-to-client payload
pub trait S2CPayload {
    fn encode(&self) -> String;
}

/// client-to-server payload
pub trait C2SPayload {
    fn parse(pieces: &mut Pieces) -> Result<Self, RCE> where Self: Sized;
}
