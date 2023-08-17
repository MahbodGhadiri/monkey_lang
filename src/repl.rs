use super::lexer;
use super::parser;
use std::io::{self, Write};

pub fn start() {
    println!("started monkey interpreter");
    let lexer = lexer::Lexer::new(read_line());
    let mut parser = parser::Parser::new(lexer);
    loop {
        parser.parse_program();
        parser.new_line(read_line());
    }
}

fn read_line() -> String {
    print!("    > ");
    io::stdout().flush().unwrap();
    let mut line = String::new();
    let l = match io::stdin().read_line(&mut line) {
        Ok(n) => {
            if n > 0 {
                line
            } else {
                panic!("EOF");
            }
        }
        Err(e) => {
            panic!("{}", e);
        }
    };
    l
}
