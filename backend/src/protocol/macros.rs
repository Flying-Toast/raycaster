macro_rules! client_to_server_messages {
    ($($payload_key:literal, $enum_variant:ident, $payload_ident:ident),*$(,)?) => {
        /// A payload from the client
        pub enum ClientMessage {
            $(
                $enum_variant($payload_ident),
            )*
        }

        /// Reads the next full payload from `pieces`
        pub fn next_message(pieces: &mut Pieces) -> Option<Result<ClientMessage, RCE>> {
            let payload_key = match pieces.get_str() {
                Ok(s) => s,
                Err(RCE::EmptyPieces) => return None,
                Err(e) => return Some(Err(e)),
            };
            Some(match payload_key {
                $(
                    $payload_key => {
                        let payload = $payload_ident::parse(pieces);
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

macro_rules! lines {
    () => {
        vec![Self::payload_key()]
    };
}

/// Shorthand for defining `S2CPayload::payload_key()`
macro_rules! key {
    ($key:literal) => {
        fn payload_key() -> &'static str { $key }
    }
}
