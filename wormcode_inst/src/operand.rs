mod mode;

#[cfg(test)]
mod tests;

pub use self::mode::Mode;

use wormcode_bits::{Decode, Encode, B};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Operand {
    mode: Mode,
    scalar: B<6>,
}

impl Operand {
    pub fn new(mode: Mode, scalar: B<6>) -> Operand {
        Operand { mode, scalar }
    }
}

impl Encode<8> for Operand {
    fn encode(self) -> B<8> {
        B::<2>::from(self.mode).concat(self.scalar)
    }
}

impl Decode<8> for Operand {
    fn decode(src: B<8>) -> Option<Self> {
        let (modebits, scalar): (B<2>, B<6>) = src.split();
        Mode::decode(modebits).map(|m| Operand::new(m, scalar))
    }
}
