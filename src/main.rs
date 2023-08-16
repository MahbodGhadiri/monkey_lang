pub mod lexer;

use std::fs::File;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let f = File::open("example.monkey").unwrap();
    let mut reader = BufReader::new(f);

    let mut lexer = lexer::Lexer::new(reader);
    lexer.start();
    for token in lexer.get_tokens() {
        println!("{:?}", token);
    }
    Ok(())
}
