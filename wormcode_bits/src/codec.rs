use crate::B;

pub trait Encode<const N: usize> {
    fn encode(self) -> B<N>;
}

pub trait Decode<const N: usize>: Sized {
    fn decode(b: B<N>) -> Option<Self>;
}
