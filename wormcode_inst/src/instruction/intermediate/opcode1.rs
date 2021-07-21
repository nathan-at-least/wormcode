use wormcode_bits::{Decode, Encode, B};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OpCode1 {
    Step,
}

impl Encode<16> for OpCode1 {
    fn encode(self) -> B<16> {
        B::<16>::from(self as u32)
    }
}

impl Decode<16> for OpCode1 {
    fn decode_option(src: B<16>) -> Option<Self> {
        use OpCode1::*;

        match u32::from(src) {
            0 => Some(Step),
            _ => None,
        }
    }
}
