mod mode;

pub use self::mode::Mode;

use crate::decode::Decode;
use crate::B;

#[derive(Debug)]
pub struct Operand {
    mode: Mode,
    scalar: B<6>,
}

impl Operand {
    pub fn new(mode: Mode, scalar: B<6>) -> Operand {
        Operand { mode, scalar }
    }
}

impl From<Operand> for B<8> {
    fn from(od: Operand) -> B<8> {
        B::<8>::concat(B::<2>::from(od.mode), od.scalar)
    }
}

impl Decode<8> for Operand {
    fn decode(src: B<8>) -> Option<Self> {
        let (modebits, scalar): (B<2>, B<6>) = src.split();
        Mode::decode(modebits).map(|m| Operand::new(m, scalar))
    }
}
