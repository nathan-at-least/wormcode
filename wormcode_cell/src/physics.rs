use wormcode_bits::{DecodeExact, Encode, B};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Physics {
    Air,
    Dirt,
    Body,
    Head,
}

impl DecodeExact<2> for Physics {
    fn decode_exact(b: B<2>) -> Physics {
        use Physics::*;

        match u32::from(b) {
            0b00 => Air,
            0b01 => Dirt,
            0b10 => Body,
            0b11 => Head,
            _ => unreachable!(),
        }
    }
}

impl Encode<2> for Physics {
    fn encode(self) -> B<2> {
        B::from(self as u32)
    }
}
