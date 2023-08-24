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

    pub fn parse_if(expre_parser: &mut ExpressionParser, token: token::Token) -> ast::Expression {
        let if_token = token;

        if !expre_parser.parser.expect_current(token::TokenType::LParen) {
            panic!("expected ( but got {:?}", expre_parser.parser.current_token);
        };
        expre_parser.parser.next_token(); //drop lpar

        let condition_token = match expre_parser.parser.next_token() {
            Some(t) => t,
            None => panic!(),
        };
        let condition = expre_parser.parse_expression(Precedence::Lowest, condition_token);

        if !expre_parser.parser.expect_current(token::TokenType::RParen) {
            panic!("expected ) but got {:?}", expre_parser.parser.current_token);
        };

        expre_parser.parser.next_token(); //drop rpar

        if !expre_parser.parser.expect_current(token::TokenType::LBrace) {
            panic!("expected ) but got {:?}", expre_parser.parser.current_token);
        };

        let consequence = expre_parser.parser.parse_block_statement();

        let mut alternative: Option<ast::BlockStatement> = None;

        if expre_parser.parser.expect_current(token::TokenType::Else) {
            if !expre_parser.parser.expect_peek(token::TokenType::LBrace) {
                panic!("expected ( but got {:?}", expre_parser.parser.peek_token);
            };
            expre_parser.parser.next_token(); //drop else
            alternative = Some(expre_parser.parser.parse_block_statement());
        }

        let if_expression = ast::IfExpression::new(if_token, condition, consequence, alternative);
        ast::Expression::IfExpression(Box::new(if_expression))
    }

    pub fn parse_fn(expre_parser: &mut ExpressionParser, token: token::Token) -> ast::Expression {
        let func_name = expre_parser.parser.next_token().unwrap(); //handle None
        if !expre_parser.parser.expect_current(token::TokenType::LParen) {
            panic!("expected ( but got {:?}", expre_parser.parser.current_token);
        };
        expre_parser.parser.next_token(); //drop (
        let mut parameters: Vec<ast::Expression> = Vec::new();

        if !(expre_parser.parser.expect_current(token::TokenType::RParen)) {
            //TODO check if it is identifier
            //TODO handle none
            let ident = ast::Expression::Identifier(expre_parser.parser.next_token().unwrap());
            parameters.push(ident);
            while expre_parser.parser.expect_current(token::TokenType::Comma) {
                //drop comma
                expre_parser.parser.next_token();
                //TODO check if it is identifier
                //TODO handle none
                let ident = ast::Expression::Identifier(expre_parser.parser.next_token().unwrap());
                parameters.push(ident);
            }
        }

        if !expre_parser.parser.expect_current(token::TokenType::RParen) {
            panic!("expected ) but got {:?}", expre_parser.parser.current_token)
        }

        expre_parser.parser.next_token(); //drop )

        if !expre_parser.parser.expect_current(token::TokenType::LBrace) {
            panic!(
                "expected {{ but got {:?}",
                expre_parser.parser.current_token
            );
        };
        let body = expre_parser.parser.parse_block_statement();
        let function_literal = ast::FunctionLiteral::new(func_name, parameters, body);
        ast::Expression::FunctionExpression(Box::new(function_literal))
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
            token::TokenType::If => PrefixExpressionParser::parse_if(self, token),
            token::TokenType::Function => PrefixExpressionParser::parse_fn(self, token),
            _ => panic!("does not supported token: {:?} yet", &token.get_type()),
        };

        let precedence = precedence as u16;
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
