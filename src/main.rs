use std::io::{self, Write};

fn main() {
    // Read, evaluate, print, and loop
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // For now, spit whatever was input back
        println!("{}", input);
    }
}
