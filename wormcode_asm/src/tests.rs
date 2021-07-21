use crate::{parse, ParseResult};
use test_case::test_case;
use wormcode_bits::B;
use wormcode_inst::{
    Instruction::{self, Inc, Nop},
    Mode::Literal,
    Operand,
};

#[test_case("nop" => Ok(vec![Nop]))]
#[test_case(
    "nop\ninc 1 2" =>
    Ok(vec![
        Nop,
        Inc(
            Operand::new(Literal, B::from(1)),
            Operand::new(Literal, B::from(2))
            )
    ])
)]
fn test_parse(s: &str) -> ParseResult<Vec<Instruction>> {
    parse(s)
}
