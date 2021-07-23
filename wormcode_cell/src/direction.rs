use wormcode_bits::{DecodeExact, Encode, B};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl DecodeExact<2> for Direction {
    fn decode_exact(b: B<2>) -> Direction {
        use Direction::*;

        match u32::from(b) {
            0b00 => North,
            0b01 => East,
            0b10 => South,
            0b11 => West,
            _ => unreachable!(),
        }
    }
}

impl Encode<2> for Direction {
    fn encode(self) -> B<2> {
        B::from(self as u32)
    }
}
