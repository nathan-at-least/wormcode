mod encoding;

use crate::{Operand, B};

pub type Instruction = InstG<Operand>;

pub enum InstG<Op> {
    Data(B<24>),
    Nop,
    Step(Op),
    Inc(Op, Op),
    MemCpy(Op, Op, Op),
}
