mod parseinst;
mod tokenstream;

use wormcode_inst::Instruction;

pub use parseinst::parse_instruction;

#[derive(Debug)]
pub enum ParseError {
    Expected(&'static str),
    Unexpected(String),
    UnknownMnemonic(String),
}

pub type ParseResult<T> = Result<T, ParseError>;

pub fn parse(src: &str) -> ParseResult<Vec<Instruction>> {
    let mut insts = vec![];

    for line in src.lines() {
        insts.push(parse_instruction(line)?);
    }

    Ok(insts)
}
