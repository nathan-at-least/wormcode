use crate::{Direction, Physics};
use wormcode_bits::{DecodeExact, Encode, B};

#[derive(Copy, Clone, Debug)]
pub struct Cell(B<32>);

impl Cell {
    pub fn physics(self) -> Physics {
        let (p, _, _) = self.fields();
        p
    }

    pub fn direction(self) -> Direction {
        let (_, d, _) = self.fields();
        d
    }

    pub fn body(self) -> B<28> {
        let (_, _, b) = self.fields();
        b
    }

    pub fn fields(self) -> (Physics, Direction, B<28>) {
        self.0.split3_decode_exact()
    }
}

impl DecodeExact<32> for Cell {
    fn decode_exact(b: B<32>) -> Cell {
        Cell(b)
    }
}

impl Encode<32> for Cell {
    fn encode(self) -> B<32> {
        self.0
    }
}
