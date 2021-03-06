use crate::vector::Vector;
use proc::Codable;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Codable)]
pub struct EntityID(u16);

impl EntityID {
    pub fn new(value: u16) -> Self {
        Self(value)
    }
}

#[derive(Debug, Codable, Clone)]
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

    pub fn id(&self) -> EntityID {
        self.id
    }
}
