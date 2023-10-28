use std::io::stdin;

use micro::{Lexeme, Lexer};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer)?;

    let mut lexer = Lexer::new(&buffer);

    loop {
        let lexeme = lexer.next();
        println!("{:?}", lexeme);

        match lexeme {
            Lexeme::Illegal(_) | Lexeme::EndOfFile(_) => break,
            _ => {}
        }
    }

    Ok(())
}
