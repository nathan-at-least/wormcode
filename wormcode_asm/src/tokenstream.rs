use crate::{ParseError, ParseResult};

pub(crate) struct TokenStream<'a>(std::str::SplitWhitespace<'a>);

impl<'a> TokenStream<'a> {
    pub(crate) fn from_line(line: &'a str) -> TokenStream<'a> {
        TokenStream(line.split_whitespace())
    }

    pub(crate) fn require_token(&mut self, tokname: &'static str) -> ParseResult<&'a str> {
        self.0.next().ok_or(ParseError::Expected(tokname))
    }

    pub(crate) fn finish(mut self) -> ParseResult<()> {
        if let Some(noise) = self.0.next() {
            Err(ParseError::Unexpected(String::from(noise)))
        } else {
            Ok(())
        }
    }
}
