mod mode;

pub use self::mode::Mode;

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
