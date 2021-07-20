use wormcode_bits::B;

pub(crate) enum ArgInfo {
    Datum(B<24>),
    Operands(usize),
}
