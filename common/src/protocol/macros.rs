macro_rules! generic_decl_payloads {
    ($enum_name:ident, $next_message:ident, $error_type:ident,
        $($payload_key:literal, $enum_variant:ident, $payload_ident:ident),*$(,)?) =>
    {
        #[derive(Debug)]
        pub enum $enum_name {
            $(
                $enum_variant(crate::protocol::payloads::$payload_ident),
            )*
        }

        $(
            impl crate::protocol::payloads::$payload_ident {
                pub const fn payload_key() -> u16 {
                    $payload_key
                }
            }
        )*

        /// Reads the next full payload from `pieces`
        #[deny(unreachable_patterns)] // NOTE: if this causes a compile error, it means a payload key was used more than once
        pub fn $next_message(pieces: &mut crate::protocol::payload::Pieces)
            -> Option<Result<$enum_name, crate::error::CME>>
        {
            use crate::protocol::payload::Payload;
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
                            Ok(p) => Ok($enum_name::$enum_variant(p)),
                        }
                    },
                )*
                _ => Err(crate::error::CME::$error_type{payload_key}),
            })
        }
    };
}

macro_rules! c2s_payloads {
    ($($payload_key:literal, $enum_variant:ident, $payload_ident:ident),*$(,)?) => {
        generic_decl_payloads!(
            ClientMessage,
            next_message_from_client,
            BadClientMessageType,
            $(
                $payload_key, $enum_variant, $payload_ident,
            )*
        );
    };
}

macro_rules! s2c_payloads {
    ($($payload_key:literal, $enum_variant:ident, $payload_ident:ident),*$(,)?) => {
        generic_decl_payloads!(
            ServerMessage,
            next_message_from_server,
            BadServerMessageType,
            $(
                $payload_key, $enum_variant, $payload_ident,
            )*
        );
    }
}

macro_rules! builder {
    () => {
        crate::protocol::payload::PayloadBuilder::new(Self::payload_key())
    };
}
