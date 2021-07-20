mod arginfo;

use wormcode_bits::B;
use wormcode_inst::{Instruction, Operand};

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
        let line = trim_comment(line);
        let mut stream = TokenStream(line.split_whitespace());
        let mnemonic = stream.require_token("mnemonic")?;
        let inst = parse_instruction(&mut stream, mnemonic)?;
        stream.finish()?;
    }

    Ok(insts)
}

fn parse_instruction<'a>(stream: &mut TokenStream<'a>, mnemonic: &str) -> ParseResult<Instruction> {
    use Instruction::*;

    match mnemonic {
        "data" => parse_b(stream.require_token("datum")?).map(Data),
        "nop" => Ok(Nop),
        "step" => {
            let a = parse_operand(stream)?;
            Ok(Step(a))
        }
        "inc" => {
            let a = parse_operand(stream)?;
            let b = parse_operand(stream)?;
            Ok(Inc(a, b))
        }
        "memcpy" => {
            let a = parse_operand(stream)?;
            let b = parse_operand(stream)?;
            let c = parse_operand(stream)?;
            Ok(MemCpy(a, b, c))
        }
        other => Err(ParseError::UnknownMnemonic(String::from(other))),
    }
}

fn parse_operand<'a>(stream: &mut TokenStream<'a>) -> ParseResult<Operand> {
    use wormcode_inst::Mode::*;

    fn unwrap_brackets(s: &str) -> ParseResult<Option<&str>> {
        if let Some(t) = s.strip_prefix('[') {
            if let Some(u) = s.strip_prefix(']') {
                Ok(Some(u))
            } else {
                Err(ParseError::Expected("close bracket ']'"))
            }
        } else {
            Ok(None)
        }
    }

    let s = stream.require_token("operand")?;
    if let Some(t) = unwrap_brackets(s)? {
        if let Some(u) = unwrap_brackets(t)? {
            let b: B<6> = parse_b(u)?;
            Ok(Operand::new(Indirect, b))
        } else {
            let b: B<6> = parse_b(t)?;
            Ok(Operand::new(Direct, b))
        }
    } else {
        let b: B<6> = parse_b(s)?;
        Ok(Operand::new(Literal, b))
    }
}

fn parse_b<const N: usize>(s: &str) -> ParseResult<B<N>> {
    todo!();
}

fn trim_comment(line: &str) -> &str {
    if let Some((x, _)) = line.split_once('#') {
        x
    } else {
        line
    }
    .trim()
}

struct TokenStream<'a>(std::str::SplitWhitespace<'a>);

impl<'a> TokenStream<'a> {
    fn require_token(&mut self, tokname: &'static str) -> ParseResult<&'a str> {
        self.0.next().ok_or(ParseError::Expected(tokname))
    }

    fn finish(mut self) -> ParseResult<()> {
        if let Some(noise) = self.0.next() {
            Err(ParseError::Unexpected(String::from(noise)))
        } else {
            Ok(())
        }
    }
}
