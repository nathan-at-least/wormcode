mod encoding;

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

#[cfg(test)]
mod encoding_tests {
    use crate::{InstG::*, Instruction, Mode::*, Operand, B};

    #[test]
    fn data_0x42() {
        check_encoding(0x42, Data(B::<24>::from(0x42u32)));
    }

    #[test]
    fn nop() {
        check_encoding(0x1_00_00_00, Nop);
    }

    #[test]
    fn step_literal() {
        check_encoding(0x1_00_00_00, Step(Operand::new(Literal, B::from(0xd))));
    }

    #[test]
    fn step_indirect() {
        check_encoding(0x1_00_00_00, Step(Operand::new(Literal, B::from(0xd))));
    }

    fn check_encoding(b28: u32, inst: Instruction) {
        let expected = B::<28>::from(b28);
        let actual = inst.encode();
        assert_eq!(expected, actual);
    }
}
