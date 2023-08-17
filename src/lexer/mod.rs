use self::token::Token;

pub mod token;

pub struct Lexer {
    current_line: String,
    current_line_number: u32,
    current_char: u32,
    start_char: u32,
}

pub enum Line {
    Content(String),
    EOF,
}

impl Lexer {
    pub fn new(line: String) -> Lexer {
        Lexer {
            current_line_number: 1,
            current_char: 1,
            start_char: 1,
            current_line: line,
        }
    }

    pub fn new_line(&mut self, line: String) {
        self.current_line_number += 1;
        self.current_char = 1;
        self.current_line = line;
    }

    pub fn get_next_token(&mut self) -> Option<Token> {
        let line = &self.current_line;
        let mut token: Option<Token> = Option::None;
        for (i, char) in line.chars().enumerate() {
            if (i as u32) < self.current_char - 1 {
                continue;
            }
            if char.is_whitespace() {
                self.current_char += 1;
                continue;
            }
            self.start_char = self.current_char;
            token = Option::Some(self.next_token(char));
            self.current_char += 1;
            break;
        }
        token
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
            "return" => token::TokenType::Return,
            "if" => token::TokenType::If,
            "else" => token::TokenType::Else,
            "true" => token::TokenType::True,
            "false" => token::TokenType::False,
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
            '=' => {
                if self.peak_ahead() == '=' {
                    self.current_char += 1;
                    token::TokenType::EQ
                } else {
                    token::TokenType::Assign
                }
            }
            '+' => token::TokenType::Plus,
            '-' => token::TokenType::Minus,
            '!' => {
                if self.peak_ahead() == '=' {
                    self.current_char += 1;
                    token::TokenType::NotEQ
                } else {
                    token::TokenType::Bang
                }
            }
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

    // returns next character without moving current_char forward
    fn peak_ahead(&self) -> char {
        let line = self.current_line.clone();
        for (i, char) in line.chars().enumerate() {
            if (i as u32) < self.current_char {
                continue;
            }
            return char;
        }
        return ' ';
    }
}
