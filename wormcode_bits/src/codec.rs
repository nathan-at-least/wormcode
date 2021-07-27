use crate::B;

pub trait Encode<const N: usize> {
    fn encode(self) -> B<N>;
}

pub trait DecodeExact<const N: usize>: Sized {
    fn decode_exact(b: B<N>) -> Self;
}

#[derive(Debug)]
pub struct DecodeError {
    typename: &'static str,
    src: String,
}

pub type DecodeResult<T> = Result<T, DecodeError>;

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

impl<const N: usize> Encode<N> for B<N> {
    fn encode(self) -> B<N> {
        self
    }
}

impl<T, const N: usize> Decode<N> for T
where
    T: DecodeExact<N>,
{
    fn decode_option(b: B<N>) -> Option<Self> {
        Some(Self::decode_exact(b))
    }
}

impl<const N: usize> DecodeExact<N> for B<N> {
    fn decode_exact(src: B<N>) -> B<N> {
        src
    }
}
