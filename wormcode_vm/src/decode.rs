use crate::B;

// This is very similar to TryFrom, except its blanket impl for Into prevents usage.
pub trait Decode<const N: usize>: Sized {
    fn decode(b: B<N>) -> Option<Self>;
}
