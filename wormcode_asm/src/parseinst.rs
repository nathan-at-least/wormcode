mod tokenstream;

#[cfg(test)]
mod tests;

use crate::{ParseError, ParseResult};
use tokenstream::TokenStream;
use wormcode_bits::B;
use wormcode_inst::{Instruction, Operand};

pub fn parse_instruction(line: &str) -> ParseResult<Instruction> {
    let line = trim_comment(line);
    InstructionParser::from_line(line).parse()
}

struct InstructionParser<'a>(TokenStream<'a>);

impl<'a> InstructionParser<'a> {
    fn from_line(line: &str) -> InstructionParser {
        InstructionParser(TokenStream::from_line(line))
    }

    fn parse(mut self) -> ParseResult<Instruction> {
        let mnemonic = self.0.require_token("mnemonic")?;
        let inst = self.parse_mnemonic(mnemonic)?;
        self.0.finish()?;
        Ok(inst)
    }

    fn parse_mnemonic(&mut self, mnemonic: &str) -> ParseResult<Instruction> {
        use Instruction::*;

        match mnemonic {
            "data" => parse_b(self.0.require_token("datum")?).map(Data),
            "nop" => Ok(Nop),
            "step" => {
                let a = self.parse_operand()?;
                Ok(Step(a))
            }
            "inc" => {
                let a = self.parse_operand()?;
                let b = self.parse_operand()?;
                Ok(Inc(a, b))
            }
            "memcpy" => {
                let a = self.parse_operand()?;
                let b = self.parse_operand()?;
                let c = self.parse_operand()?;
                Ok(MemCpy(a, b, c))
            }
            other => Err(ParseError::UnknownMnemonic(String::from(other))),
        }
    }

    fn parse_operand(&mut self) -> ParseResult<Operand> {
        use wormcode_inst::Mode::*;

        fn unwrap_brackets(s: &str) -> ParseResult<Option<&str>> {
            if let Some(t) = s.strip_prefix('[') {
                if let Some(u) = t.strip_prefix(']') {
                    Ok(Some(u))
                } else {
                    Err(ParseError::Expected("close bracket ']'"))
                }
            } else {
                Ok(None)
            }
        }

        let s = self.0.require_token("operand")?;
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
}

fn parse_b<const N: usize>(_s: &str) -> ParseResult<B<N>> {
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
