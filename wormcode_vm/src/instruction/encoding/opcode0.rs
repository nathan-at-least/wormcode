use crate::decode::Decode;
use crate::B;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OpCode0 {
    Nop,
}

impl From<OpCode0> for B<24> {
    fn from(oc: OpCode0) -> B<24> {
        B::<24>::from(oc as u32)
    }
}

impl Decode<24> for OpCode0 {
    fn decode(src: B<24>) -> Option<Self> {
        use OpCode0::*;

        match u32::from(src) {
            0 => Some(Nop),
            _ => None,
        }
    }
}
