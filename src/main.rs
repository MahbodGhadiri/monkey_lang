pub mod ast;
pub mod lexer;
pub mod parser;
pub mod repl;

fn main() {
    repl::start();
}
