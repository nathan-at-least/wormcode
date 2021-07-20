use crate::DatumParseError::Overflow as DPEOverflow;
use crate::{ParseError::MalformedDatum, ParseResult};
use test_case::test_case;
use wormcode_bits::{Overflow, B};
use wormcode_inst::{
    Instruction::{self, *},
    Mode::*,
    Operand,
};

#[test_case("data 42" => Ok(Data(B::from(42))))]
#[test_case("data 0x42" => Ok(Data(B::from(0x42))))]
#[test_case("data 0x12345678" => Err(MalformedDatum(DPEOverflow(Overflow { bitsize: 24, input: 0x12_34_56_78 }))))]
#[test_case("nop" => Ok(Nop))]
#[test_case(
    "step 1" =>
    Ok(Step(Operand::new(Literal, B::from(1))))
)]
#[test_case(
    "step [3]" =>
    Ok(Step(Operand::new(Direct, B::from(3))))
)]
#[test_case(
    "step [[5]]" =>
    Ok(Step(Operand::new(Indirect, B::from(5))))
)]
#[test_case(
    "inc 1 2" =>
    Ok(
        Inc(
            Operand::new(Literal, B::from(1)),
            Operand::new(Literal, B::from(2))
        )
    )
)]
#[test_case(
    "memcpy 1 2 3" =>
    Ok(
        MemCpy(
            Operand::new(Literal, B::from(1)),
            Operand::new(Literal, B::from(2)),
            Operand::new(Literal, B::from(3))
        )
    )
)]
fn parse_instruction(src: &str) -> ParseResult<Instruction> {
    crate::parse_instruction(src)
}
