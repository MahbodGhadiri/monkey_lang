#[derive(Debug)]
pub enum TokenType {
    EOF,
    Illegal(String),

    // identifiers + literals
    Int(i64),
    Identifier(String),

    // Operators
    Assign,
    Plus,
    Minus,
    Bang, // !
    Asterisk,
    Slash,
    LT, // <
    GT, // >

    // Delimeters
    Comma,
    Semicolon,
    Quate,
    DoubleQuate,

    LParen, // (
    RParen, // )
    LBrace, // {
    RBrace, // }

    // Keywords
    Function, // fn
    Let,      // let
}

#[derive(Debug)]
pub struct Token {
    t_type: TokenType,
    file: String,
    line: u32,
    start_char: u32,
    finish_char: u32,
}

impl Token {
    pub fn new(
        t_type: TokenType,
        file: String,
        line: u32,
        start_char: u32,
        finish_char: u32,
    ) -> Token {
        let token = Token {
            t_type,
            file,
            line,
            start_char,
            finish_char,
        };
        return token;
    }
}
