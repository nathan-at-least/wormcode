mod parseb;
mod parseinst;

#[cfg(test)]
mod tests;

pub use parseb::DatumParseError;
pub use parseinst::parse_instruction;

use wormcode_inst::Instruction;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    Expected(&'static str),
    Unexpected(String),
    UnknownMnemonic(String),
    MalformedDatum(DatumParseError),
}

pub type ParseResult<T> = Result<T, ParseError>;

pub fn parse(src: &str) -> ParseResult<Vec<Instruction>> {
    let mut insts = vec![];

    for line in src.lines() {
        insts.push(parse_instruction(line)?);
    }

    Ok(insts)
}
