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
    char: u32,
}

impl Token {
    pub fn new(t_type: TokenType, file: String, line: u32, char: u32) -> Token {
        let token = Token {
            t_type,
            file,
            line,
            char,
        };
        return token;
    }
}
