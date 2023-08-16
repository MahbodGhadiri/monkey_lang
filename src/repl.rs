use super::lexer;
use std::io::{self, Write};

pub fn start() {
    let mut lexer = lexer::Lexer::new();
    let mut current_token: usize = 0;
    println!("started monkey interpreter");
    loop {
        print!("    > ");
        io::stdout().flush();
        let mut line = String::new();
        let l = match io::stdin().read_line(&mut line) {
            Ok(n) => {
                if n > 0 {
                    lexer::Line::Content(line)
                } else {
                    lexer::Line::EOF
                }
            }
            Err(e) => {
                panic!("{}", e);
            }
        };
        lexer.process_line(l);
        let tokens = lexer.get_tokens();
        while current_token < tokens.len() {
            println!("{:?}", tokens[current_token]);
            current_token += 1;
        }
    }
}
