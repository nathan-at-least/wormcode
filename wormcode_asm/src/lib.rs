// todo: rename parse -> assemble
mod error;
mod parseb;
mod parseinst;

#[cfg(test)]
mod tests;

pub use error::{ParseError, ParsePathError, ParsePathResult, ParseResult};
pub use parseb::DatumParseError;
pub use parseinst::parse_instruction;

use std::path::Path;
use wormcode_inst::Instruction;

pub type Assemblage = Vec<Instruction>;

pub fn parse_path(p: &Path) -> ParsePathResult<Assemblage> {
    use std::io::Read;

    let mut src = String::new();
    let mut f = std::fs::File::open(p)?;
    f.read_to_string(&mut src)?;
    let worm = parse(&src)?;
    Ok(worm)
}

pub fn parse(src: &str) -> ParseResult<Assemblage> {
    let mut insts = vec![];

    for line in src.lines() {
        insts.push(parse_instruction(line)?);
    }

    Ok(insts)
}
