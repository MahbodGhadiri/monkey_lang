use crate::ast::Expression;

use super::ast;
use super::lexer;
use super::lexer::token;
pub struct Parser {
    lexer: lexer::Lexer,
    current_token: lexer::token::Token,
    peek_token: lexer::token::Token,
}

impl Parser {
    pub fn new(mut lexer: lexer::Lexer) -> Parser {
        let current_token = lexer.get_next_token().unwrap(); //TODO handle better
        let peek_token = lexer.get_next_token().unwrap();

        Parser {
            current_token,
            peek_token,
            lexer,
        }
    }

    pub fn new_line(&mut self, line: String) {
        self.lexer.new_line(line);
    }

    fn next_token(&mut self) -> lexer::token::Token {
        let old_peek =
            std::mem::replace(&mut self.peek_token, self.lexer.get_next_token().unwrap());
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
        let statement = match self.current_token.get_type() {
            lexer::token::TokenType::Let => self.parse_let_statement(),
            _ => panic!("currently only support let statements"),
        };

        match statement {
            Some(s) => s,
            None => panic!("failed to parse"),
        }
    }

    fn parse_let_statement(&mut self) -> Option<ast::Statement> {
        let keyword = self.next_token();
        let node = ast::Node::new(keyword);

        if !self.expect_current(token::TokenType::Identifier("".to_string())) {
            return Option::None;
        }

        let name = self.next_token();

        //TODO: skipping the expression until semicolon
        while !(self.expect_peek(token::TokenType::Semicolon)) {
            self.next_token();
        }

        let let_statement = ast::LetStatement::new(node, name, Expression::Int); //todo add expression(this is placeholder)
        Option::Some(ast::Statement::LetStatement(let_statement))
    }

    fn expect_current(&self, t_type: token::TokenType) -> bool {
        std::mem::discriminant(self.current_token.get_type()) == std::mem::discriminant(&t_type)
    }

    fn expect_peek(&self, t_type: token::TokenType) -> bool {
        std::mem::discriminant(self.peek_token.get_type()) == std::mem::discriminant(&t_type)
    }
}
