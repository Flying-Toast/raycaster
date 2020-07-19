macro_rules! client_to_server_messages {
    ($($payload_key:literal, $enum_variant:ident, $payload_ident:ident),*$(,)?) => {
        /// A payload from the client
        #[derive(Debug)]
        pub enum ClientMessage {
            $(
                $enum_variant(crate::protocol::payloads::$payload_ident),
            )*
        }

        /// Reads the next full payload from `pieces`
        #[deny(unreachable_patterns)] // NOTE: if this causes a compile error, it means a payload key was used more than once
        pub fn next_message(pieces: &mut crate::protocol::payload::Pieces)
            -> Option<Result<ClientMessage, crate::error::RCE>>
        {
            use crate::protocol::payload::C2SPayload;
            if pieces.is_empty() {
                return None;
            }
            let payload_key = match pieces.get_u16() {
                Ok(s) => s,
                Err(e) => return Some(Err(e)),
            };
            Some(match payload_key {
                $(
                    $payload_key => {
                        let payload = crate::protocol::payloads::$payload_ident::parse(pieces);
                        match payload {
                            Err(e) => Err(e),
                            Ok(p) => Ok(ClientMessage::$enum_variant(p)),
                        }
                    },
                )*
                _ => Err(crate::error::RCE::BadClientMessageType),
            })
        }
    };
}

macro_rules! s2c_payload_keys {
    ($($payload_ident:ident, $payload_key:literal),*$(,)?) => {
        // HACK: ensure payload keys are only used once
        #[deny(unreachable_patterns)] // NOTE: if this causes a compile error, it means a payload key was used more than once
        #[doc(hidden)]
        fn _for_lint_only_do_not_call() {
            match 0 {
                $(
                    $payload_key => (),
                )*
                _ => (),
            }
        }

        $(
            impl crate::protocol::payloads::$payload_ident {
                pub fn payload_key() -> u16 {
                    $payload_key
                }
            }
        )*
    }
}

macro_rules! builder {
    () => {
        crate::protocol::payload::PayloadBuilder::new(Self::payload_key())
    };
}
