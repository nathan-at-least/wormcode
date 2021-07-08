use crate::B;

#[derive(Debug)]
pub struct Operand {
    mode: Mode,
    scalar: B<6>,
}

#[derive(Debug)]
pub enum Mode {
    Literal,
    Direct,
    Indirect,
}

impl From<Operand> for B<8> {
    fn from(od: Operand) -> B<8> {
        B::<8>::from_u32(((od.mode as u32) << 6) & u32::from(od.scalar))
    }
}
