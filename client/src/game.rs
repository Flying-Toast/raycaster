use common::gamestate::GameState;
use common::map::Map;
use common::entity::EntityID;
use common::protocol::ServerMessage;


#[derive(Debug)]
pub struct Game {
    state: GameState,
    my_id: EntityID,
}

impl Game {
    pub fn new() -> Self {
        Self {
            state: GameState::new(Map::dummy()),
            // dummy value, overwritten when "YourID" message is received
            my_id: EntityID::new(12345),
        }
    }

    pub fn on_message(&mut self, message: ServerMessage) {
        match message {
            ServerMessage::YourID(payload) => {
                self.my_id = payload.id;
            },
            ServerMessage::NewEntity(payload) => {
                self.state.add_entity(payload.entity);
            },
            ServerMessage::RemoveEntity(payload) => {
                self.state.remove_entity(payload.entity);
            },
            ServerMessage::SetMap(payload) => {
                self.state.set_map(payload.map);
            },
            ServerMessage::LastProcessedInput(payload) => {
                //TODO
            },
        }
    }
}
