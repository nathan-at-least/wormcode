use std::path::Path;
use wormcode_asm::{assemble_path, Assemblage, AssemblePathResult};
use wormcode_cell::Cell;
use wormcode_prng::Prng;
use wormcode_space::{Coords, Space};

pub struct Sim {
    prng: Prng,
    space: Space<Cell>,
}

impl Sim {
    pub fn init<'a, I>(seed: u64, size: Coords, wormsrcs: I) -> AssemblePathResult<Self>
    where
        I: IntoIterator<Item = &'a Path>,
    {
        let mut prng = Prng::new(seed);
        let wormcodes = assemble_worms(wormsrcs)?;
        let space = layout(&mut prng, size, wormcodes);
        Ok(Sim { prng, space })
    }
}

fn assemble_worms<'a, I>(wormsrcs: I) -> AssemblePathResult<Vec<Assemblage>>
where
    I: IntoIterator<Item = &'a Path>,
{
    let mut wormcodes = vec![];
    for wormsrc in wormsrcs.into_iter() {
        wormcodes.push(assemble_path(wormsrc)?);
    }

    Ok(wormcodes)
}

fn layout(prng: &mut Prng, size: Coords, wormcodes: Vec<Assemblage>) -> Space<Cell> {
    let mut canvas: Space<Option<Cell>> = Space::new_empty(size);

    for wormcode in wormcodes {
        layout_wormcode(prng, &mut canvas, wormcode);
    }

    canvas.map_cells(|oc| oc.unwrap_or(gen_cell(prng)))
}

fn layout_wormcode(prng: &mut Prng, canvas: &mut Space<Option<Cell>>, wormcode: Assemblage) {
    todo!();
}

fn gen_cell(prng: &mut Prng) -> Cell {
    use wormcode_bits::{DecodeExact, Encode, B};

    let p = gen_physics(prng);
    let d: B<30> = prng.gen_bits();
    Cell::decode_exact(p.encode().concat(d))
}

fn gen_physics(prng: &mut Prng) -> wormcode_cell::Physics {
    use wormcode_cell::Physics::{Air, Body, Dirt, Head};
    let r = prng.gen_normf();
    if r < 1.0 / 81.0 {
        Head
    } else if r < 1.0 / 16.0 {
        Body
    } else if r < 1.0 / 9.0 {
        Dirt
    } else {
        Air
    }
}
