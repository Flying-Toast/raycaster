macro_rules! bitflags {
    ($struct_vis:vis $flags_struct:ident; $enum_vis:vis $flag_enum:ident { $($flag:ident),*$(,)? }) => {
        #[repr(usize)]
        #[derive(Copy, Clone, strum_macros::EnumCount)]
        $enum_vis enum $flag_enum {
            $(
                $flag,
            )*
        }

        $struct_vis struct $flags_struct {
            #[allow(dead_code)]
            bytes: [u8; Self::num_bytes()],
        }

        impl $flags_struct {
            #[allow(dead_code)]
            pub fn new() -> Self {
                Self {
                    bytes: [0; Self::num_bytes()],
                }
            }

            #[allow(dead_code)]
            pub fn get(&self, flag: $flag_enum) -> bool {
                let (byte, bit) = Self::bit_position_of_flag(flag);

                self.bytes[byte] & bit != 0
            }

            #[allow(dead_code)]
            pub fn set(&mut self, flag: $flag_enum, value: bool) {
                let (byte, bit) = Self::bit_position_of_flag(flag);

                if value {
                    self.bytes[byte] |= bit;
                } else {
                    self.bytes[byte] &= !bit;
                }
            }

            fn as_bytes(&self) -> &[u8] {
                &self.bytes
            }

            fn from_bytes(bytes: [u8; Self::num_bytes()]) -> Self {
                Self {
                    bytes,
                }
            }

            #[allow(dead_code)]
            const fn num_bytes() -> usize {
                (Self::num_flags() / 8) + ((Self::num_flags() % 8 != 0) as usize)
            }

            #[allow(dead_code)]
            const fn num_flags() -> usize {
                use strum::EnumCount;

                $flag_enum::COUNT
            }

            #[allow(dead_code)]
            fn bit_position_of_flag(flag: $flag_enum) -> (usize, u8) {
                let byte_index = (flag as usize) / 8;
                let bit_offset = (flag as usize) % 8;

                (byte_index, 1 << bit_offset)
            }
        }

        impl crate::protocol::payload::Encodable for &$flags_struct {
            fn encode_to(self, builder: &mut crate::protocol::payload::PayloadBuilder) {
                builder.extend(self.as_bytes());
            }
        }

        impl crate::protocol::payload::Decodable for $flags_struct {
            fn decode_from(pieces: &mut crate::protocol::payload::Pieces) -> Result<Self, crate::error::CME> {
                use std::convert::TryInto;

                Ok(Self::from_bytes(
                    pieces.bytes_from_front(Self::num_bytes())?
                        .try_into()
                        .unwrap()
                ))
            }
        }
    };
}
