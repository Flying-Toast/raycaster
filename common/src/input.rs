use crate::error::*;
use crate::protocol::payload::{Pieces, PayloadBuilder, Encodable, Decodable};


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

impl Encodable for &InputState {
    fn encode_to(self, builder: &mut PayloadBuilder) {
        builder.add(&self.bistate_flags);
    }
}

impl Decodable for InputState {
    fn decode_from(pieces: &mut Pieces) -> Result<Self, CME> {
        Ok(Self {
            bistate_flags: pieces.get()?,
        })
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

impl Encodable for &Input {
    fn encode_to(self, builder: &mut PayloadBuilder) {
        builder.add(&self.state);
        builder.add(self.seq_id);
        builder.add(self.dt);
    }
}

impl Decodable for Input {
    fn decode_from(pieces: &mut Pieces) -> Result<Self, CME> {
        Ok(
            Self::new(pieces.get()?, pieces.get()?, pieces.get()?)
        )
    }
}
