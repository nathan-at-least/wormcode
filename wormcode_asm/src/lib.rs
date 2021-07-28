pub mod error;

mod inst;

#[cfg(test)]
mod tests;

pub use error::{AssembleError, AssemblePathError, AssemblePathResult, AssembleResult};
pub use inst::assemble_instruction;

use std::path::Path;
use wormcode_inst::Instruction;

pub type Assemblage = Vec<Instruction>;

pub fn assemble_path(p: &Path) -> AssemblePathResult<Assemblage> {
    use std::io::Read;

    let mut src = String::new();
    let mut f = std::fs::File::open(p)?;
    f.read_to_string(&mut src)?;
    let worm = assemble(&src)?;
    Ok(worm)
}

pub fn assemble(src: &str) -> AssembleResult<Assemblage> {
    let mut insts = vec![];

    for line in src.lines() {
        insts.push(assemble_instruction(line)?);
    }

    Ok(insts)
}
