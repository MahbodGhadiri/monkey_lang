use crate::ast::Expression;

use super::super::ast;
use super::super::lexer::token;
struct InfixExpressionParser {}

struct PrefixExpressionParser {}

impl PrefixExpressionParser {
    pub fn parse_identifier(token: token::Token) -> ast::Expression {
        ast::Expression::Identifier(token)
    }

    pub fn parse_int(token: token::Token) -> ast::Expression {
        ast::Expression::Int(token)
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

pub fn parse_expression(precedence: Precedence, token: token::Token) -> ast::Expression {
    let left_expression = match token.get_type() {
        token::TokenType::Identifier(..) => PrefixExpressionParser::parse_identifier(token),
        token::TokenType::Int(..) => PrefixExpressionParser::parse_int(token),
        _ => panic!("not supported yet"),
    };

    return left_expression;
}
