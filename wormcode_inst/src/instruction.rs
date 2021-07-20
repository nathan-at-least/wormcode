// This pulls in `Encode<28>` / `Decode<28>`
mod intermediate;

#[cfg(test)]
mod tests;

use crate::Operand as Op;
use wormcode_bits::B;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Instruction {
    Data(B<24>),
    Nop,
    Step(Op),
    Inc(Op, Op),
    MemCpy(Op, Op, Op),
}
