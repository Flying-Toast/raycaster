#[macro_use]
mod macros;
pub mod payload;
pub mod payloads;


c2s_payloads!(
//  payload key   name of enum variant            payload struct type
    0,            Hello,                          ClientHelloPayload,
);

s2c_payload_keys!(
    YourIDPayload, 0,
    RemoveEntityPayload, 1,
    NewEntityPayload, 2,
);
