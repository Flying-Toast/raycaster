use common::entity::EntityID;
use common::gamestate::GameState;

pub struct Renderer {
}

impl Renderer {
    pub fn new() -> Self {
        Self {
        }
    }

    /// Renders `state` as seen by the `perspective` entity
    pub fn render(&self, state: &GameState, perspective: EntityID) {
    }
}
