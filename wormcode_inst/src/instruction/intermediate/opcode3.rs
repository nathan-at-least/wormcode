use wormcode_bits::{Decode, Encode, B};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OpCode3 {
    // PlaceHolders reserve spine values for non-tertiary instructions:
    // Data, Nullary, Unary, Binary

    // Actual tertiary instruction opcodes:
    MemCpy = 0x4,
}

impl Encode<4> for OpCode3 {
    fn encode(self) -> B<4> {
        B::<4>::from(self as u32)
    }
}

impl Decode<4> for OpCode3 {
    fn decode_option(src: B<4>) -> Option<Self> {
        use OpCode3::*;

        match u32::from(src) {
            4 => Some(MemCpy),
            _ => None,
        }
    }
}
