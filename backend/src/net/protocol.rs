use crate::error::*;
use std::str::{Lines, FromStr};
use strum_macros::{EnumString, AsRefStr};

macro_rules! client_to_server_messages {
($($msg_key:literal, $enum_variant:ident, $payload_ident:ident),*$(,)?) => {
    pub enum ClientMessage {
        $(
            $enum_variant($payload_ident),
        )*
    }

    pub(super) fn next_message(lines: &mut Lines) -> Option<Result<ClientMessage, RCE>> {
        let msg_key = lines.next()?;
        Some(match msg_key {
            $(
                $msg_key => {
                    let payload = $payload_ident::parse(lines);
                    match payload {
                        Err(e) => Err(e),
                        Ok(p) => Ok(ClientMessage::$enum_variant(p)),
                    }
                },
            )*
            _ => Err(RCE::BadClientMessageType),
        })
    }
};
}

client_to_server_messages!(
    "ng", NewGame, NewGamePayload,
);


// PROTOCOL DATA TYPES //

#[derive(EnumString, AsRefStr)]
pub enum GameMode { FFA }


// PAYLOADS //

/// Server-to-Client payload
pub trait S2CPayload {
    fn msg_key() -> &'static str;
    fn encode(&self) -> String;
}
/// Client-to-Server payload
pub trait C2SPayload {
    fn parse(lines: &mut Lines) -> Result<Self, RCE> where Self: std::marker::Sized;
}

macro_rules! lines {
    () => {
        vec![Self::msg_key()]
    };
}

macro_rules! msg_key {
    ($key:literal) => {
        fn msg_key() -> &'static str { $key }
    }
}

pub struct NewGamePayload {
    pub map_name: String,
    pub gamemode: GameMode,
}
impl S2CPayload for NewGamePayload {
    msg_key!("ng");

    fn encode(&self) -> String {
        let mut lines = lines!();
        lines.push(&self.map_name[..]);
        lines.push(self.gamemode.as_ref());

        lines.join("\n")
    }
}
impl C2SPayload for NewGamePayload {
    fn parse(lines: &mut Lines) -> Result<Self, RCE> {
        const E: RCE = RCE::ProtocolDecode;
        let map_name = lines.next().to(E)?.to_string();
        let gamemode = GameMode::from_str(lines.next().to(E)?).to(E)?;

        Ok(Self {
            map_name,
            gamemode,
        })
    }
}
