use crate::entity::{EntityID, Entity};
use crate::map::Map;
use crate::input::Input;


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

#[derive(Debug)]
pub struct InputPayload {
    pub input: Input,
}

def_serialized_fields!(
    InputPayload {
        input <- &Input,
    }
);
