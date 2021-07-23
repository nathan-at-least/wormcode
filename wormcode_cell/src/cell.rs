use crate::{Direction, Physics};
use wormcode_bits::{Decode, DecodeResult, Encode, B};

#[derive(Debug)]
pub struct Cell {
    physics: Physics,
    tailward: Direction,
    composition: B<28>,
}

impl Decode<32> for Cell {
    fn decode(b: B<32>) -> DecodeResult<Cell> {
        let (physics, tailward, composition) = b.split3_decode()?;
        Ok(Cell {
            physics,
            tailward,
            composition,
        })
    }
}

impl Encode<32> for Cell {
    fn encode(self) -> B<32> {
        let physbits: B<4> = self.physics.encode().concat(self.tailward.encode());
        physbits.concat(self.composition)
    }
}
