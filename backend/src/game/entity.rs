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
}

impl Entity {
    pub fn new(location: Vector) -> Self {
        Self {
            location,
        }
    }

    pub fn location(&self) -> &Vector {
        &self.location
    }
}
