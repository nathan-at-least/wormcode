use crate::{error::DatumAssembleError, AssembleResult};
use wormcode_bits::B;

pub fn assemble_bits<const N: usize>(s: &str) -> AssembleResult<B<N>> {
    use crate::AssembleError::MalformedDatum;

    assemble_bits_inner(s).map_err(MalformedDatum)
}

fn assemble_bits_inner<const N: usize>(s: &str) -> Result<B<N>, DatumAssembleError> {
    use std::str::FromStr;

    let u = if let Some(t) = s.strip_prefix("0x") {
        u32::from_str_radix(t, 16)
    } else {
        u32::from_str(s)
    }?;

    let b = B::try_from_u32(u)?;
    Ok(b)
}

#[cfg(test)]
mod tests {
    use super::{
        assemble_bits_inner,
        DatumAssembleError::{self, Overflow},
    };
    use test_case::test_case;
    use wormcode_bits::Overflow as BO;
    use wormcode_bits::B;

    #[test_case("0" => Ok(B::from(0)))]
    #[test_case("0x0" => Ok(B::from(0)))]
    #[test_case("5" => Ok(B::from(5)))]
    #[test_case("0x5" => Ok(B::from(5)))]
    #[test_case("9" => Err(Overflow(BO { bitsize: 3, input: 9 })))]
    fn test_assemble(s: &str) -> Result<B<3>, DatumAssembleError> {
        assemble_bits_inner(s)
    }
}
