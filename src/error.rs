use crate::lexer::Token;

#[derive(Debug, PartialEq)]
pub struct ParserError {
    expected: Token,
    found: Option<Token>,
}

impl ParserError {
    pub fn new(expected: Token, found: Option<Token>) -> Self {
        Self { expected, found }
    }
}
