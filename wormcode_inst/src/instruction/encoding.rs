mod opcode0;
mod opcode1;
mod opcode2;
mod opcode3;

#[cfg(test)]
mod tests;

use self::{opcode0::OpCode0, opcode1::OpCode1, opcode2::OpCode2, opcode3::OpCode3};
use crate::{InstG, Instruction, Operand};
use wormcode_bits::{Decode, B};

pub fn encode(inst: Instruction) -> B<28> {
    B::<28>::from(Encoding::from(inst))
}

pub fn decode(bits: B<28>) -> Option<Instruction> {
    Encoding::decode(bits).map(Instruction::from)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Encoding {
    Data(B<24>),
    Nullary(OpCode0),
    Unary(OpCode1, Operand),
    Binary(OpCode2, Operand, Operand),
    Trinary(OpCode3, Operand, Operand, Operand),
}

impl From<Instruction> for Encoding {
    fn from(i: Instruction) -> Encoding {
        use Encoding::{Binary, Nullary, Trinary, Unary};

        match i {
            InstG::Data(d) => Encoding::Data(d),
            InstG::Nop => Nullary(OpCode0::Nop),
            InstG::Step(a) => Unary(OpCode1::Step, a),
            InstG::Inc(a, b) => Binary(OpCode2::Inc, a, b),
            InstG::MemCpy(a, b, c) => Trinary(OpCode3::MemCpy, a, b, c),
        }
    }
}

impl From<Encoding> for Instruction {
    fn from(e: Encoding) -> Instruction {
        use Encoding::{Binary, Nullary, Trinary, Unary};

        match e {
            Encoding::Data(d) => InstG::Data(d),
            Nullary(OpCode0::Nop) => InstG::Nop,
            Unary(OpCode1::Step, a) => InstG::Step(a),
            Binary(OpCode2::Inc, a, b) => InstG::Inc(a, b),
            Trinary(OpCode3::MemCpy, a, b, c) => InstG::MemCpy(a, b, c),
        }
    }
}

impl From<Encoding> for B<28> {
    fn from(e: Encoding) -> B<28> {
        use Encoding::*;
        match e {
            Data(d) => B::<28>::from_b(d),
            Nullary(op) => {
                let spine = B::<4>::from(0x1);
                let opcode = B::<24>::from(op);
                B::<28>::concat(spine, opcode)
            }
            Unary(op, a) => {
                let spine = B::<4>::from(0x2);
                let op = B::<16>::from(op);
                let opa = B::<8>::from(a);
                B::<28>::concat(spine, B::<24>::concat(op, opa))
            }
            Binary(op, a, b) => {
                let spine = B::<4>::from(0x3);
                let op = B::<8>::from(op);
                let opa = B::<8>::from(a);
                let opb = B::<8>::from(b);
                B::<28>::concat(spine, B::<24>::concat(op, B::<16>::concat(opa, opb)))
            }
            Trinary(op, a, b, c) => {
                let op = B::<4>::from(op);
                let opa = B::<8>::from(a);
                let opb = B::<8>::from(b);
                let opc = B::<8>::from(c);
                B::<28>::concat(op, B::<24>::concat(opa, B::<16>::concat(opb, opc)))
            }
        }
    }
}

impl Decode<28> for Encoding {
    fn decode(src: B<28>) -> Option<Self> {
        use Encoding::*;

        let (spine, guts): (B<4>, B<24>) = src.split();

        if let Some(opc3) = OpCode3::decode(spine) {
            let (a, bc): (B<8>, B<16>) = guts.split();
            let (b, c): (B<8>, B<8>) = bc.split();
            match (Operand::decode(a), Operand::decode(b), Operand::decode(c)) {
                (Some(opa), Some(opb), Some(opc)) => Some(Trinary(opc3, opa, opb, opc)),
                _ => None,
            }
        } else {
            match u32::from(spine) {
                0 => Some(Data(guts)),
                1 => OpCode0::decode(guts).map(Nullary),
                2 => {
                    let (boc, bop): (B<16>, B<8>) = guts.split();
                    OpCode1::decode(boc)
                        .zip(Operand::decode(bop))
                        .map(|(oc, op)| Unary(oc, op))
                }
                3 => {
                    let (boc, bopab): (B<8>, B<16>) = guts.split();
                    let (bopa, bopb): (B<8>, B<8>) = bopab.split();
                    OpCode2::decode(boc)
                        .zip(Operand::decode(bopa))
                        .zip(Operand::decode(bopb))
                        .map(|((oc, opa), opb)| Binary(oc, opa, opb))
                }
                _ => unreachable!(),
            }
        }
    }
}
