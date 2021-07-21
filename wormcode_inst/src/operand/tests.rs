use crate::Mode;
use test_case::test_case;

#[test_case(Mode::Literal, 0x5 => 0x5)]
#[test_case(Mode::Direct, 0x5 => 0x45)]
#[test_case(Mode::Indirect, 0x5 => 0x85)]
fn test_encode_decode(m: Mode, scalarbits: u32) -> u32 {
    use crate::Operand;
    use wormcode_bits::{Decode, Encode, B};

    let op = Operand::new(m, B::<6>::from(scalarbits));
    let enc = op.encode();
    let opdec = Operand::decode_option(enc);
    assert_eq!(Some(op), opdec);

    u32::from(enc)
}
