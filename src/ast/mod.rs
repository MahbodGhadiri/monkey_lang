use super::lexer::token;

#[derive(Debug)]
pub enum Statement {
    LetStatement(LetStatement),
    ReturnStatement(ReturnStatement),
    ExpressionStatement(ExpressionStatement),
}

#[derive(Debug)]
pub enum Expression {
    Int(token::Token),
    Identifier(token::Token),
    Boolean(token::Token),
    PrefixExpression(Box<PrefixExpression>),
    InfixExpression(Box<InfixExpression>),
    IfExpression(Box<IfExpression>),
    FunctionExpression(Box<FunctionLiteral>),
    Placeholder,
}

pub struct Program {
    statements: Vec<Statement>,
}

#[derive(Debug)]
pub struct PrefixExpression {
    token: token::Token, //e.g. ! or -
    right: Expression,
}

#[derive(Debug)]
pub struct InfixExpression {
    left: Expression,
    token: token::Token,
    right: Expression,
}

#[derive(Debug)]
pub struct LetStatement {
    token: token::Token,
    name: token::Token,
    value: Expression,
}

#[derive(Debug)]
pub struct IfExpression {
    token: token::Token,
    condition: Expression,
    consequence: BlockStatement,
    alternative: Option<BlockStatement>,
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

#[derive(Debug)]
pub struct BlockStatement {
    token: token::Token,
    statements: Vec<Statement>,
}

#[derive(Debug)]
pub struct FunctionLiteral {
    token: token::Token,
    parameters: Vec<Expression>,
    body: BlockStatement,
}

impl Program {
    fn get_token(&self) -> Option<&Statement> {
        if self.statements.len() > 0 {
            return Option::Some(&self.statements[0]);
        }
        Option::None
    }
}

impl LetStatement {
    pub fn new(token: token::Token, name: token::Token, value: Expression) -> LetStatement {
        LetStatement { token, name, value }
    }
}

impl IfExpression {
    pub fn new(
        token: token::Token,
        condition: Expression,
        consequence: BlockStatement,
        alternative: Option<BlockStatement>,
    ) -> IfExpression {
        IfExpression {
            token,
            condition,
            consequence,
            alternative,
        }
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

impl BlockStatement {
    pub fn new(token: token::Token, statements: Vec<Statement>) -> BlockStatement {
        BlockStatement { token, statements }
    }
}

impl PrefixExpression {
    pub fn new(token: token::Token, right: Expression) -> PrefixExpression {
        PrefixExpression { token, right }
    }
}

impl InfixExpression {
    pub fn new(left: Expression, token: token::Token, right: Expression) -> InfixExpression {
        InfixExpression { left, token, right }
    }
}

impl FunctionLiteral {
    pub fn new(
        token: token::Token,
        parameters: Vec<Expression>,
        body: BlockStatement,
    ) -> FunctionLiteral {
        FunctionLiteral {
            token,
            parameters,
            body,
        }
    }
}
