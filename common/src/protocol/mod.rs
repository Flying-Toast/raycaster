#[macro_use]
mod macros;
pub mod payload;
pub mod payloads;


// client-to-server payloads
c2s_payloads!(
//  payload key   name of enum variant            payload struct type
    0,            Hello,                          ClientHelloPayload,
);

// server-to-client payloads
s2c_payloads!(
//  payload key   name of enum variant            payload struct type
    0,            YourID,                         YourIDPayload,
    1,            RemoveEntity,                   RemoveEntityPayload,
    2,            NewEntity,                      NewEntityPayload,
);
