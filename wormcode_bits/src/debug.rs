use super::{Overflow, B};
use std::fmt;

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
