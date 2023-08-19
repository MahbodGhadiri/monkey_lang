use crate::ast::Expression;

use super::super::ast;
use super::super::lexer::token;
use super::Parser;

pub struct ExpressionParser<'a> {
    parser: &'a mut Parser,
}

struct InfixExpressionParser {}

struct PrefixExpressionParser {}

impl PrefixExpressionParser {
    pub fn parse_identifier(token: token::Token) -> ast::Expression {
        ast::Expression::Identifier(token)
    }

    pub fn parse_int(token: token::Token) -> ast::Expression {
        ast::Expression::Int(token)
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
        let prefix_expression = ast::PrefixExpession::new(operator, right);
        ast::Expression::PrefixExpession(Box::new(prefix_expression))
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
        let left_expression = match token.get_type() {
            token::TokenType::Identifier(..) => PrefixExpressionParser::parse_identifier(token),
            token::TokenType::Int(..) => PrefixExpressionParser::parse_int(token),
            token::TokenType::Minus => PrefixExpressionParser::parse_prefix_expression(self, token),
            token::TokenType::Bang => PrefixExpressionParser::parse_prefix_expression(self, token),
            _ => panic!("does not supported token: {:?} yet", &token.get_type()),
        };

        return left_expression;
    }
}
