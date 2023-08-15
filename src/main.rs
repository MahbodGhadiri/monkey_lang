pub mod lexer;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let f = File::open("monkey.txt")?;
    let mut reader = BufReader::new(f);

    let mut lexer = lexer::Lexer::new(reader);
    lexer.start();
    Ok(())
}
