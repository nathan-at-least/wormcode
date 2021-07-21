use wormcode_bits::{Decode, Encode, B};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OpCode0 {
    Nop,
}

impl Encode<24> for OpCode0 {
    fn encode(self) -> B<24> {
        B::<24>::from(self as u32)
    }
}

impl Decode<24> for OpCode0 {
    fn decode_option(src: B<24>) -> Option<Self> {
        use OpCode0::*;

        match u32::from(src) {
            0 => Some(Nop),
            _ => None,
        }
    }
}
