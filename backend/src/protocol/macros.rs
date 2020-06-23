macro_rules! client_to_server_messages {
    ($($msg_key:literal, $enum_variant:ident, $payload_ident:ident),*$(,)?) => {
        pub enum ClientMessage {
            $(
                $enum_variant($payload_ident),
            )*
        }

        pub fn next_message(pieces: &mut Pieces) -> Option<Result<ClientMessage, RCE>> {
            let msg_key = match pieces.get_str()? {
                Ok(s) => s,
                Err(e) => return Some(Err(e)),
            };
            Some(match msg_key {
                $(
                    $msg_key => {
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
        vec![Self::msg_key()]
    };
}

macro_rules! msg_key {
    ($key:literal) => {
        fn msg_key() -> &'static str { $key }
    }
}
