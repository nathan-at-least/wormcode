use crate::{InstG::*, Instruction, Mode::*, Operand};
use test_case::test_case;
use wormcode_bits::B;

#[test_case(0x42, Data(B::<24>::from(0x42u32)))]
#[test_case(0x1_00_00_00, Nop)]
#[test_case(0x2_00_00_0d, Step(Operand::new(Literal, B::from(0xd))))]
#[test_case(0x2_00_00_8d, Step(Operand::new(Indirect, B::from(0xd))))]
#[test_case(
    0x3_00_03_8d,
    Inc(
        Operand::new(Literal, B::from(0x3)),
        Operand::new(Indirect, B::from(0xd)),
    )
)]
fn check_encoding(b28: u32, inst: Instruction) {
    use wormcode_bits::Decode;

    let expected = B::<28>::from(b28);
    let enc = inst.encode();
    assert_eq!(expected, enc);
    let dec = Instruction::decode(enc);
    assert_eq!(Some(inst), dec);
}
