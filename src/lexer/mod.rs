use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
#[derive(Debug)]
enum TokenType {
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
struct Token {
    t_type: TokenType,
    file: String,
    line: u32,
    char: u32,
}

impl Token {
    fn new(t_type: TokenType, file: String, line: u32, char: u32) -> Token {
        let token = Token {
            t_type,
            file,
            line,
            char,
        };
        return token;
    }
}

pub struct Lexer {
    input: BufReader<File>,
    current_line: String,
    current_line_number: u32,
    current_char: u32,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(mut input: BufReader<File>) -> Lexer {
        let mut line = String::new();
        let len = input.read_line(&mut line);
        Lexer {
            input,
            current_line_number: 1,
            current_char: 1,
            current_line: line,
            tokens: Vec::new(),
        }
    }

    pub fn start(&mut self) {
        loop {
            self.process_line();
            let line = self.get_next_line();
            self.current_line = line;
            self.current_line_number += 1;
        }
    }

    fn get_next_line(&mut self) -> String {
        let mut line = String::new();
        self.input
            .read_line(&mut line)
            .expect("cound not read line");
        return line;
    }

    fn process_line(&mut self) {
        let line = &self.current_line.clone(); //TODO better than clone
        self.current_char = 1;
        for (i, char) in line.chars().enumerate() {
            if (i as u32) < self.current_char - 1 {
                continue;
            }
            if char.is_whitespace() {
                self.current_char += 1;
                continue;
            }
            let token = self.next_token(char);
            self.current_char += 1;
            println!("{:?}", token);
            self.tokens.push(token);
        }
    }

    fn read_identifier(&mut self, c: char) -> TokenType {
        let line = &self.current_line;
        let mut s = c.to_string();
        for (i, char) in line.chars().enumerate() {
            if (i as u32) < self.current_char - 1 {
                continue;
            }
            if char.is_alphanumeric() || char == '_' {
                s += &char.to_string();
            } else if char.is_whitespace() {
                break;
            } else if char == ';' {
                break;
            } else {
                s += &char.to_string();
                return TokenType::Illegal(s);
            }
            self.current_char += 1;
        }

        match s.as_str() {
            "let" => TokenType::Let,
            "fn" => TokenType::Function,
            &_ => TokenType::Identifier(s),
        }
    }

    fn read_integer(&mut self, c: char) -> TokenType {
        let line = &self.current_line;
        let mut s = c.to_string();
        for (i, char) in line.chars().enumerate() {
            if (i as u32) < self.current_char - 1 {
                continue;
            }
            if char.is_numeric() {
                s += &char.to_string();
            } else if char.is_whitespace() {
                break;
            } else if char == ';' {
                break;
            } else {
                s += &char.to_string();
                return TokenType::Illegal(s);
            }
            self.current_char += 1;
        }
        let integer = s.parse::<i64>().unwrap();
        return TokenType::Int(integer);
    }

    fn next_token(&mut self, c: char) -> Token {
        let token_type = match c {
            '=' => TokenType::Assign,
            '+' => TokenType::Plus,
            ';' => TokenType::Semicolon,
            '(' => TokenType::LParen,
            ')' => TokenType::RParen,
            ',' => TokenType::Comma,
            '{' => TokenType::LBrace,
            '}' => TokenType::RBrace,
            '\'' => TokenType::Quate,
            '"' => TokenType::DoubleQuate,
            _ => {
                if c.is_alphabetic() {
                    self.current_char += 1;
                    self.read_identifier(c)
                } else if c.is_numeric() {
                    self.current_char += 1;
                    self.read_integer(c)
                } else {
                    TokenType::Illegal(c.to_string())
                }
            }
        };
        let token = Token::new(
            token_type,
            "file".to_string(),
            self.current_line_number,
            self.current_char,
        );
        return token;
    }
}
