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
