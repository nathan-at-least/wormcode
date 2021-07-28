mod bits;
mod tokenstream;

#[cfg(test)]
mod tests;

use self::bits::assemble_bits;
use self::tokenstream::TokenStream;
use crate::{AssembleError, AssembleResult};
use wormcode_bits::B;
use wormcode_inst::{Instruction, Operand};

pub fn assemble_instruction(line: &str) -> AssembleResult<Instruction> {
    let line = trim_comment(line);
    InstructionAssembler::from_line(line).assemble()
}

struct InstructionAssembler<'a>(TokenStream<'a>);

impl<'a> InstructionAssembler<'a> {
    fn from_line(line: &str) -> InstructionAssembler {
        InstructionAssembler(TokenStream::from_line(line))
    }

    fn assemble(mut self) -> AssembleResult<Instruction> {
        let mnemonic = self.0.require_token("mnemonic")?;
        let inst = self.assemble_mnemonic(mnemonic)?;
        self.0.finish()?;
        Ok(inst)
    }

    fn assemble_mnemonic(&mut self, mnemonic: &str) -> AssembleResult<Instruction> {
        use Instruction::*;

        match mnemonic {
            "data" => assemble_bits(self.0.require_token("datum")?).map(Data),
            "nop" => Ok(Nop),
            "step" => {
                let a = self.assemble_operand()?;
                Ok(Step(a))
            }
            "inc" => {
                let a = self.assemble_operand()?;
                let b = self.assemble_operand()?;
                Ok(Inc(a, b))
            }
            "memcpy" => {
                let a = self.assemble_operand()?;
                let b = self.assemble_operand()?;
                let c = self.assemble_operand()?;
                Ok(MemCpy(a, b, c))
            }
            other => Err(AssembleError::UnknownMnemonic(String::from(other))),
        }
    }

    fn assemble_operand(&mut self) -> AssembleResult<Operand> {
        use wormcode_inst::Mode::*;

        fn unwrap_brackets(s: &str) -> AssembleResult<Option<&str>> {
            if let Some(t) = s.strip_prefix('[') {
                if let Some(u) = t.strip_suffix(']') {
                    Ok(Some(u))
                } else {
                    Err(AssembleError::Expected("close bracket ']'"))
                }
            } else {
                Ok(None)
            }
        }

        let s = self.0.require_token("operand")?;
        if let Some(t) = unwrap_brackets(s)? {
            if let Some(u) = unwrap_brackets(t)? {
                let b: B<6> = assemble_bits(u)?;
                Ok(Operand::new(Indirect, b))
            } else {
                let b: B<6> = assemble_bits(t)?;
                Ok(Operand::new(Direct, b))
            }
        } else {
            let b: B<6> = assemble_bits(s)?;
            Ok(Operand::new(Literal, b))
        }
    }
}

fn trim_comment(line: &str) -> &str {
    if let Some((x, _)) = line.split_once('#') {
        x
    } else {
        line
    }
    .trim()
}
