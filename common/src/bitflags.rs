macro_rules! bitflags {
    ($flags_struct:ident, $flag_enum:ident { $($flag:ident),*$(,)? }) => {
        #[repr(usize)]
        #[derive(Copy, Clone)]
        pub enum $flag_enum {
            $(
                $flag,
            )*
            _MaxFlag,
        }

        pub struct $flags_struct {
            bytes: [u8; Self::num_bytes()],
        }

        impl $flags_struct {
            pub fn new() -> Self {
                Self {
                    bytes: [0; Self::num_bytes()],
                }
            }

            pub const fn num_bytes() -> usize {
                (Self::num_flags() / 8) + ((Self::num_flags() % 8 != 0) as usize)
            }

            pub fn get(&self, flag: $flag_enum) -> bool {
                let (byte, bit) = Self::bit_position_of_flag(flag);

                self.bytes[byte] & bit != 0
            }

            pub fn set(&mut self, flag: $flag_enum, value: bool) {
                let (byte, bit) = Self::bit_position_of_flag(flag);

                if value {
                    self.bytes[byte] |= bit;
                } else {
                    self.bytes[byte] &= !bit;
                }
            }

            const fn num_flags() -> usize {
                $flag_enum::_MaxFlag as usize
            }

            fn bit_position_of_flag(flag: $flag_enum) -> (usize, u8) {
                assert!(!matches!(flag, $flag_enum::_MaxFlag), "_MaxFlag is not a valid flag");

                let byte_index = (flag as usize) / 8;
                let bit_offset = (flag as usize) % 8;

                (byte_index, 1 << bit_offset)
            }
        }
    };
}
