pub use crate::DatumParseError;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    Expected(&'static str),
    Unexpected(String),
    UnknownMnemonic(String),
    MalformedDatum(DatumParseError),
}

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug, derive_more::From)]
pub enum ParsePathError {
    IO(std::io::Error),
    Parse(ParseError),
}

pub type ParsePathResult<T> = Result<T, ParsePathError>;
