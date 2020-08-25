use common::gamestate::GameState;
use common::map::Map;
use common::entity::EntityID;
use common::protocol::ServerMessage;
use common::input::Input;


#[derive(Debug)]
pub struct Game {
    authoritative_state: GameState,
    /// A clone of the most recent authoritative_state with our unprocessed_inputs applied on top
    predicted_state: GameState,
    /// Our prediction becomes invalid whenever we receive an authoritative state change
    prediction_invalid: bool,
    /// ID of the most recent input applied to predicted_state
    last_predicted_input: u32,
    my_id: EntityID,
    ready: bool,
    /// For client-side prediction
    unprocessed_inputs: Vec<Input>,
}

impl Game {
    pub fn new() -> Self {
        let dummy_state = GameState::new(Map::dummy());
        Self {
            authoritative_state: dummy_state.clone(),
            predicted_state: dummy_state,
            prediction_invalid: false,
            last_predicted_input: 0,
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

    pub fn predict_state(&mut self) -> &GameState {
        if self.prediction_invalid {
            self.predicted_state = self.authoritative_state.clone();
            self.last_predicted_input = 0;
            self.prediction_invalid = false;
        }

        let last_predicted_input = self.last_predicted_input;
        for input in self.unprocessed_inputs
            .iter()
            .skip_while(|input| input.seq_id() <= last_predicted_input)
        {
            self.predicted_state.apply_input(self.my_id, input.as_foreign());
            self.last_predicted_input = input.seq_id();
        }

        &self.predicted_state
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
                self.authoritative_state.add_entity(payload.entity);
                self.prediction_invalid = true;
            },
            ServerMessage::RemoveEntity(payload) => {
                self.authoritative_state.remove_entity(payload.entity);
                self.prediction_invalid = true;
            },
            ServerMessage::SetMap(payload) => {
                self.authoritative_state.set_map(payload.map);
                self.prediction_invalid = true;
            },
            ServerMessage::ForeignInput(payload) => {
                self.authoritative_state.apply_input(payload.puppeteer, &payload.input);
                self.prediction_invalid = true;
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
