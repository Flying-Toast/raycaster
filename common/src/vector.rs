use crate::error::*;
use crate::protocol::payload::{Pieces, PayloadBuilder, Encodable, Decodable};


#[derive(Debug)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl Encodable for &Vector {
    fn encode_to(self, builder: &mut PayloadBuilder) {
        builder.add(self.x);
        builder.add(self.y);
    }
}

impl Decodable for Vector {
    fn decode_from(pieces: &mut Pieces) -> Result<Self, CME> {
        Ok(
            Self::new(pieces.get()?, pieces.get()?)
        )
    }
}
