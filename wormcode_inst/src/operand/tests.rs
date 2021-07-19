use crate::Mode;

#[test]
fn encode_decode_literal() {
    test_encode_decode(Mode::Literal, 0x5, 0x5);
}

#[test]
fn encode_decode_direct() {
    test_encode_decode(Mode::Direct, 0x5, 0x45);
}

#[test]
fn encode_decode_indirect() {
    test_encode_decode(Mode::Indirect, 0x5, 0x85);
}

fn test_encode_decode(m: Mode, scalarbits: u32, encbits: u32) {
    use crate::Operand;
    use wormcode_bits::Decode;
    use wormcode_bits::B;

    let op = Operand::new(m, B::<6>::from(scalarbits));
    let enc = B::<8>::from(op);
    assert_eq!(B::<8>::from(encbits), enc);
    let opdec = Operand::decode(enc);
    assert_eq!(Some(op), opdec);
}
