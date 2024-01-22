use crate::token::token::{Token, TokenRange, TokenEnum};

pub enum Node {
    Statement(Statement),
    Expression(Expression),
    Program(Program),
}

#[derive(Debug, Clone)]
pub enum Statement {
    LetStatement(LetStatement),
    ReturnStatement(ReturnStatement),
    Expression(Expression),
}

#[derive(Debug, Clone)]
pub struct LetStatement {
    pub token: Token,
    pub expression: Expression,
    pub range: TokenRange,
}

#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub expression: Expression,
    pub range: TokenRange,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(Identifier),
    Literal(Literal),
    Prefix(Prefix),
    Infix(Infix),
}

#[derive(Debug, Clone)]
pub enum Literal {
    Integer(Integer),
}

#[derive(Debug, Clone)]
pub struct Prefix {
    pub token: Token,
    pub expression: Box<Expression>,
    pub range: TokenRange,
}

#[derive(Debug, Clone)]
pub struct Infix {
    pub token: Token,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub range: TokenRange,
}

#[derive(Debug, Clone)]
pub struct Integer {
    pub value: u32,
    pub range: TokenRange,
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub name: String,
    pub range: TokenRange,
}

pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Precedence {
    LOWEST,
    EQUALS,
    LessGreater, 
    SUM,
    PRODUCT,
    PREFIX,
    CALL,
    INDEX,
}

pub fn get_precedence(token: &TokenEnum) -> Precedence {
    match token {
        TokenEnum::EQ => Precedence::EQUALS,
        TokenEnum::NEQ => Precedence::EQUALS,
        TokenEnum::LT => Precedence::LessGreater,
        TokenEnum::GT => Precedence::LessGreater,
        TokenEnum::PLUS => Precedence::SUM,
        TokenEnum::MINUS => Precedence::SUM,
        TokenEnum::ASTERISK => Precedence::PRODUCT,
        TokenEnum::SLASH => Precedence::PRODUCT,
        TokenEnum::LPAREN => Precedence::CALL,
        _ => Precedence::LOWEST,
    }
}
