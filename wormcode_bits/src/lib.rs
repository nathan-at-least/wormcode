mod codec;
mod debug;

#[cfg(test)]
mod tests;

pub use codec::{Decode, DecodeError, DecodeResult, Encode};

/// A collection of N contiguous bits. N <= 32.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct B<const N: usize>(u32);

#[derive(PartialEq, Eq)]
pub struct Overflow {
    pub bitsize: usize,
    pub input: u32,
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
    pub fn concat<const J: usize, const K: usize>(self, suffix: B<J>) -> B<K> {
        assert!(K <= 32);
        assert_eq!(N + J, K);
        B::<K>::from_u32((self.0 << J) | suffix.0)
    }

    pub fn split_decode<X, Y, const J: usize, const K: usize>(self) -> DecodeResult<(X, Y)>
    where
        X: Decode<J>,
        Y: Decode<K>,
    {
        let (a, b) = self.split();
        let x = X::decode(a)?;
        let y = Y::decode(b)?;
        Ok((x, y))
    }

    pub fn split3_decode<X, Y, Z, const J: usize, const K: usize, const L: usize>(
        self,
    ) -> DecodeResult<(X, Y, Z)>
    where
        X: Decode<J>,
        Y: Decode<K>,
        Z: Decode<L>,
    {
        let (a, b, c) = self.split3();
        let x = X::decode(a)?;
        let y = Y::decode(b)?;
        let z = Z::decode(c)?;
        Ok((x, y, z))
    }

    pub fn split<const J: usize, const K: usize>(self) -> (B<J>, B<K>) {
        assert!(N <= 32);
        assert_eq!(J + K, N);
        let a = B::<J>::from(self.0 >> K);
        let b = B::<K>::from(self.0 & ((1 << K) - 1));
        (a, b)
    }

    pub fn split3<const J: usize, const K: usize, const L: usize>(self) -> (B<J>, B<K>, B<L>) {
        assert!(N <= 32);
        assert_eq!(J + K + L, N);
        dbg!(self);
        let a = B::<J>::from(self.0 >> (K + L));
        dbg!(a);
        let b = B::<K>::from(self.0 >> L & ((1 << K) - 1));
        dbg!(b);
        let c = B::<L>::from(self.0 & ((1 << L) - 1));
        dbg!(c);
        (a, b, c)
    }

    pub fn try_from_b<const K: usize>(other: B<K>) -> Result<Self, Overflow> {
        assert!(K > N);
        Self::try_from_u32(other.0)
    }

    pub fn from_b<const K: usize>(other: B<K>) -> Self {
        assert!(K <= N);
        Self::from_u32(other.0)
    }

    // We can't implement std::convert::TryFrom due to core blanket conflict.
    pub fn try_from_u32(u: u32) -> Result<Self, Overflow> {
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
