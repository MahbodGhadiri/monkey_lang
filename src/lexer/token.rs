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
    LT,    // <
    GT,    // >
    EQ,    // ==
    NotEQ, // !=

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
    Return,
    If,
    Else,
    True,
    False,
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

    pub fn get_type(&self) -> &TokenType {
        &self.t_type
    }

    pub fn get_position(&self) -> String {
        self.line.to_string() + ":" + &self.start_char.to_string()
    }
}
