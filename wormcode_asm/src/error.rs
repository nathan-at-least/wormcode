#[derive(Debug, PartialEq, Eq)]
pub enum AssembleError {
    Expected(&'static str),
    Unexpected(String),
    UnknownMnemonic(String),
    MalformedDatum(DatumAssembleError),
}

pub type AssembleResult<T> = Result<T, AssembleError>;

#[derive(Debug, PartialEq, Eq, derive_more::From)]
pub enum DatumAssembleError {
    Format(std::num::ParseIntError),
    Overflow(wormcode_bits::Overflow),
}

#[derive(Debug, derive_more::From)]
pub enum AssemblePathError {
    IO(std::io::Error),
    Assemble(AssembleError),
}

pub type AssemblePathResult<T> = Result<T, AssemblePathError>;
