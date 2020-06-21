use crate::error::*;
use std::str::{Lines, FromStr};
use strum_macros::{EnumString, AsRefStr};

macro_rules! def_messages {
($($msg_type:literal, $enum_variant:ident, $payload_ident:ident),*$(,)?) => {
    pub enum ProtocolMessage {
        $(
            $enum_variant($payload_ident),
        )*
    }

    pub(super) fn next_message(lines: &mut Lines) -> Option<Result<ProtocolMessage, RCE>> {
        let msg_type = lines.next()?;
        Some(match msg_type {
            $(
                $msg_type => {
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

/// Client <- Server
pub struct NewGamePayload {
    pub map_name: String,
    pub gamemode: GameMode,
}
impl NewGamePayload {
    fn parse(lines: &mut Lines) -> Result<Self, RCE> {
        const E: RCE = RCE::ProtocolDecode;
        let map_name = lines.next().to(E)?.to_string();
        let gamemode = GameMode::from_str(lines.next().to(E)?).to(E)?;

        Ok(Self {
            map_name,
            gamemode,
        })
    }

    fn encode(&self) -> Result<String, RCE> {
        let mut lines: Vec<&str> = Vec::new();
        lines.push(&self.map_name[..]);
        lines.push(self.gamemode.as_ref());

        Ok(lines.join("\n"))
    }
}
