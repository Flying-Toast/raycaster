use std::mem;
use std::convert::TryInto;
use crate::error::*;
use crate::game::entity::EntityID;
use crate::game::vector::Vector;


/// Parses incoming payloads
pub struct Pieces {
    bytes: Vec<u8>,
}

impl Pieces {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self {
            bytes,
        }
    }

    /// Parse the next `String`
    pub fn get_string(&mut self) -> Result<String, RCE> {
        let string_len = self.get_u64()?;
        let bytes = self.bytes_from_front(string_len as usize)?;
        let string = String::from_utf8(bytes).to(RCE::BadString)?;

        Ok(string)
    }

    pub fn get_u32(&mut self) -> Result<u32, RCE> {
        type Int = u32;
        Ok(Int::from_be_bytes(
            self.bytes_from_front(mem::size_of::<Int>())?
                .as_slice()
                .try_into()
                .unwrap()
        ))
    }

    pub fn get_u16(&mut self) -> Result<u16, RCE> {
        type Int = u16;
        Ok(Int::from_be_bytes(
            self.bytes_from_front(mem::size_of::<Int>())?
                .as_slice()
                .try_into()
                .unwrap()
        ))
    }

    pub fn get_u64(&mut self) -> Result<u64, RCE> {
        type Int = u64;
        Ok(Int::from_be_bytes(
            self.bytes_from_front(mem::size_of::<Int>())?
                .as_slice()
                .try_into()
                .unwrap()
        ))
    }

    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Removes the first `num` bytes from `self.bytes` and returns the removed bytes.
    fn bytes_from_front(&mut self, num: usize) -> Result<Vec<u8>, RCE> {
        if self.bytes.len() < num {
            Err(RCE::NotEnoughBytes)
        } else {
            let mut front = self.bytes.split_off(num);
            mem::swap(&mut front, &mut self.bytes);

            Ok(front)
        }
    }
}

/// An outgoing payload ready to be sent. Produced by a `PaylodBuilder`.
pub struct BuiltPayload(Vec<u8>);

impl BuiltPayload {
    pub fn encode(&self) -> &[u8] {
        &self.0
    }
}

/// Builds an encoded payload for a packet
pub struct PayloadBuilder {
    bytes: Vec<u8>,
}

impl PayloadBuilder {
    pub fn new(payload_key: u16) -> Self {
        Self {
            bytes: Vec::from(payload_key.to_be_bytes()),
        }
    }

    pub fn add_str(&mut self, string: &str) {
        let str_len = string.len() as u64;
        self.add_u64(str_len);
        self.bytes.extend_from_slice(string.as_bytes());
    }

    pub fn add_u32(&mut self, int: u32) {
        self.bytes.extend_from_slice(&int.to_be_bytes());
    }

    pub fn add_u64(&mut self, int: u64) {
        self.bytes.extend_from_slice(&int.to_be_bytes());
    }

    pub fn add_f32(&mut self, float: f32) {
        self.bytes.extend_from_slice(&float.to_be_bytes());
    }

    pub fn add_ent_id(&mut self, id: EntityID) {
        self.add_u32(id.0);
    }

    pub fn add_vector(&mut self, vector: &Vector) {
        self.add_f32(vector.x);
        self.add_f32(vector.y);
    }

    pub fn build(self) -> BuiltPayload {
        BuiltPayload(self.bytes)
    }
}

/// client-to-server payload
pub trait C2SPayload {
    fn parse(pieces: &mut Pieces) -> Result<Self, RCE> where Self: Sized;
}
