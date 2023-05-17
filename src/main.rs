mod error;
mod lexer;
mod parser;

use std::io::{self, Write};

use crate::lexer::Lexer;
use crate::parser::Parser;

/// Entrypoint.
fn main() {
    // Read, evaluate, print, and loop
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // For now, spit whatever was input back
        println!("Raw input: {}", input.trim());

        // Lex the input
        let mut lexer = Lexer::new(input.as_str());
        let tokens = lexer.lex_all();
        println!("List of tokens: {:?}", tokens);

        // Parse the tokens
        let mut parser = Parser::new(tokens.into_iter());
        let ast = parser.parse_all();
        println!("AST: {:?}", ast);
    }
}
