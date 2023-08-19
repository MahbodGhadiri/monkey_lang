use super::lexer::token;

#[derive(Debug)]
pub enum Statement {
    LetStatement(LetStatement),
    IfStatement,
    ReturnStatement(ReturnStatement),
    ExpressionStatement(ExpressionStatement),
}

#[derive(Debug)]
pub enum Expression {
    Int(token::Token),
    Identifier(token::Token),
    Placeholder,
}

pub struct Program {
    statements: Vec<Statement>,
}

impl Program {
    fn get_token(&self) -> Option<&Statement> {
        if self.statements.len() > 0 {
            return Option::Some(&self.statements[0]);
        }
        Option::None
    }
}

#[derive(Debug)]
pub struct LetStatement {
    token: token::Token,
    name: token::Token,
    value: Expression,
}

#[derive(Debug)]
pub struct ReturnStatement {
    token: token::Token,
    value: Expression,
}

#[derive(Debug)]
pub struct ExpressionStatement {
    value: Expression,
}

impl LetStatement {
    pub fn new(token: token::Token, name: token::Token, value: Expression) -> LetStatement {
        LetStatement { token, name, value }
    }
}

impl ReturnStatement {
    pub fn new(token: token::Token, value: Expression) -> ReturnStatement {
        ReturnStatement { token, value }
    }
}

impl ExpressionStatement {
    pub fn new(value: Expression) -> ExpressionStatement {
        ExpressionStatement { value }
    }
}
