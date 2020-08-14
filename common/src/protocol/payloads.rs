use crate::error::*;
use crate::protocol::payload::{BuiltPayload, Payload, Pieces};
use crate::entity::{EntityID, Entity};


/// Tells a client what their player entity's id is
#[derive(Debug)]
pub struct YourIDPayload {
    pub id: EntityID,
}
impl Payload for YourIDPayload {
    fn parse(pieces: &mut Pieces) -> Result<Self, CME> {
        Ok(Self {
            id: pieces.get_ent_id()?,
        })
    }
}
impl YourIDPayload {
    pub fn assemble(entity_id: EntityID) -> BuiltPayload {
        let mut builder = builder!();
        builder.add_ent_id(entity_id);

        builder.build()
    }
}

/// Tells a client to remove the specified entity
#[derive(Debug)]
pub struct RemoveEntityPayload {
    pub entity: EntityID,
}
impl Payload for RemoveEntityPayload {
    fn parse(pieces: &mut Pieces) -> Result<Self, CME> {
        Ok(Self {
            entity: pieces.get_ent_id()?,
        })
    }
}
impl RemoveEntityPayload {
    pub fn assemble(entity: EntityID) -> BuiltPayload {
        let mut builder = builder!();
        builder.add_ent_id(entity);

        builder.build()
    }
}

/// Announces the creation of a new entity
#[derive(Debug)]
pub struct NewEntityPayload {
    pub entity: Entity,
}
impl Payload for NewEntityPayload {
    fn parse(pieces: &mut Pieces) -> Result<Self, CME> {
        Ok(Self {
            entity: pieces.get_entity()?,
        })
    }
}
impl NewEntityPayload {
    pub fn assemble(entity: &Entity) -> BuiltPayload {
        let mut builder = builder!();
        builder.add_entity(&entity);

        builder.build()
    }
}

//TEMP
#[derive(Debug)]
pub struct ClientHelloPayload {
    pub message: String,
    pub random_u32: u32,
}
impl Payload for ClientHelloPayload {
    fn parse(pieces: &mut Pieces) -> Result<Self, CME> {
        Ok(Self {
            message: pieces.get_string()?,
            random_u32: pieces.get_u32()?,
        })
    }
}
impl ClientHelloPayload {
    pub fn assemble(message: &str, random_u32: u32) -> BuiltPayload {
        let mut builder = builder!();
        builder.add_str(message);
        builder.add_u32(random_u32);

        builder.build()
    }
}
