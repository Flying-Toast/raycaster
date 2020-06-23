pub mod types;
#[macro_use]
mod macros;
pub mod payload;
pub mod payloads;


use crate::protocol::payload::{C2SPayload};
use crate::protocol::payloads::*;
use std::str::{Lines};
use crate::error::*;


client_to_server_messages!(
    "ng", NewGame, NewGamePayload,
);
