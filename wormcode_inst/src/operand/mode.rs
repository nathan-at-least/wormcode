#[cfg(test)]
mod tests;

use crate::decode::Decode;
use wormcode_bits::B;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Mode {
    Literal,
    Direct,
    Indirect,
}

impl From<Mode> for B<2> {
    fn from(m: Mode) -> B<2> {
        B::<2>::from(m as u32)
    }
}

impl Decode<2> for Mode {
    fn decode(b: B<2>) -> Option<Self> {
        use Mode::*;

        match u32::from(b) {
            0 => Some(Literal),
            1 => Some(Direct),
            2 => Some(Indirect),
            _ => None,
        }
    }
}
