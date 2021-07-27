use rand_chacha::ChaCha8Rng;
use wormcode_bits::B;

pub struct Prng(ChaCha8Rng);

impl Prng {
    pub fn new(seed: u64) -> Prng {
        use rand_chacha::rand_core::SeedableRng;

        Prng(ChaCha8Rng::seed_from_u64(seed))
    }

    pub fn gen_bits<const N: usize>(&mut self) -> B<N> {
        use rand_chacha::rand_core::RngCore;

        let u = self.0.next_u32();
        B::from(u & ((1 << N) - 1))
    }

    pub fn gen_normf(&mut self) -> f64 {
        use rand::Rng;

        self.0.gen_range(0.0..1.0)
    }
}
