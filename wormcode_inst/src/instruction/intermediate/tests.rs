use super::{Intermediate, OpCode0, OpCode1, OpCode2, OpCode3};
use crate::{Mode, Operand};
use test_case::test_case;
use wormcode_bits::B;

#[test]
fn test_instruction_data_0xabcdef() {
    use crate::Instruction;
    use wormcode_bits::Encode;

    let expected = B::<28>::from(0xabcdef);
    let inst = Instruction::Data(B::<24>::from(0xabcdef));
    let enc = Intermediate::from(inst);
    let b28: B<28> = enc.encode();
    assert_eq!(expected, b28);
}

#[test_case(Intermediate::Data(B::from(0x12_34_56)))]
#[test_case(Intermediate::Data(B::from(0x65_43_21)))]
#[test_case(Intermediate::Nullary(OpCode0::Nop))]
#[test_case(Intermediate::Unary(OpCode1::Step, Operand::new(Mode::Direct, B::from(0x2a))))]
#[test_case(Intermediate::Binary(
    OpCode2::Inc,
    Operand::new(Mode::Direct, B::from(0x2a)),
    Operand::new(Mode::Literal, B::from(0x3f))
))]
#[test_case(Intermediate::Trinary(
    OpCode3::MemCpy,
    Operand::new(Mode::Direct, B::from(0x2a)),
    Operand::new(Mode::Literal, B::from(0x3f)),
    Operand::new(Mode::Indirect, B::from(0x29))
))]
fn test_encode_decode(enc: Intermediate) {
    use wormcode_bits::{Decode, Encode};

    let b: B<28> = enc.encode();
    let dec = Intermediate::decode(b);
    assert_eq!(Some(enc), dec);
}
