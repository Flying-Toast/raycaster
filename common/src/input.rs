use proc::Codable;

bitflags!(
    pub BistateInputs;
    pub BistateInput {
        MoveForwards,
        MoveBackwards,
        MoveLeft,
        MoveRight,
    }
);

#[derive(Debug, Clone, Codable)]
pub struct InputState {
    bistate_flags: BistateInputs,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            bistate_flags: BistateInputs::new(),
        }
    }

    pub fn bistates(&mut self) -> &mut BistateInputs {
        &mut self.bistate_flags
    }
}

#[derive(Debug, Codable)]
pub struct ForeignInput {
    state: InputState,
    dt: u8,
}

impl ForeignInput {
    pub fn new(state: InputState, dt: u8) -> Self {
        Self {
            state,
            dt,
        }
    }
}

#[derive(Debug, Codable)]
pub struct Input {
    input: ForeignInput,
    seq_id: u32,
}

impl Input {
    pub fn new(input: ForeignInput, seq_id: u32) -> Self {
        Self {
            input,
            seq_id,
        }
    }

    pub fn seq_id(&self) -> u32 {
        self.seq_id
    }

    pub fn as_foreign(&self) -> &ForeignInput {
        &self.input
    }

    pub fn into_foreign(self) -> ForeignInput {
        self.input
    }
}
