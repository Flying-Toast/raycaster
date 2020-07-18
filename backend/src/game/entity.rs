use crate::game::vector::Vector;


#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct EntityID(pub u32);

impl EntityID {
    pub fn new(value: u32) -> Self {
        Self(value)
    }
}

pub struct Entity {
    location: Vector,
    pub id: EntityID,
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
}
