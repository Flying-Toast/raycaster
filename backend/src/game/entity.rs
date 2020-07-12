#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct EntityID(u32);

impl EntityID {
    pub fn new(value: u32) -> Self {
        Self(value)
    }
}

pub struct Entity {
}

impl Entity {
    pub fn new() -> Self {
        Self {
        }
    }
}
