use itertools::{Itertools, MultiPeek};

use crate::error::ParserError;
use crate::lexer::Token;

pub struct Parser<T>
where
    T: Iterator<Item = Token>,
{
    input: MultiPeek<T>,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Identifier(String),
    FunctionDefinition(Box<Expression>, Box<Expression>),
    FunctionApplication(Box<Expression>, Box<Expression>),
    ExpressionGroup(Box<Expression>),
    Empty,
}

macro_rules! expect {
    ($self:ident, $token:expr) => {
        if let Some(token) = $self.input.next() {
            if token == $token {
                Ok(token)
            } else {
                Err(ParserError::new($token, Some(token)))
            }
        } else {
            Err(ParserError::new($token, None))
        }
    };
}

impl<T> Parser<T>
where
    T: Iterator<Item = Token>,
{
    pub fn new(input: T) -> Self {
        Self {
            input: input.multipeek(),
        }
    }

    pub fn parse_all(&mut self) -> Vec<Expression> {
        let mut nodes = Vec::new();
        while self.input.peek().is_some() {
            self.input.reset_peek();
            nodes.push(self.parse());
        }
        nodes
    }

    pub fn parse(&mut self) -> Expression {
        // A node is basically an expression as-is, delimited by newline or EOF
        let node = self.parse_expression();

        // Consume all leftover newlines
        while let Some(Token::Newline) = self.input.peek() {
            self.input.next();
        }

        node.unwrap()
    }

    fn parse_expression(&mut self) -> Result<Expression, ParserError> {
        // An expression can be parsed by the following steps:
        //   1. Parse one element as the result
        //   2. Try parse another element
        //   3. If no element is parsed, then return the expression
        //   4. If an element is parsed, create a function application with the expression
        //   5. Go back to step 2

        // Parse one element and save it as the result
        let mut result = Ok(Expression::Empty);

        // Try parse another element
        while let Some(token) = self.input.peek() {
            match token {
                Token::Punctuation(x) if x == ")" => break,
                Token::Newline | Token::EOF => break,
                _ => {
                    let element = self.parse_element();

                    if let Ok(Expression::Empty) = result {
                        result = element;
                    } else {
                        result = Ok(Expression::FunctionApplication(
                            Box::new(result.unwrap()),
                            Box::new(element.unwrap()),
                        ));
                    }
                }
            }
        }

        result
    }

    fn parse_element(&mut self) -> Result<Expression, ParserError> {
        self.input.reset_peek();
        match self.input.peek() {
            Some(Token::Punctuation(x)) if x == "\\" => self.parse_function_definition(),
            Some(Token::Punctuation(x)) if x == "(" => self.parse_expression_group(),
            Some(Token::Identifier(_)) => self.parse_identifier(),
            Some(x) => Err(ParserError::new(x.clone(), None)),
            None => Err(ParserError::new(Token::EOF, None)),
        }
    }

    fn parse_identifier(&mut self) -> Result<Expression, ParserError> {
        // An identifier is just a string
        match self.input.next() {
            Some(Token::Identifier(s)) => Ok(Expression::Identifier(s)),
            Some(token) => Err(ParserError::new(
                Token::Identifier(String::new()),
                Some(token),
            )),
            None => Err(ParserError::new(Token::Identifier(String::new()), None)),
        }
    }

    fn parse_function_definition(&mut self) -> Result<Expression, ParserError> {
        // A function definition is:
        //   1. a backslash followed by
        //   2. an identifier followed by
        //   3. a dot followed by
        //   4. an element
        let lambda = expect!(self, Token::Punctuation(String::from("\\")));
        let argument = self.parse_identifier();
        let dot = expect!(self, Token::Punctuation(String::from(".")));
        let body = self.parse_element();

        match (lambda, argument, dot, body) {
            (Ok(_), Ok(argument), Ok(_), Ok(body)) => Ok(Expression::FunctionDefinition(
                Box::new(argument),
                Box::new(body),
            )),
            (Ok(_), Ok(_), Ok(_), x) => x,
            (Ok(_), Ok(_), x, _) => Err(x.unwrap_err()),
            (Ok(_), x, _, _) => x,
            (x, _, _, _) => Err(x.unwrap_err()),
        }
    }

    fn parse_expression_group(&mut self) -> Result<Expression, ParserError> {
        // An expression group is an expression surrounded by parentheses

        // Validate
        let lparen = expect!(self, Token::Punctuation("(".to_string()));
        let expr = self.parse_expression();
        let rparen = expect!(self, Token::Punctuation(")".to_string()));

        match (lparen, expr, rparen) {
            (Ok(_), Ok(expr), Ok(_)) => Ok(Expression::ExpressionGroup(Box::new(expr))),
            (Ok(_), Ok(_), x) => Err(x.unwrap_err()),
            (Ok(_), x, _) => x,
            (x, _, _) => Err(x.unwrap_err()),
        }
    }
}
