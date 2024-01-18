use crate::{token::token::{Token, TokenRange}, lexer::lexer::Lexer};


pub enum Node {
    Statement(Statement),
    Expression(Expression),
    Program(Program),
}

#[derive(Debug, Clone)]
pub enum Statement {
    LetStatement(LetStatement),
}

#[derive(Debug, Clone)]
pub struct LetStatement {
    pub token: Token,
    pub expression: Expression,
    pub range: TokenRange,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(Identifier),
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub name: String,
    pub range: TokenRange,
}

pub struct Program {
    pub statements: Vec<Statement>,
}

// impl Program {
//     pub fn new(lexer: Lexer) -> Self {
//         Program {
//             statements,
//         }
//     }
// }
