#[macro_use]
mod macros;
pub mod payload;
pub mod payloads;

// client-to-server payloads
c2s_payloads!(
//  name of enum variant            payload struct type
    Input,                          InputPayload,
);

// server-to-client payloads
s2c_payloads!(
//  name of enum variant            payload struct type
    YourID,                         YourIDPayload,
    RemoveEntity,                   RemoveEntityPayload,
    NewEntity,                      NewEntityPayload,
    SetMap,                         SetMapPayload,
    LastProcessedInput,             LastProcessedInputPayload,
    Ready,                          ReadyPayload,
    ForeignInputs,                   ForeignInputsPayload,
);
