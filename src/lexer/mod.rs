use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use self::token::Token;

pub mod token;

pub struct Lexer {
    input: BufReader<File>,
    current_line: String,
    current_line_number: u32,
    current_char: u32,
    start_char: u32,
    tokens: Vec<token::Token>,
}

enum Line {
    Content(String),
    EOF,
}

impl Lexer {
    pub fn new(mut input: BufReader<File>) -> Lexer {
        let mut line = String::new();
        let _len = input.read_line(&mut line);
        Lexer {
            input,
            current_line_number: 1,
            current_char: 1,
            start_char: 1,
            current_line: line,
            tokens: Vec::new(),
        }
    }

    pub fn start(&mut self) {
        loop {
            self.process_line();
            let line = self.get_next_line();
            self.current_line_number += 1;
            match line {
                Line::Content(l) => self.current_line = l,
                Line::EOF => {
                    let token = Token::new(
                        token::TokenType::EOF,
                        "file".to_string(),
                        self.current_line_number,
                        1,
                        1,
                    );
                    self.tokens.push(token);
                    break;
                }
            }
        }
    }

    pub fn get_tokens(&self) -> &Vec<Token> {
        return &self.tokens;
    }

    fn get_next_line(&mut self) -> Line {
        let mut line = String::new();
        let len = self
            .input
            .read_line(&mut line)
            .expect("cound not read line");
        if len == 0 {
            return Line::EOF;
        }
        return Line::Content(line);
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
            self.start_char = self.current_char;
            let token = self.next_token(char);
            self.current_char += 1;
            self.tokens.push(token);
        }
    }

    fn is_legal_symbol(c: char) -> bool {
        let is_legal = match c {
            '=' => true,
            '+' => true,
            '-' => true,
            '!' => true,
            '*' => true,
            '/' => true,
            '<' => true,
            '>' => true,
            ',' => true,
            ';' => true,
            '\'' => true,
            '"' => true,
            '(' => true,
            ')' => true,
            '{' => true,
            '}' => true,
            _ => false,
        };
        return is_legal;
    }

    fn read_identifier(&mut self, c: char) -> token::TokenType {
        let line = &self.current_line;
        let mut s = c.to_string();
        for (i, char) in line.chars().enumerate() {
            if (i as u32) < self.current_char - 1 {
                continue;
            }
            if char.is_alphanumeric() || char == '_' {
                s += &char.to_string();
            } else if char.is_whitespace() {
                self.current_char -= 1;
                break;
            } else if Lexer::is_legal_symbol(char) {
                self.current_char -= 1;
                break;
            } else {
                s += &char.to_string();
                return token::TokenType::Illegal(s);
            }
            self.current_char += 1;
        }

        match s.as_str() {
            "let" => token::TokenType::Let,
            "fn" => token::TokenType::Function,
            &_ => token::TokenType::Identifier(s),
        }
    }

    fn read_integer(&mut self, c: char) -> token::TokenType {
        let line = &self.current_line;
        let mut s = c.to_string();
        for (i, char) in line.chars().enumerate() {
            if (i as u32) < self.current_char - 1 {
                continue;
            }
            if char.is_numeric() {
                s += &char.to_string();
            } else if char.is_whitespace() {
                self.current_char -= 1;
                break;
            } else if Lexer::is_legal_symbol(char) {
                self.current_char -= 1;
                break;
            } else {
                s += &char.to_string();
                return token::TokenType::Illegal(s);
            }
            self.current_char += 1;
        }
        let integer = s.parse::<i64>().unwrap();
        return token::TokenType::Int(integer);
    }

    fn next_token(&mut self, c: char) -> token::Token {
        let token_type = match c {
            '=' => token::TokenType::Assign,
            '+' => token::TokenType::Plus,
            '-' => token::TokenType::Minus,
            '!' => token::TokenType::Bang,
            '*' => token::TokenType::Asterisk,
            '/' => token::TokenType::Slash,
            '<' => token::TokenType::LT,
            '>' => token::TokenType::GT,
            ',' => token::TokenType::Comma,
            ';' => token::TokenType::Semicolon,
            '\'' => token::TokenType::Quate,
            '"' => token::TokenType::DoubleQuate,
            '(' => token::TokenType::LParen,
            ')' => token::TokenType::RParen,
            '{' => token::TokenType::LBrace,
            '}' => token::TokenType::RBrace,

            _ => {
                if c.is_alphabetic() {
                    self.current_char += 1;
                    self.read_identifier(c)
                } else if c.is_numeric() {
                    self.current_char += 1;
                    self.read_integer(c)
                } else {
                    token::TokenType::Illegal(c.to_string())
                }
            }
        };
        let token = token::Token::new(
            token_type,
            "file".to_string(),
            self.current_line_number,
            self.start_char,
            self.current_char,
        );
        return token;
    }
}
