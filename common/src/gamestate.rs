use std::collections::HashMap;
use crate::map::Map;
use crate::entity::{Entity, EntityID};
use crate::input::Input;


#[derive(Debug)]
pub struct GameState {
    map: Map,
    entities: HashMap<EntityID, Entity>,
}

impl GameState {
    pub fn new(map: Map) -> Self {
        Self {
            map,
            entities: HashMap::new(),
        }
    }

    pub fn apply_input(&mut self, puppeteer: EntityID, input: &Input) {
    }

    pub fn set_map(&mut self, map: Map) {
        self.map = map;
    }

    pub fn map(&self) -> &Map {
        &self.map
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.insert(entity.id(), entity);
    }

    pub fn remove_entity(&mut self, id: EntityID) -> Option<Entity> {
        self.entities.remove(&id)
    }

    pub fn entities(&self) -> impl Iterator<Item=&Entity> {
        self.entities.values()
    }
}
