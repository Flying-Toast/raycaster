use std::mem;
use std::convert::TryInto;
use crate::error::*;
use crate::entity::{Entity, EntityID};
use crate::vector::Vector;


/// Parses incoming payloads
#[derive(Debug)]
pub struct Pieces<'a> {
    bytes: &'a [u8],
}

impl<'a> Pieces<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self {
            bytes,
        }
    }

    /// Parse the next `String`
    pub fn string(&mut self) -> Result<String, CME> {
        let string_len = self.u32()?;
        let bytes = self.bytes_from_front(string_len as usize)?;
        let string = String::from_utf8(bytes.to_vec())
            .map_err(|e| CME::BadString{bytes: e.into_bytes()})?;

        Ok(string)
    }

    /// Parse the next `u32`
    pub fn u32(&mut self) -> Result<u32, CME> {
        type Thing = u32;
        Ok(Thing::from_be_bytes(
            self.bytes_from_front(mem::size_of::<Thing>())?
                .try_into()
                .unwrap()
        ))
    }

    /// Parse the next `u16`
    pub fn u16(&mut self) -> Result<u16, CME> {
        type Thing = u16;
        Ok(Thing::from_be_bytes(
            self.bytes_from_front(mem::size_of::<Thing>())?
                .try_into()
                .unwrap()
        ))
    }

    pub fn f32(&mut self) -> Result<f32, CME> {
        type Thing = f32;
        Ok(Thing::from_be_bytes(
            self.bytes_from_front(mem::size_of::<Thing>())?
                .try_into()
                .unwrap()
        ))
    }

    pub fn ent_id(&mut self) -> Result<EntityID, CME> {
        Ok(
            EntityID::new(self.u32()?)
        )
    }

    pub fn vector(&mut self) -> Result<Vector, CME> {
        Ok(
            Vector::new(self.f32()?, self.f32()?)
        )
    }

    pub fn entity(&mut self) -> Result<Entity, CME> {
        Ok(
            Entity::new(self.ent_id()?, self.vector()?)
        )
    }

    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Removes the first `num` bytes from `self.bytes` and returns the removed bytes.
    fn bytes_from_front(&mut self, num: usize) -> Result<&[u8], CME> {
        if self.bytes.len() < num {
            Err(CME::NotEnoughBytes{requested: num, available: self.bytes.len()})
        } else {
            let (front, back) = self.bytes.split_at(num);
            self.bytes = back;

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

    /// Adds a `&str` to the payload
    pub fn string(&mut self, string: &str) {
        let str_len = string.len() as u32;
        self.u32(str_len);
        self.bytes.extend_from_slice(string.as_bytes());
    }

    /// Adds a `u32` to the payload
    pub fn u32(&mut self, int: u32) {
        self.bytes.extend_from_slice(&int.to_be_bytes());
    }

    /// Adds a `f32` to the payload
    pub fn f32(&mut self, float: f32) {
        self.bytes.extend_from_slice(&float.to_be_bytes());
    }

    /// Adds an `EntityID` to the payload
    pub fn ent_id(&mut self, id: EntityID) {
        self.u32(id.0);
    }

    /// Adds an `Entity` to the payload
    pub fn entity(&mut self, entity: &Entity) {
        self.ent_id(entity.id);
        self.vector(entity.location());
    }

    /// Adds a `Vector` to the payload
    pub fn vector(&mut self, vector: &Vector) {
        self.f32(vector.x);
        self.f32(vector.y);
    }

    pub fn build(self) -> BuiltPayload {
        BuiltPayload(self.bytes)
    }
}

pub trait Payload {
    fn parse(pieces: &mut Pieces) -> Result<Self, CME> where Self: Sized;
}
