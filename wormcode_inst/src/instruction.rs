mod encoding;

#[cfg(test)]
mod tests;

use crate::decode::Decode;
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

impl Instruction {
    pub fn encode(self) -> B<28> {
        encoding::encode(self)
    }
}

impl Decode<28> for Instruction {
    fn decode(b: B<28>) -> Option<Self> {
        encoding::decode(b)
    }
}
