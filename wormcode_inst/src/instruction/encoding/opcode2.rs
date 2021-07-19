use wormcode_bits::{Decode, B};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OpCode2 {
    Inc,
}

impl From<OpCode2> for B<8> {
    fn from(oc: OpCode2) -> B<8> {
        B::<8>::from(oc as u32)
    }
}

impl Decode<8> for OpCode2 {
    fn decode(src: B<8>) -> Option<Self> {
        use OpCode2::*;

        match u32::from(src) {
            0 => Some(Inc),
            _ => None,
        }
    }
}
