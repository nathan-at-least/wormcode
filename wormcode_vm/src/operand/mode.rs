use crate::B;

#[derive(Debug)]
pub enum Mode {
    Literal,
    Direct,
    Indirect,
}

impl From<Mode> for B<2> {
    fn from(m: Mode) -> B<2> {
        B::<2>::from(m as u32)
    }
}

#[test]
fn b2_from_literal() {
    assert_eq!(B::<2>::from(0), B::<2>::from(Mode::Literal));
}

#[test]
fn b2_from_direct() {
    assert_eq!(B::<2>::from(1), B::<2>::from(Mode::Direct));
}

#[test]
fn b2_from_indirect() {
    assert_eq!(B::<2>::from(2), B::<2>::from(Mode::Indirect));
}
