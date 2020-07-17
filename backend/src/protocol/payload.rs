use crate::error::*;
use std::str::Lines;


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

    pub fn get_u32(&mut self) -> Result<u32, RCE> {
        self.get_str()?
            .parse()
            .to(self.parse_error::<u32>())
    }

    fn parse_error<T>(&self) -> RCE {
        RCE::PayloadParse {
            attempted_parse_type: std::any::type_name::<T>(),
            packet_line_num: self.current_line,
        }
    }
}

/// Builds an encoded payload for a packet
pub struct PayloadBuilder {
    lines: String,
}

impl PayloadBuilder {
    pub fn new(payload_key: &str) -> Self {
        Self {
            lines: String::from(payload_key),
        }
    }

    pub fn add_str(&mut self, string: &str) {
        self.lines.push('\n');
        self.lines.push_str(string);
    }

    pub fn build(self) -> String {
        self.lines
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
