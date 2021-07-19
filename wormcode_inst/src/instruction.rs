mod encoding;

#[cfg(test)]
mod tests;

use crate::Operand;
use wormcode_bits::B;

pub type Instruction = InstG<Operand>;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum InstG<Op> {
    Data(B<24>),
    Nop,
    Step(Op),
    Inc(Op, Op),
    MemCpy(Op, Op, Op),
}
