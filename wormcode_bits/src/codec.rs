use crate::B;

#[derive(Debug)]
pub struct DecodeError {
    typename: &'static str,
    src: String,
}

pub type DecodeResult<T> = Result<T, DecodeError>;

pub trait Encode<const N: usize> {
    fn encode(self) -> B<N>;
}

pub trait Decode<const N: usize>: Sized {
    fn decode(b: B<N>) -> DecodeResult<Self> {
        Self::decode_option(b).ok_or(DecodeError {
            typename: std::any::type_name::<Self>(),
            src: format!("{:?}", b),
        })
    }

    // Any Self that implements only this will be the `src` of the DecodeError:
    fn decode_option(b: B<N>) -> Option<Self> {
        Self::decode(b).ok()
    }
}
