use crate::error::*;
use std::str::{Lines, FromStr};
use strum_macros::{EnumString, AsRefStr};

macro_rules! def_messages {
($($msg_key:literal, $enum_variant:ident, $payload_ident:ident),*$(,)?) => {
    pub enum ProtocolMessage {
        $(
            $enum_variant($payload_ident),
        )*
    }

    $(
        impl $payload_ident {
            fn msg_key() -> &'static str {
                $msg_key
            }
        }
    )*

    pub(super) fn next_message(lines: &mut Lines) -> Option<Result<ProtocolMessage, RCE>> {
        let msg_key = lines.next()?;
        Some(match msg_key {
            $(
                $msg_key => {
                    let payload = $payload_ident::parse(lines);
                    match payload {
                        Err(e) => Err(e),
                        Ok(p) => Ok(ProtocolMessage::$enum_variant(p)),
                    }
                },
            )*
            _ => Err(RCE::BadProtocolMessageType),
        })
    }
};
}

def_messages!(
    "ng", NewGame, NewGamePayload,
);


// PROTOCOL DATA TYPES //

#[derive(EnumString, AsRefStr)]
pub enum GameMode { FFA }


// PAYLOADS //

pub trait Payload {
    fn parse(lines: &mut Lines) -> Result<Self, RCE> where Self: std::marker::Sized;
    fn encode(&self) -> String;
}

macro_rules! lines {
    () => {
        vec![Self::msg_key()]
    };
}

/// Client <- Server
pub struct NewGamePayload {
    pub map_name: String,
    pub gamemode: GameMode,
}
impl Payload for NewGamePayload {
    fn parse(lines: &mut Lines) -> Result<Self, RCE> {
        const E: RCE = RCE::ProtocolDecode;
        let map_name = lines.next().to(E)?.to_string();
        let gamemode = GameMode::from_str(lines.next().to(E)?).to(E)?;

        Ok(Self {
            map_name,
            gamemode,
        })
    }

    fn encode(&self) -> String {
        let mut lines = lines!();
        lines.push(&self.map_name[..]);
        lines.push(self.gamemode.as_ref());

        lines.join("\n")
    }
}
