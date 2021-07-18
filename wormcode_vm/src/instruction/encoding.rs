use crate::{InstG, Instruction, Operand, B};

pub fn encode(inst: Instruction) -> B<28> {
    B::<28>::from(Encoding::from(inst))
}

#[derive(Debug)]
enum Encoding {
    Data(B<24>),
    Nullary(OpCode0),
    Unary(OpCode1, Operand),
    Binary(OpCode2, Operand, Operand),
    Trinary(OpCode3, Operand, Operand, Operand),
}

#[derive(Debug)]
enum OpCode0 {
    Nop,
}

impl From<OpCode0> for B<24> {
    fn from(oc: OpCode0) -> B<24> {
        B::<24>::from(oc as u32)
    }
}

#[derive(Debug)]
enum OpCode1 {
    Step,
}

impl From<OpCode1> for B<16> {
    fn from(oc: OpCode1) -> B<16> {
        B::<16>::from(oc as u32)
    }
}

#[derive(Debug)]
enum OpCode2 {
    Inc,
}

impl From<OpCode2> for B<8> {
    fn from(oc: OpCode2) -> B<8> {
        B::<8>::from(oc as u32)
    }
}

#[derive(Debug)]
enum OpCode3 {
    // PlaceHolders reserve spine values for non-tertiary instructions:
    // Data, Nullary, Unary, Binary

    // Actual tertiary instruction opcodes:
    MemCpy = 0x4,
}

impl From<OpCode3> for B<4> {
    fn from(oc: OpCode3) -> B<4> {
        B::<4>::from(oc as u32)
    }
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

#[test]
fn test_b28_encoding_data_0xabcdef() {
    let expected = B::<28>::from(0xabcdef);
    let inst = Instruction::Data(B::<24>::from(0xabcdef));
    let enc = Encoding::from(inst);
    let b28 = B::<28>::from(enc);
    assert_eq!(expected, b28);
}
