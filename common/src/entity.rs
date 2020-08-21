use crate::vector::Vector;
use crate::error::*;
use crate::protocol::payload::{Pieces, PayloadBuilder, Encodable, Decodable};


#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct EntityID(pub u32);

impl EntityID {
    pub fn new(value: u32) -> Self {
        Self(value)
    }
}

impl Encodable for EntityID {
    fn encode_to(self, builder: &mut PayloadBuilder) {
        builder.add(self.0);
    }
}

impl Decodable for EntityID {
    fn decode_from(pieces: &mut Pieces) -> Result<Self, CME> {
        Ok(
            EntityID::new(pieces.get()?)
        )
    }
}

#[derive(Debug)]
pub struct Entity {
    location: Vector,
    id: EntityID,
}

impl Entity {
    pub fn new(id: EntityID, location: Vector) -> Self {
        Self {
            location,
            id,
        }
    }

    pub fn location(&self) -> &Vector {
        &self.location
    }

    pub fn id(&self) -> EntityID {
        self.id
    }
}

impl Encodable for &Entity {
    fn encode_to(self, builder: &mut PayloadBuilder) {
        builder.add(self.id());
        builder.add(self.location());
    }
}

impl Decodable for Entity {
    fn decode_from(pieces: &mut Pieces) -> Result<Self, CME> {
        let id: EntityID = pieces.get()?;
        let location: Vector = pieces.get()?;

        Ok(
            Entity::new(id, location)
        )
    }
}
