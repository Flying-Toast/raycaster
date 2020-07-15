#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct EntityID(u32);

// for encoding in payloads
impl ToString for EntityID {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

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
