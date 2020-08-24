use std::mem;
use std::convert::TryInto;
use crate::error::*;


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

    pub fn get<T: Decodable>(&mut self) -> Result<T, CME> {
        T::decode_from(self)
    }

    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Removes the first `num` bytes from `self.bytes` and returns the removed bytes.
    pub fn bytes_from_front(&mut self, num: usize) -> Result<&[u8], CME> {
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

    pub fn add<T: Encodable>(&mut self, thing: T) {
        thing.encode_to(self);
    }

    pub fn extend(&mut self, bytes: &[u8]) {
        self.bytes.extend_from_slice(bytes);
    }

    pub fn build(self) -> BuiltPayload {
        BuiltPayload(self.bytes)
    }
}

/// Something that can be put in a `PayloadBuilder`
pub trait Encodable {
    fn encode_to(self, builder: &mut PayloadBuilder);
}

/// Something that can be parsed from a `Pieces`
pub trait Decodable {
    fn decode_from(pieces: &mut Pieces) -> Result<Self, CME> where Self: Sized;
}

macro_rules! codable_primatives {
    ($($primative:ty),*$(,)?) => {
        $(
            impl Encodable for &$primative {
                fn encode_to(self, builder: &mut PayloadBuilder) {
                    builder.extend(&self.to_be_bytes());
                }
            }

            impl Decodable for $primative {
                fn decode_from(pieces: &mut Pieces) -> Result<Self, CME> {
                    Ok(<$primative>::from_be_bytes(
                        pieces.bytes_from_front(mem::size_of::<$primative>())?
                            .try_into()
                            .unwrap()
                    ))
                }
            }
        )*
    };
}

codable_primatives!(u8, u16, u32, f32);

impl Encodable for &str {
    fn encode_to(self, builder: &mut PayloadBuilder) {
        let length = self.len() as u32;
        builder.add(&length);
        builder.extend(self.as_bytes());
    }
}

impl Decodable for String {
    fn decode_from(pieces: &mut Pieces) -> Result<Self, CME> {
        let length: u32 = pieces.get()?;
        let bytes = pieces.bytes_from_front(length as usize)?;
        let string = String::from_utf8(bytes.to_vec())
            .map_err(|e| CME::BadString{bytes: e.into_bytes()})?;

        Ok(string)
    }
}
