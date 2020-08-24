use common::gamestate::GameState;
use common::map::Map;
use common::entity::EntityID;
use common::protocol::ServerMessage;
use common::input::Input;


#[derive(Debug)]
pub struct Game {
    state: GameState,
    my_id: EntityID,
    ready: bool,
    /// For client-side prediction
    unprocessed_inputs: Vec<Input>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            state: GameState::new(Map::dummy()),
            // dummy value, overwritten when "YourID" message is received
            my_id: EntityID::new(12345),
            ready: false,
            unprocessed_inputs: Vec::new(),
        }
    }

    pub fn ready(&self) -> bool {
        self.ready
    }

    /// Pushes an input to the unprocessed inputs queue
    pub fn push_input(&mut self, input: Input) {
        self.unprocessed_inputs.push(input);
    }

    pub fn on_message(&mut self, message: ServerMessage) {
        match message {
            ServerMessage::Ready(_) => {
                self.ready = true;
            },
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
                let mut tmp = Vec::new();
                std::mem::swap(&mut tmp, &mut self.unprocessed_inputs);
                self.unprocessed_inputs = tmp
                    .into_iter()
                    .skip_while(|i| i.seq_id() <= payload.id)
                    .collect();
            },
        }
    }
}
