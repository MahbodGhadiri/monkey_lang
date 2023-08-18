use super::lexer::token;

#[derive(Debug)]
pub struct Node {
    token: token::Token,
}

impl Node {
    pub fn new(token: token::Token) -> Node {
        Node { token }
    }

    fn get_token(&self) -> &token::Token {
        return &self.token;
    }
}

#[derive(Debug)]
pub enum Statement {
    LetStatement(LetStatement),
    IfStatement,
    ReturnStatement(ReturnStatement),
}

#[derive(Debug)]
pub enum Expression {
    Int,
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
    node: Node,
    name: token::Token,
    value: Expression,
}

#[derive(Debug)]
pub struct ReturnStatement {
    node: Node,
    value: Expression,
}

impl LetStatement {
    pub fn new(node: Node, name: token::Token, value: Expression) -> LetStatement {
        LetStatement { node, name, value }
    }
}

impl ReturnStatement {
    pub fn new(node: Node, value: Expression) -> ReturnStatement {
        ReturnStatement { node, value }
    }
}
