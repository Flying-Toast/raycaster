bitflags!(
    pub BistateInputs;
    pub BistateInput {
        MoveForwards,
        MoveBackwards,
        MoveLeft,
        MoveRight,
    }
);

#[derive(Debug, Clone)]
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

#[derive(Debug)]
pub struct Input {
    state: InputState,
    seq_id: u32,
    dt: u8,
}

impl Input {
    pub fn new(state: InputState, seq_id: u32, dt: u8) -> Self {
        Self {
            state,
            seq_id,
            dt,
        }
    }

    pub fn seq_id(&self) -> u32 {
        self.seq_id
    }

    pub fn state(&self) -> &InputState {
        &self.state
    }

    pub fn dt(&self) -> u8 {
        self.dt
    }
}
