use crate::error::*;
use crate::protocol::payload::{BuiltPayload, C2SPayload, Pieces};
use crate::game::entity::EntityID;


/// Tells a client what their player entity's id is
#[derive(Debug)]
pub struct YourIDPayload;
impl YourIDPayload {
    pub fn assemble(entity_id: EntityID) -> BuiltPayload {
        let mut builder = builder!();
        builder.add_ent_id(entity_id);

        builder.build()
    }
}

//TEMP
#[derive(Debug)]
pub struct ClientHelloPayload {
    pub message: String,
    pub random_u32: u32,
}
impl C2SPayload for ClientHelloPayload {
    fn parse(pieces: &mut Pieces) -> Result<Self, RCE> {
        let message = pieces.get_str()?.to_string();
        let random_u32 = pieces.get_u32()?;

        Ok(Self {
            message,
            random_u32,
        })
    }
}
