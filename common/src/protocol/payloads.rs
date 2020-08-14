use crate::entity::{EntityID, Entity};


/// Tells a client what their player entity's id is
#[derive(Debug)]
pub struct YourIDPayload {
    pub id: EntityID,
}

def_serialized_fields!(
    YourIDPayload {
        id: EntityID, ent_id,
    }
);

/// Tells a client to remove the specified entity
#[derive(Debug)]
pub struct RemoveEntityPayload {
    pub entity: EntityID,
}

def_serialized_fields!(
    RemoveEntityPayload {
        entity: EntityID, ent_id,
    }
);

/// Announces the creation of a new entity
#[derive(Debug)]
pub struct NewEntityPayload {
    pub entity: Entity,
}

def_serialized_fields!(
    NewEntityPayload {
        entity: &Entity, entity,
    }
);

//TEMP
#[derive(Debug)]
pub struct HelloPayload {
    pub message: String,
    pub random_u32: u32,
}

def_serialized_fields!(
    HelloPayload {
        message: &str, string,
        random_u32: u32, u32,
    }
);
