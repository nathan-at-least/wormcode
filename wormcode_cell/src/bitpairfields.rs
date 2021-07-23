use wormcode_bits::{DecodeExact, Encode, B};

macro_rules! def_bitpair_field {
    ( $t:ident, $testmod:ident, $a:ident, $b:ident, $c:ident, $d:ident ) => {
        #[derive(Copy, Clone, Debug, PartialEq, Eq)]
        pub enum $t {
            $a,
            $b,
            $c,
            $d,
        }

        impl DecodeExact<2> for $t {
            fn decode_exact(b: B<2>) -> $t {
                match u32::from(b) {
                    0b00 => $t::$a,
                    0b01 => $t::$b,
                    0b10 => $t::$c,
                    0b11 => $t::$d,
                    _ => unreachable!(),
                }
            }
        }

        impl Encode<2> for $t {
            fn encode(self) -> B<2> {
                B::from(self as u32)
            }
        }

        #[cfg(test)]
        mod $testmod {
            use super::$t;
            use test_case::test_case;
            use wormcode_bits::{DecodeExact, Encode, B};

            #[test_case($t::$a, 0b00)]
            #[test_case($t::$b, 0b01)]
            #[test_case($t::$c, 0b10)]
            #[test_case($t::$d, 0b11)]
            fn codec(x: $t, enc: u32) {
                let enc = B::<2>::from(enc);
                assert_eq!(enc, x.encode());
                assert_eq!(x, $t::decode_exact(enc));
            }
        }
    };
}

def_bitpair_field!(Physics, test_physics, Air, Dirt, Body, Head);
def_bitpair_field!(Direction, test_direction, North, East, South, West);
