use crate::ast::Expression;

use super::super::ast;
use super::super::lexer::token;
use super::super::lexer::token::TokenType;
use super::Parser;

pub struct ExpressionParser<'a> {
    parser: &'a mut Parser,
}

struct InfixExpressionParser {}

struct PrefixExpressionParser {}

impl InfixExpressionParser {
    pub fn parse_infix_expression(
        expre_parser: &mut ExpressionParser,
        left: ast::Expression,
        operator: token::Token,
    ) -> ast::Expression {
        println!(
            "infix parcing current token: {:?}",
            expre_parser.parser.current_token
        );
        let operator_precedence = expre_parser.get_precedence(&operator);
        let operator = operator;
        let right_token = expre_parser.parser.next_token().unwrap(); //tODO handle none
        let right = expre_parser.parse_expression(operator_precedence, right_token);
        let infix_expression = ast::InfixExpression::new(left, operator, right);
        ast::Expression::InfixExpression(Box::new(infix_expression))
    }
}

impl PrefixExpressionParser {
    pub fn parse_identifier(token: token::Token) -> ast::Expression {
        ast::Expression::Identifier(token)
    }

    pub fn parse_int(token: token::Token) -> ast::Expression {
        ast::Expression::Int(token)
    }

    pub fn parse_bool(token: token::Token) -> ast::Expression {
        ast::Expression::Boolean(token)
    }

    pub fn parse_grouped(expre_parser: &mut ExpressionParser) -> ast::Expression {
        let token = match expre_parser.parser.next_token() {
            Some(t) => t,
            None => panic!("expected expression got nothing"),
        };

        let expression = expre_parser.parse_expression(Precedence::Lowest, token);
        if !expre_parser.parser.expect_current(token::TokenType::RParen) {
            panic!("expected");
        };
        expression
    }

    pub fn parse_prefix_expression(
        expre_parser: &mut ExpressionParser,
        token: token::Token,
    ) -> ast::Expression {
        let operator = token;
        let right_token = match expre_parser.parser.next_token() {
            Some(t) => t,
            None => panic!("No right hand provided in a prefix expression"),
        };
        let right = expre_parser.parse_expression(Precedence::Prefix, right_token);
        let prefix_expression = ast::PrefixExpression::new(operator, right);
        ast::Expression::PrefixExpression(Box::new(prefix_expression))
    }
}

pub enum Precedence {
    Lowest = 1,
    Equals = 2,
    LessGreater = 3,
    Sum = 4,
    Product = 5,
    Prefix = 6,
    Call = 7,
}

impl ExpressionParser<'_> {
    pub fn new<'a>(parser: &'a mut Parser) -> ExpressionParser {
        ExpressionParser { parser }
    }

    pub fn parse_expression(
        &mut self,
        precedence: Precedence,
        token: token::Token,
    ) -> ast::Expression {
        let mut left_expression = match token.get_type() {
            token::TokenType::Identifier(..) => PrefixExpressionParser::parse_identifier(token),
            token::TokenType::Int(..) => PrefixExpressionParser::parse_int(token),
            token::TokenType::True => PrefixExpressionParser::parse_bool(token),
            token::TokenType::False => PrefixExpressionParser::parse_bool(token),
            token::TokenType::Minus => PrefixExpressionParser::parse_prefix_expression(self, token),
            token::TokenType::Bang => PrefixExpressionParser::parse_prefix_expression(self, token),
            token::TokenType::LParen => PrefixExpressionParser::parse_grouped(self),
            _ => panic!("does not supported token: {:?} yet", &token.get_type()),
        };

        let precedence = precedence as u16;
        println!("current token: {:?}", self.parser.current_token);
        println!("peek token: {:?}", self.parser.peek_token);
        while !self.parser.expect_current(token::TokenType::Semicolon)
            && (precedence < (self.assume_precedence(&self.parser.current_token) as u16))
        {
            let token = self.parser.next_token().unwrap(); //todo handle none

            left_expression = match token.get_type() {
                _ => InfixExpressionParser::parse_infix_expression(self, left_expression, token),
            }
        }

        return left_expression;
    }

    pub fn assume_precedence(&self, token: &Option<token::Token>) -> Precedence {
        let current_token = match token {
            Some(t) => t,
            None => return Precedence::Lowest,
        };

        match current_token.get_type() {
            TokenType::EQ => Precedence::Equals,
            TokenType::NotEQ => Precedence::Equals,
            TokenType::Asterisk => Precedence::Product,
            TokenType::Slash => Precedence::Product,
            TokenType::LT => Precedence::LessGreater,
            TokenType::GT => Precedence::LessGreater,
            TokenType::Plus => Precedence::Sum,
            TokenType::Minus => Precedence::Sum,
            _ => Precedence::Lowest,
        }
    }

    pub fn get_precedence(&self, token: &token::Token) -> Precedence {
        match token.get_type() {
            TokenType::EQ => Precedence::Equals,
            TokenType::NotEQ => Precedence::Equals,
            TokenType::Asterisk => Precedence::Product,
            TokenType::Slash => Precedence::Product,
            TokenType::LT => Precedence::LessGreater,
            TokenType::GT => Precedence::LessGreater,
            TokenType::Plus => Precedence::Sum,
            TokenType::Minus => Precedence::Sum,
            _ => Precedence::Lowest,
        }
    }
}
