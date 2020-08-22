use crate::entity::{EntityID, Entity};
use crate::map::Map;


/// Tells a client what their player entity's id is
#[derive(Debug)]
pub struct YourIDPayload {
    pub id: EntityID,
}

def_serialized_fields!(
    YourIDPayload {
        id <- EntityID,
    }
);

/// Tells a client to remove the specified entity
#[derive(Debug)]
pub struct RemoveEntityPayload {
    pub entity: EntityID,
}

def_serialized_fields!(
    RemoveEntityPayload {
        entity <- EntityID,
    }
);

/// Announces the creation of a new entity
#[derive(Debug)]
pub struct NewEntityPayload {
    pub entity: Entity,
}

def_serialized_fields!(
    NewEntityPayload {
        entity <- &Entity,
    }
);

/// Instructs a client to change their map
#[derive(Debug)]
pub struct SetMapPayload {
    pub map: Map,
}

def_serialized_fields!(
    SetMapPayload {
        map <- &Map,
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
        message <- &str,
        random_u32 <- u32,
    }
);
