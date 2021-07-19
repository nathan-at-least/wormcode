use wormcode_bits::{Decode, B};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OpCode3 {
    // PlaceHolders reserve spine values for non-tertiary instructions:
    // Data, Nullary, Unary, Binary

    // Actual tertiary instruction opcodes:
    MemCpy = 0x4,
}

impl From<OpCode3> for B<4> {
    fn from(oc: OpCode3) -> B<4> {
        B::<4>::from(oc as u32)
    }
}

impl Decode<4> for OpCode3 {
    fn decode(src: B<4>) -> Option<Self> {
        use OpCode3::*;

        match u32::from(src) {
            4 => Some(MemCpy),
            _ => None,
        }
    }
}
