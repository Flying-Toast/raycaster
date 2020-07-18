use std::str::Lines;
use crate::error::*;
use crate::game::entity::EntityID;


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

/// An outgoing payload ready to be sent. Produced by a `PaylodBuilder`.
#[derive(Clone)]
pub struct BuiltPayload(String);

impl BuiltPayload {
    pub fn encode(self) -> String {
        self.0
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

    pub fn add_u32(&mut self, int: u32) {
        self.add_str(&int.to_string());
    }

    pub fn add_ent_id(&mut self, id: EntityID) {
        self.add_u32(id.0);
    }

    pub fn build(self) -> BuiltPayload {
        BuiltPayload(self.lines)
    }
}

/// client-to-server payload
pub trait C2SPayload {
    fn parse(pieces: &mut Pieces) -> Result<Self, RCE> where Self: Sized;
}
