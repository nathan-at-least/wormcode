use super::Mode::{self, Direct, Indirect, Literal};
use test_case::test_case;

#[test_case(0, Literal)]
#[test_case(1, Direct)]
#[test_case(2, Indirect)]
fn codec(exp: u32, mode: Mode) {
    use wormcode_bits::{Decode, B};

    let expected = B::<2>::from(exp);
    let encoded = B::<2>::from(mode);
    assert_eq!(expected, encoded);
    let decoded = Mode::decode(encoded).unwrap();
    assert_eq!(mode, decoded);
}
