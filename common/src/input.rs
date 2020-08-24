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
