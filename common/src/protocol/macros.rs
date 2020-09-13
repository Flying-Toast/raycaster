macro_rules! def_serialized_fields {
    ($payload_type:ident {$($field_name:ident <- $encode_type:ty),*$(,)?}) => {
        #[allow(unused_variables)]
        impl crate::protocol::payload::Decodable for crate::protocol::payloads::$payload_type {
            fn decode_from(pieces: &mut crate::protocol::payload::Pieces) -> Result<Self, crate::error::CME> {
                Ok(Self {
                    $(
                        $field_name: pieces.get()?,
                    )*
                })
            }
        }

        #[allow(unused_mut)]
        impl crate::protocol::payloads::$payload_type {
            pub fn assemble($($field_name: &$encode_type),*) -> crate::protocol::payload::BuiltPayload {
                let mut builder = builder!();
                $(
                    builder.add($field_name);
                )*

                builder.build()
            }
        }
    };
}

macro_rules! generic_decl_payloads {
    ($keys_enum:ident, $enum_name:ident, $next_message:ident, $error_type:ident,
        $($enum_variant:ident, $payload_ident:ident),*$(,)?) =>
    {
        #[derive(Debug)]
        pub enum $enum_name {
            $(
                $enum_variant(crate::protocol::payloads::$payload_ident),
            )*
        }

        #[repr(u8)]
        enum $keys_enum {
            $(
                $enum_variant,
            )*
        }

        $(
            impl crate::protocol::payloads::$payload_ident {
                pub const fn payload_key() -> u8 {
                    $keys_enum::$enum_variant as u8
                }
            }
        )*

        /// Reads the next full payload from `pieces`
        pub fn $next_message(pieces: &mut crate::protocol::payload::Pieces)
            -> Option<Result<$enum_name, crate::error::CME>>
        {
            use crate::protocol::payload::Decodable;
            if pieces.is_empty() {
                return None;
            }
            let payload_key: u8 = match pieces.get() {
                Ok(s) => s,
                Err(e) => return Some(Err(e)),
            };
            Some(match payload_key {
                $(
                    key if key == crate::protocol::payloads::$payload_ident::payload_key() => {
                        let payload = crate::protocol::payloads::$payload_ident::decode_from(pieces);
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
    ($($enum_variant:ident, $payload_ident:ident),*$(,)?) => {
        generic_decl_payloads!(
            ClientPayloadKeys,
            ClientMessage,
            next_message_from_client,
            BadClientMessageType,
            $(
                $enum_variant, $payload_ident,
            )*
        );
    };
}

macro_rules! s2c_payloads {
    ($($enum_variant:ident, $payload_ident:ident),*$(,)?) => {
        generic_decl_payloads!(
            ServerPayloadKeys,
            ServerMessage,
            next_message_from_server,
            BadServerMessageType,
            $(
                $enum_variant, $payload_ident,
            )*
        );
    }
}

macro_rules! builder {
    () => {
        crate::protocol::payload::PayloadBuilder::new(Self::payload_key())
    };
}
