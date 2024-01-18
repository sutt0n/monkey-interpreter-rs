use crate::{lexer::lexer::Lexer, token::token::{Token, TokenEnum, TokenRange}};

use super::ast::{Program, Statement, LetStatement, Expression, Identifier};

type ParsingError = String;
type ParsingErrors = Vec<ParsingError>;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
    peek_token: Token,
    errors: ParsingErrors,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();

        let errors = Vec::new();
        
        Parser { 
            lexer, 
            current_token, 
            peek_token,
            errors,
        }
    }

    pub fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn current_token_is(&self, token_type: TokenEnum) -> bool {
        self.current_token.token_type == token_type
    }

    pub fn peek_token_is(&self, token_type: &TokenEnum) -> bool {
        println!("peek_token type {:?}", self.peek_token.token_type);
        self.peek_token.token_type == *token_type
    }

    pub fn expect_peek(&mut self, token_type: &TokenEnum) -> bool {
        if self.peek_token_is(token_type) {
            self.next_token();
            true
        } else {
            self.errors.push(format!("expected next token to be {:?}, got {:?} instead", *token_type, self.peek_token.token_type));
            false
        }
    }

    pub fn parse_program(&mut self) -> Program {
        let mut statements: Vec<Statement> = Vec::new();

        while self.current_token.token_type != TokenEnum::EOF {
            if let Some(statement) = self.parse_statement() {
                println!("statement: {:?}", statement);
                statements.push(statement);
            } else {
                println!("didn't get the statement");
            }
            self.next_token();
        }

        Program {
            statements,
        }
    }

    pub fn parse_statement(&mut self) -> Option<Statement> {
        match &self.current_token.token_type {
            TokenEnum::LET => self.parse_let_statement(),
            _ => None,
        }
    }

    pub fn parse_let_statement(&mut self) -> Option<Statement> {
        let start = self.current_token.range.start;
        self.next_token();

        let current_token = self.current_token.clone();
        let mut identifier = "".to_string();

        match &self.current_token.token_type {
            TokenEnum::IDENT { name } => {
                identifier = name.to_string()
            },
            _ => {
                self.errors.push(format!("expected next token to be IDENT, got {:?} instead", self.current_token.token_type));
                return None;
            },
        }

        let expression = Expression::Identifier(
            Identifier { 
                name: identifier, 
                range: current_token.range 
            }
        );

        if !self.expect_peek(&TokenEnum::ASSIGN) {
            println!("second expect_peek failed");
            return None;
        }

        while !self.current_token_is(TokenEnum::SEMICOLON) {
            self.next_token();
        }

        let end = self.current_token.range.end;

        let statement = LetStatement {
            token: self.current_token.clone(),
            expression,
            range: TokenRange {
                start,
                end,
            },
        };

        Some(Statement::LetStatement(statement))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_let_statement() {
        let lexer = Lexer::new("
            let x = 5;
            let y = 10;
            let foobar = 838383;
        ");

        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        println!("program.statements {:?}", program.statements);
        assert_eq!(program.statements.len(), 3);
    }

    #[test]
    fn test_parse_errors() {
        let lexer = Lexer::new("
            let x 5;
            let = 10;
            let 838383;
        ");

        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        assert_eq!(program.statements.len(), 2);
        println!("parser.errors {:?}", parser.errors);
    }
}
