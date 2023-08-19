pub mod expression_parser;

use super::ast;
use super::lexer;
use super::lexer::token;
use crate::ast::Expression;

pub struct Parser {
    lexer: lexer::Lexer,
    current_token: Option<lexer::token::Token>,
    peek_token: Option<lexer::token::Token>,
    parsed_partial: bool,
}

impl Parser {
    pub fn new(mut lexer: lexer::Lexer) -> Parser {
        let current_token = lexer.get_next_token();
        let peek_token = lexer.get_next_token();

        Parser {
            current_token,
            peek_token,
            lexer,
            parsed_partial: false,
        }
    }

    pub fn new_line(&mut self, line: String) {
        self.lexer.new_line(line);
        if !self.parsed_partial {
            self.next_token();
            self.next_token();
        }
    }

    fn next_token(&mut self) -> Option<lexer::token::Token> {
        let old_peek = std::mem::replace(&mut self.peek_token, self.lexer.get_next_token());
        let old_current = std::mem::replace(&mut self.current_token, old_peek);
        return old_current;
    }

    pub fn parse_program(&mut self) {
        //for test
        let statement = self.parse_statement();
        println!("statement");
        println!("{:?}", statement);
    }

    fn parse_statement(&mut self) -> ast::Statement {
        let current_token = match &self.current_token {
            Some(t) => t,
            None => panic!("current token is None"),
        };
        let statement = match current_token.get_type() {
            lexer::token::TokenType::Let => self.parse_let_statement(),
            lexer::token::TokenType::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        };

        match statement {
            Some(s) => s,
            None => panic!("failed to parse"),
        }
    }

    fn parse_let_statement(&mut self) -> Option<ast::Statement> {
        let let_token = self.next_token().unwrap(); // None handled in parse_statement function

        if !self.expect_current(token::TokenType::Identifier("".to_string())) {
            return Option::None;
        }

        let name = match self.next_token() {
            Some(t) => t,
            None => panic!("Expected token::Token but got None"),
        };

        //TODO: skipping the expression until semicolon
        while !(self.expect_peek(token::TokenType::Semicolon)) {
            self.next_token();
        }

        let let_statement = ast::LetStatement::new(let_token, name, Expression::Placeholder); //todo add expression(this is placeholder)
        self.parsed_partial = false;
        Option::Some(ast::Statement::LetStatement(let_statement))
    }

    fn parse_return_statement(&mut self) -> Option<ast::Statement> {
        let return_token = self.next_token().unwrap(); // None handled in parse_statement function

        //TODO: skipping the expression until semicolon
        while !self.expect_peek(token::TokenType::Semicolon) {
            self.next_token();
        }
        let return_statement = ast::ReturnStatement::new(return_token, Expression::Placeholder);
        self.parsed_partial = false;
        Option::Some(ast::Statement::ReturnStatement(return_statement))
    }

    fn parse_expression_statement(&mut self) -> Option<ast::Statement> {
        let token = self.next_token().unwrap(); // None handled in parse_statement function
        let mut expre_parser = expression_parser::ExpressionParser::new(self);
        let expression =
            expre_parser.parse_expression(expression_parser::Precedence::Lowest, token);

        if self.expect_peek(token::TokenType::Semicolon) {
            self.parsed_partial = false;
        };

        let expression_statement = ast::ExpressionStatement::new(expression);
        Option::Some(ast::Statement::ExpressionStatement(expression_statement))
    }

    fn expect_current(&self, t_type: token::TokenType) -> bool {
        let t = match &self.current_token {
            Some(t) => t,
            None => {
                return false;
            }
        };
        std::mem::discriminant(t.get_type()) == std::mem::discriminant(&t_type)
    }

    fn expect_peek(&self, t_type: token::TokenType) -> bool {
        let t = match &self.peek_token {
            Some(t) => t,
            None => {
                return false;
            }
        };
        std::mem::discriminant(t.get_type()) == std::mem::discriminant(&t_type)
    }
}
