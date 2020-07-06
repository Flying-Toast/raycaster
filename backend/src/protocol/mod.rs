/// Definitions of all types that can be sent over the network
pub mod types;
#[macro_use]
mod macros;
pub mod payload;
/// Implementations of payloads
pub mod payloads;


// defines the `ClientMessage` enum and `next_message` function in this scope
client_to_server_messages!(
//  payload key     name of ClientMessage variant   payload struct type
    "p",            Pong,                           PingPongPayload,
);
