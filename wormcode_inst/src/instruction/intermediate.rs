mod opcode0;
mod opcode1;
mod opcode2;
mod opcode3;

#[cfg(test)]
mod tests;

use self::{opcode0::OpCode0, opcode1::OpCode1, opcode2::OpCode2, opcode3::OpCode3};
use crate::{Instruction, Operand};
use wormcode_bits::{Decode, DecodeResult, Encode, B};

impl Encode<28> for Instruction {
    fn encode(self) -> B<28> {
        Intermediate::from(self).encode()
    }
}

impl Decode<28> for Instruction {
    fn decode(bits: B<28>) -> DecodeResult<Self> {
        Intermediate::decode(bits).map(Instruction::from)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Intermediate {
    Data(B<24>),
    Nullary(OpCode0),
    Unary(OpCode1, Operand),
    Binary(OpCode2, Operand, Operand),
    Trinary(OpCode3, Operand, Operand, Operand),
}

impl From<Instruction> for Intermediate {
    fn from(i: Instruction) -> Intermediate {
        use Intermediate::{Binary, Nullary, Trinary, Unary};

        match i {
            Instruction::Data(d) => Intermediate::Data(d),
            Instruction::Nop => Nullary(OpCode0::Nop),
            Instruction::Step(a) => Unary(OpCode1::Step, a),
            Instruction::Inc(a, b) => Binary(OpCode2::Inc, a, b),
            Instruction::MemCpy(a, b, c) => Trinary(OpCode3::MemCpy, a, b, c),
        }
    }
}

impl From<Intermediate> for Instruction {
    fn from(e: Intermediate) -> Instruction {
        use Intermediate::{Binary, Nullary, Trinary, Unary};

        match e {
            Intermediate::Data(d) => Instruction::Data(d),
            Nullary(OpCode0::Nop) => Instruction::Nop,
            Unary(OpCode1::Step, a) => Instruction::Step(a),
            Binary(OpCode2::Inc, a, b) => Instruction::Inc(a, b),
            Trinary(OpCode3::MemCpy, a, b, c) => Instruction::MemCpy(a, b, c),
        }
    }
}

impl Encode<28> for Intermediate {
    fn encode(self) -> B<28> {
        use Intermediate::*;
        match self {
            Data(d) => B::<28>::from_b(d),
            Nullary(op) => {
                let spine = B::<4>::from(0x1);
                let opcode = op.encode();
                spine.concat(opcode)
            }
            Unary(op, a) => {
                let spine = B::<4>::from(0x2);
                let opop = op.encode();
                let opa = a.encode();
                spine.concat::<16, 20>(opop).concat(opa)
            }
            Binary(op, a, b) => {
                let spine = B::<4>::from(0x3);
                let opop = op.encode();
                let opa = a.encode();
                let opb = b.encode();
                spine.concat::<8, 12>(opop).concat::<8, 20>(opa).concat(opb)
            }
            Trinary(op, a, b, c) => {
                let opop = op.encode();
                let opa = a.encode();
                let opb = b.encode();
                let opc = c.encode();
                opop.concat::<8, 12>(opa).concat::<8, 20>(opb).concat(opc)
            }
        }
    }
}

impl Decode<28> for Intermediate {
    fn decode(src: B<28>) -> DecodeResult<Self> {
        use Intermediate::*;

        let (spine, guts) = src.split::<4, 24>();

        if let Some(opc3) = OpCode3::decode_option(spine) {
            let (a, b, c): (Operand, Operand, Operand) = guts.split3_decode()?;
            Ok(Trinary(opc3, a, b, c))
        } else {
            match u32::from(spine) {
                0 => Ok(Data(guts)),
                1 => OpCode0::decode(guts).map(Nullary),
                2 => {
                    let (oc, op): (OpCode1, Operand) = guts.split_decode()?;
                    Ok(Unary(oc, op))
                }
                3 => {
                    let (oc, opa, opb) = guts.split3_decode()?;
                    Ok(Binary(oc, opa, opb))
                }
                _ => unreachable!(),
            }
        }
    }
}
