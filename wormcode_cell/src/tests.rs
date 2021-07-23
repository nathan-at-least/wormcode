use crate::{
    Cell,
    Direction::{self, *},
    Physics::{self, *},
};
use test_case::test_case;
use wormcode_bits::{DecodeExact, Encode, B};

#[test_case(North, 0b00)]
#[test_case(East, 0b01)]
#[test_case(South, 0b10)]
#[test_case(West, 0b11)]
fn direction_codec(d: Direction, enc: u32) {
    let enc = B::<2>::from(enc);
    assert_eq!(enc, d.encode());
    assert_eq!(d, Direction::decode_exact(enc));
}

#[test_case(Air, 0b00)]
#[test_case(Dirt, 0b01)]
#[test_case(Body, 0b10)]
#[test_case(Head, 0b11)]
fn physics_codec(p: Physics, enc: u32) {
    let enc = B::<2>::from(enc);
    assert_eq!(enc, p.encode());
    assert_eq!(p, Physics::decode_exact(enc));
}
