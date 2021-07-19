mod debug;

#[cfg(test)]
mod tests;

/// A collection of N contiguous bits. N <= 32.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct B<const N: usize>(u32);

pub struct Overflow {
    bitsize: usize,
    input: u32,
}

impl<const N: usize> From<B<N>> for u32 {
    fn from(b: B<N>) -> u32 {
        b.0
    }
}

impl<const N: usize> From<u32> for B<N> {
    fn from(u: u32) -> B<N> {
        B::from_u32(u)
    }
}

impl<const N: usize> B<N> {
    pub fn concat<const J: usize, const K: usize>(a: B<J>, b: B<K>) -> B<N> {
        assert!(N <= 32);
        assert_eq!(J + K, N);
        Self::from_u32((a.0 << K) | b.0)
    }

    pub fn split<const J: usize, const K: usize>(self) -> (B<J>, B<K>) {
        assert!(N <= 32);
        assert_eq!(J + K, N);
        let a = B::<J>::from(self.0 >> K);
        let b = B::<K>::from(self.0 & ((1 << K) - 1));
        (a, b)
    }

    pub fn try_from_b<const K: usize>(other: B<K>) -> Result<Self, Overflow> {
        assert!(K > N);
        Self::try_from_u32(other.0)
    }

    pub fn from_b<const K: usize>(other: B<K>) -> Self {
        assert!(K <= N);
        Self::from_u32(other.0)
    }

    fn try_from_u32(u: u32) -> Result<Self, Overflow> {
        assert!(N <= 32);
        let cap: u64 = 1u64 << N;
        if (u as u64) < cap {
            Ok(Self(u))
        } else {
            Err(Overflow {
                bitsize: N,
                input: u,
            })
        }
    }

    fn from_u32(u: u32) -> Self {
        Self::try_from_u32(u).expect("Unhandled overflow.")
    }
}
