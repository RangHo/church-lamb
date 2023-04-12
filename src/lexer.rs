use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug)]
pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(String),
    Punctuation(String),
    Space,
    Newline,
    EOF,
    Comment(String),
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars().peekable(),
        }
    }

    pub fn lex_all(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut is_eof = false;
        while !is_eof {
            let token = self.lex_next();
            if token == Token::EOF {
                is_eof = true;
            }

            tokens.push(token);
        }
        tokens
    }

    pub fn lex_next(&mut self) -> Token {
        let next_char = self.input.peek();

        match next_char {
            Some(x) if is_punctuation(x) => self.lex_punctuation(),
            Some(x) if is_whitespace(x) => self.lex_whitespace(),
            Some(x) if is_newline(x) => self.lex_newline(),
            Some(x) if x.is_alphabetic() => self.lex_identifier(),
            Some('#') => self.lex_comment(),
            None => self.lex_eof(),
            _ => panic!("Unexpected character: {:?}", next_char),
        }
    }

    fn lex_whitespace(&mut self) -> Token {
        while let Some(x) = self.input.peek() {
            if !is_whitespace(x) {
                break;
            }
            self.input.next();
        }
        Token::Space
    }

    fn lex_punctuation(&mut self) -> Token {
        let mut punctuation = String::new();
        while let Some(x) = self.input.peek() {
            if !is_punctuation(x) {
                break;
            }
            punctuation.push(*x);
            self.input.next();
        }
        Token::Punctuation(punctuation)
    }

    fn lex_identifier(&mut self) -> Token {
        let mut identifier = String::new();
        while let Some(x) = self.input.peek() {
            if !x.is_alphanumeric() {
                break;
            }
            identifier.push(*x);
            self.input.next();
        }
        Token::Identifier(identifier)
    }

    fn lex_newline(&mut self) -> Token {
        self.input.next();
        Token::Newline
    }

    fn lex_eof(&mut self) -> Token {
        Token::EOF
    }

    fn lex_comment(&mut self) -> Token {
        // Consume the pound character
        self.input.next();

        let mut comment = String::new();
        while let Some(x) = self.input.peek() {
            if is_newline(x) {
                break;
            }
            comment.push(*x);
            self.input.next();
        }
        Token::Comment(comment)
    }
}

fn is_whitespace(c: &char) -> bool {
    *c == ' ' || *c == '\t' || *c == '\r'
}

fn is_newline(c: &char) -> bool {
    *c == '\n'
}

fn is_punctuation(c: &char) -> bool {
    *c == '\\' || *c == '.'
}
