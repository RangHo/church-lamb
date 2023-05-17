use std::iter::Peekable;
use std::str::Chars;

/// Lexer object.
///
/// This struct is used to tokenize a raw string input into a set of tokens.
///
/// # Examples
///
/// ```rust
/// // Create a new lexer object
/// let mut lexer = Lexer::new("\x.x");
///
/// // Tokenize one token
/// let token = lexer.lex();
///
/// // Tokenize until the end of string
/// let tokens = lexer.lex_all();
/// ```
#[derive(Debug)]
pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

/// The token type.
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Identifier(String),
    Punctuation(String),
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
            let token = self.lex();
            if token == Token::EOF {
                is_eof = true;
            }

            tokens.push(token);
        }
        tokens
    }

    pub fn lex(&mut self) -> Token {
        let next_char = self.input.peek();

        match next_char {
            Some(x) if is_punctuation(x) => self.lex_punctuation(),
            Some(x) if is_newline(x) => self.lex_newline(),
            Some(x) if x.is_alphabetic() => self.lex_identifier(),
            Some(x) if is_whitespace(x) => {
                self.input.next();
                self.lex()
            }
            Some('#') => self.lex_comment(),
            None => self.lex_eof(),
            _ => panic!("Unexpected character: {:?}", next_char),
        }
    }

    fn lex_punctuation(&mut self) -> Token {
        Token::Punctuation(self.input.next().unwrap().to_string())
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
    *c == '\\' || *c == '.' || *c == '(' || *c == ')'
}
