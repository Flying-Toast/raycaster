use crate::error::*;
use crate::protocol::payload::{BuiltPayload, C2SPayload, Pieces};
use crate::entity::{EntityID, Entity};


/// Tells a client what their player entity's id is
pub struct YourIDPayload;
impl YourIDPayload {
    pub fn assemble(entity_id: EntityID) -> BuiltPayload {
        let mut builder = builder!();
        builder.add_ent_id(entity_id);

        builder.build()
    }
}

/// Tells a client to remove the specified entity
pub struct RemoveEntityPayload;
impl RemoveEntityPayload {
    pub fn assemble(entity: EntityID) -> BuiltPayload {
        let mut builder = builder!();
        builder.add_ent_id(entity);

        builder.build()
    }
}

/// Announces the creation of a new entity
pub struct NewEntityPayload;
impl NewEntityPayload {
    pub fn assemble(entity: &Entity) -> BuiltPayload {
        let mut builder = builder!();
        builder.add_ent_id(entity.id);
        builder.add_vector(entity.location());

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
    fn parse(pieces: &mut Pieces) -> Result<Self, CME> {
        let message = pieces.get_string()?;
        let random_u32 = pieces.get_u32()?;

        Ok(Self {
            message,
            random_u32,
        })
    }
}
