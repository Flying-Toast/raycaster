use std::collections::HashMap;
use crate::map::Map;
use crate::entity::{Entity, EntityID};


pub struct GameState {
    pub map: Map,
    entities: HashMap<EntityID, Entity>,
}

impl GameState {
    pub fn new(map: Map) -> Self {
        Self {
            map,
            entities: HashMap::new(),
        }
    }

    pub fn add_entity(&mut self, id: EntityID, entity: Entity) {
        self.entities.insert(id, entity);
    }

    pub fn remove_entity(&mut self, id: EntityID) -> Option<Entity> {
        self.entities.remove(&id)
    }

    pub fn entities(&self) -> impl Iterator<Item=&Entity> {
        self.entities.values()
    }
}
