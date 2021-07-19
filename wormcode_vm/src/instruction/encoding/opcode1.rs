use crate::decode::Decode;
use crate::B;

#[derive(Debug)]
pub enum OpCode1 {
    Step,
}

impl From<OpCode1> for B<16> {
    fn from(oc: OpCode1) -> B<16> {
        B::<16>::from(oc as u32)
    }
}

impl Decode<16> for OpCode1 {
    fn decode(src: B<16>) -> Option<Self> {
        use OpCode1::*;

        match u32::from(src) {
            0 => Some(Step),
            _ => None,
        }
    }
}
