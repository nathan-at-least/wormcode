mod encoding;

#[cfg(test)]
mod tests;

use crate::{Operand, B};

pub type Instruction = InstG<Operand>;

#[derive(Copy, Clone, Debug)]
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
