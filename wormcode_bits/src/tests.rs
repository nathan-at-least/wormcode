use super::{DecodeResult, B};

macro_rules! from_into_tests {
    ( $vname:ident, $v:expr ) => {
        from_into_tests_bs!(
            $vname,
            $v,
            [
                (b2, 2),
                (b6, 6),
                (b8, 8),
                (b16, 16),
                (b24, 24),
                (b28, 28),
                (b32, 32)
            ]
        );
    };
}

macro_rules! from_into_tests_bs {
    ( $vname:ident, $v:expr, [ $( ( $nname:ident, $n:expr ) ),* ] ) => {
        mod $vname {
            $(
                #[test]
                fn $nname() {
                    let i = $v;
                    let b = crate::B::<$n>::from(i);
                    let o = u32::from(b);
                    assert_eq!(i, o);
                }
            )*
        }
    };
}

from_into_tests!(from_into_0, 0u32);
from_into_tests!(from_into_1, 1u32);
from_into_tests!(from_into_2, 2u32);
from_into_tests!(from_into_3, 3u32);

#[test]
fn concat_2_3() {
    let a: B<5> = B::<2>::from(0x2).concat(B::<3>::from(0x6));
    let b = B::<5>::from(0x16);
    assert_eq!(a, b);
}

#[test]
fn split_2_3() {
    let x = B::<5>::from(0x16);
    let (a, b): (B<2>, B<3>) = x.split();
    let au32 = u32::from(a);
    let bu32 = u32::from(b);
    assert_eq!(au32, 0x2);
    assert_eq!(bu32, 0x6);

    // Test alt syntax:
    let (a2, b2) = x.split::<2, 3>();
    assert_eq!(a, a2);
    assert_eq!(b, b2);
}

#[test]
fn split3_2_5_3() {
    let x = B::<10>::from(0x316);
    let (a, b, c) = x.split3::<2, 5, 3>();
    assert_eq!(u32::from(a), 0x3);
    assert_eq!(u32::from(b), 0x2);
    assert_eq!(u32::from(c), 0x6);
}

use crate::Decode;

#[derive(Debug, PartialEq, Eq)]
struct U32w(u32);

impl Decode<3> for U32w {
    fn decode_option(b: B<3>) -> Option<U32w> {
        Some(U32w(u32::from(b)))
    }
}

#[test]
fn split_decode_3_3() -> DecodeResult<()> {
    let src = B::<6>::from(0x2a);
    let res: (U32w, U32w) = src.split_decode()?;
    assert_eq!(res, (U32w(0x5), U32w(0x2)));
    Ok(())
}

#[test]
fn split3_decode_3_3_3() -> DecodeResult<()> {
    let src = B::<9>::from(0xba);
    let res: (U32w, U32w, U32w) = src.split3_decode()?;
    assert_eq!(res, (U32w(0x2), U32w(0x7), U32w(0x2)));
    Ok(())
}
