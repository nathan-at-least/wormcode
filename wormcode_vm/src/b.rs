use std::fmt;

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

impl<const N: usize> fmt::Debug for B<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut bits = vec![];
        let mut n = self.0;
        for i in 0..N {
            bits.push(if n % 2 == 0 { "0" } else { "1" });
            n >>= 1;
            if (i + 1) % 4 == 0 {
                bits.push(" ");
                if (i + 1) % 8 == 0 {
                    bits.push(" ");
                }
            }
        }

        bits.reverse();

        write!(f, "B<{}>({})", N, bits.concat())
    }
}

impl fmt::Debug for Overflow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        struct Hexify(u32);

        impl fmt::Debug for Hexify {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "0x{:x}", self.0)
            }
        }

        f.debug_struct("Overflow")
            .field("bitsize", &self.bitsize)
            .field("input", &Hexify(self.input))
            .finish()
    }
}
