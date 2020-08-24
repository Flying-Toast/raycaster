use crate::entity::{EntityID, Entity};
use crate::map::Map;
use crate::input::Input;


// SERVER-TO-CLIENT PAYLOADS:

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
        entity <- Entity,
    }
);

/// Instructs a client to change their map
#[derive(Debug)]
pub struct SetMapPayload {
    pub map: Map,
}

def_serialized_fields!(
    SetMapPayload {
        map <- Map,
    }
);

#[derive(Debug)]
pub struct LastProcessedInputPayload {
    pub id: u32,
}

def_serialized_fields!(
    LastProcessedInputPayload {
        id <- u32,
    }
);

/// Sent to a new client after they have been sent the initial state
#[derive(Debug)]
pub struct ReadyPayload;

def_serialized_fields!(
    ReadyPayload {}
);

/// Someone else's input
#[derive(Debug)]
pub struct ForeignInputPayload {
    pub puppeteer: EntityID,
    pub input: Input,
}

def_serialized_fields!(
    ForeignInputPayload {
        puppeteer <- EntityID,
        input <- Input,
    }
);


// CLIENT-TO-SERVER PAYLOADS:

#[derive(Debug)]
pub struct InputPayload {
    pub input: Input,
}

def_serialized_fields!(
    InputPayload {
        input <- Input,
    }
);
