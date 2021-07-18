macro_rules! def_test {
    ( $name:ident, $mode:expr, $u:expr ) => {
        #[test]
        fn $name() {
            use crate::decode::Decode;
            use crate::Mode;
            use crate::B;

            let mode = $mode;
            let expected = B::<2>::from($u);
            let encoded = B::<2>::from(mode);
            assert_eq!(expected, encoded);
            let decoded = Mode::decode(encoded).unwrap();
            assert_eq!(mode, decoded);
        }
    };
}

def_test!(literal, Mode::Literal, 0);
def_test!(direct, Mode::Direct, 1);
def_test!(indirect, Mode::Indirect, 2);
