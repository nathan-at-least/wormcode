/// A collection of N contiguous bits. N <= 32.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct B<const N: usize>(u32);

#[derive(Debug)]
pub struct Overflow {
    bitsize: usize,
    input: u32,
}

impl<const N: usize> From<B<N>> for u32 {
    fn from(b: B<N>) -> u32 {
        b.0
    }
}

impl<const N: usize> B<N> {
    pub fn try_from_b<const K: usize>(other: B<K>) -> Result<Self, Overflow> {
        Self::try_from_u32(other.0)
    }

    pub fn from_b<const K: usize>(other: B<K>) -> Self {
        Self::from_u32(other.0)
    }

    pub fn try_from_u32(u: u32) -> Result<Self, Overflow> {
        let cap = 1u32 << (N - 1); // BUG: N == 0 case.
        if u < cap {
            Ok(Self(u))
        } else {
            Err(Overflow {
                bitsize: N,
                input: u,
            })
        }
    }

    pub fn from_u32(u: u32) -> Self {
        Self::try_from_u32(u).expect("Unhandled overflow.")
    }

    pub fn concat<const J: usize, const K: usize>(a: B<J>, b: B<K>) -> B<N> {
        Self::from_u32((a.0 << K) & b.0)
    }
}
