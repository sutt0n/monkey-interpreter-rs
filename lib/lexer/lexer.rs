use crate::token::token::{Token, TokenEnum, lookup_ident};

#[derive(Clone, Debug)]
pub struct Lexer {
    input: String,
    position: u32,
    read_position: u32,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer { 
            input, 
            position: 0, 
            read_position: 0, 
            ch: 0 as char
        };

        println!("lexer before: {:?}", lexer);
        let lex = lexer.read_char();
        println!("lexer after: {:?}", lex);
        lex.to_owned() 
    }

    fn read_char(&mut self) -> &mut Lexer {
        let mut this = self;
        if this.read_position >= this.input.len() as u32 {
            this.ch = 0 as char;
        } else {
            this.ch = this.input.as_bytes()[this.read_position as usize] as char;
        }
        this.position = this.read_position;
        this.read_position += 1;
        this
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }


    fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.ch {
            '=' => Token::new(TokenEnum::ASSIGN, self.ch.to_string()),
            ';' => Token::new(TokenEnum::SEMICOLON, self.ch.to_string()),
            '(' => Token::new(TokenEnum::LPAREN, self.ch.to_string()),
            ')' => Token::new(TokenEnum::RPAREN, self.ch.to_string()),
            ',' => Token::new(TokenEnum::COMMA, self.ch.to_string()),
            '+' => Token::new(TokenEnum::PLUS, self.ch.to_string()),
            '{' => Token::new(TokenEnum::LBRACE, self.ch.to_string()),
            '}' => Token::new(TokenEnum::RBRACE, self.ch.to_string()),
            '-' => Token::new(TokenEnum::MINUS, self.ch.to_string()),
            '!' => Token::new(TokenEnum::BANG, self.ch.to_string()),
            '*' => Token::new(TokenEnum::ASTERISK, self.ch.to_string()),
            '/' => Token::new(TokenEnum::SLASH, self.ch.to_string()),
            '<' => Token::new(TokenEnum::LT, self.ch.to_string()),
            '>' => Token::new(TokenEnum::GT, self.ch.to_string()),
            '\0' => {
                Token {
                    literal: "".to_string(),
                    token_type: TokenEnum::EOF,
                }

            },
            _ => {
                if self.ch.is_alphabetic() {
                    let literal = self.read_identifier();
                    println!("identifier {:?}", literal);
                    return Token::new(lookup_ident(&literal), literal.to_string())
                } else if self.ch.is_digit(10) {
                    let int = self.read_int();
                    println!("integer {:?}", int);
                    return Token::new(TokenEnum::INT(int), int.to_string())
                } else {
                    return Token::new(TokenEnum::ILLEGAL, self.ch.to_string())
                }
            },
        };

        self.read_char();
        token
    }

    fn read_int(&mut self) -> u32 {
        let pos = self.position;

        while self.ch.is_digit(10) {
            self.read_char();
        }

        let input_slice = self.input.get(pos as usize..self.position as usize).unwrap();

        input_slice.to_string().parse::<u32>().unwrap()
    }

    fn read_identifier(&mut self) -> String {
        let pos = self.position;

        while self.ch.is_alphabetic() {
            self.read_char();
        }

        let input_slice = self.input.get(pos as usize..self.position as usize).unwrap();

        input_slice.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let tests: Vec<Token> = [
            Token::new(TokenEnum::ASSIGN, '='.to_string()),
            Token::new(TokenEnum::PLUS, '+'.to_string()),
            Token::new(TokenEnum::LPAREN, '('.to_string()),
            Token::new(TokenEnum::RPAREN, ')'.to_string()),
            Token::new(TokenEnum::LBRACE, '{'.to_string()),
            Token::new(TokenEnum::RBRACE, '}'.to_string()),
            Token::new(TokenEnum::COMMA, ','.to_string()),
            Token::new(TokenEnum::SEMICOLON, ';'.to_string()),
        ].to_vec();

        let mut lex = Lexer::new("=+(){},;".to_string());

        for (_, token_type) in tests.iter().enumerate() {
            let token = lex.next_token();

            assert_eq!(token.token_type, token_type.token_type);
            assert_eq!(token.literal, *token_type.literal);
        }
    }

    #[test]
    fn test_next_token_extra() {
        let tests: Vec<Token> = [
            // let five = 5;
            Token::new(TokenEnum::LET, "let".to_string()),
            Token::new(TokenEnum::IDENT("five".to_string()), "five".to_string()),
            Token::new(TokenEnum::ASSIGN, '='.to_string()),
            Token::new(TokenEnum::INT(5), "5".to_string()),
            Token::new(TokenEnum::SEMICOLON, ';'.to_string()),
            // let ten = 10;
            Token::new(TokenEnum::LET, "let".to_string()),
            Token::new(TokenEnum::IDENT("ten".to_string()), "ten".to_string()),
            Token::new(TokenEnum::ASSIGN, '='.to_string()),
            Token::new(TokenEnum::INT(10), "10".to_string()),
            Token::new(TokenEnum::SEMICOLON, ';'.to_string()),
            // let add = fn(x, y) { x + y; };
            Token::new(TokenEnum::LET, "let".to_string()),
            Token::new(TokenEnum::IDENT("add".to_string()), "add".to_string()),
            Token::new(TokenEnum::ASSIGN, '='.to_string()),
            Token::new(TokenEnum::FUNCTION, "fn".to_string()),
            Token::new(TokenEnum::LPAREN, '('.to_string()),
            Token::new(TokenEnum::IDENT("x".to_string()), "x".to_string()),
            Token::new(TokenEnum::COMMA, ','.to_string()),
            Token::new(TokenEnum::IDENT("y".to_string()), "y".to_string()),
            Token::new(TokenEnum::RPAREN, ')'.to_string()),
            Token::new(TokenEnum::LBRACE, '{'.to_string()),
            Token::new(TokenEnum::IDENT("x".to_string()), "x".to_string()),
            Token::new(TokenEnum::PLUS, '+'.to_string()),
            Token::new(TokenEnum::IDENT("y".to_string()), "y".to_string()),
            Token::new(TokenEnum::SEMICOLON, ';'.to_string()),
            Token::new(TokenEnum::RBRACE, '}'.to_string()),
            Token::new(TokenEnum::SEMICOLON, ';'.to_string()),
            // let result = add(five, ten);
            Token::new(TokenEnum::LET, "let".to_string()),
            Token::new(TokenEnum::IDENT("result".to_string()), "result".to_string()),
            Token::new(TokenEnum::ASSIGN, '='.to_string()),
            Token::new(TokenEnum::IDENT("add".to_string()), "add".to_string()),
            Token::new(TokenEnum::LPAREN, '('.to_string()),
            Token::new(TokenEnum::IDENT("five".to_string()), "five".to_string()),
            Token::new(TokenEnum::COMMA, ','.to_string()),
            Token::new(TokenEnum::IDENT("ten".to_string()), "ten".to_string()),
            Token::new(TokenEnum::RPAREN, ')'.to_string()),
            Token::new(TokenEnum::SEMICOLON, ';'.to_string()),
            Token::new(TokenEnum::EOF, "".to_string()),
        ].to_vec();

        let mut lex = Lexer::new(r"
        let five = 5;
        let ten = 10;
        let add = fn(x, y) {
            x + y;
        };
        let result = add(five, ten);".to_string());

        for (_, token_type) in tests.iter().enumerate() {
            let token = lex.next_token();

            assert_eq!(token.token_type, token_type.token_type);
            assert_eq!(token.literal, *token_type.literal);
        }
    }

    #[test]
    fn test_extended_operators() {
        let tests: Vec<Token> = [
            // !-/*5;
            Token::new(TokenEnum::BANG, "!".to_string()),
            Token::new(TokenEnum::MINUS, "-".to_string()),
            Token::new(TokenEnum::SLASH, "/".to_string()),
            Token::new(TokenEnum::ASTERISK, "*".to_string()),
            Token::new(TokenEnum::INT(5), "5".to_string()),
            Token::new(TokenEnum::SEMICOLON, ";".to_string()),
            // 5 < 10 > 5;
            Token::new(TokenEnum::INT(5), "5".to_string()),
            Token::new(TokenEnum::LT, "<".to_string()),
            Token::new(TokenEnum::INT(10), "10".to_string()),
            Token::new(TokenEnum::GT, ">".to_string()),
            Token::new(TokenEnum::INT(5), "5".to_string()),
            Token::new(TokenEnum::SEMICOLON, ";".to_string()),
            // if (5 < 10) {
            Token::new(TokenEnum::IF, "if".to_string()),
            Token::new(TokenEnum::LPAREN, "(".to_string()),
            Token::new(TokenEnum::INT(5), "5".to_string()),
            Token::new(TokenEnum::LT, "<".to_string()),
            Token::new(TokenEnum::INT(10), "10".to_string()),
            Token::new(TokenEnum::RPAREN, ")".to_string()),
            Token::new(TokenEnum::LBRACE, "{".to_string()),
            // return true;
            Token::new(TokenEnum::RETURN, "return".to_string()),
            Token::new(TokenEnum::TRUE, "true".to_string()),
            Token::new(TokenEnum::SEMICOLON, ";".to_string()),
            // } else {
            Token::new(TokenEnum::RBRACE, "}".to_string()),
            Token::new(TokenEnum::ELSE, "else".to_string()),
            Token::new(TokenEnum::LBRACE, "{".to_string()),
            // return false;
            Token::new(TokenEnum::RETURN, "return".to_string()),
            Token::new(TokenEnum::FALSE, "false".to_string()),
            Token::new(TokenEnum::SEMICOLON, ";".to_string()),
            // }
            Token::new(TokenEnum::RBRACE, "}".to_string()),
        ].to_vec();

        let mut lex = Lexer::new(r"
        !-/*5;
        5 < 10 > 5;

        if (5 < 10) {
            return true;
        } else {
            return false;
        }".to_string());

        for (_, token_type) in tests.iter().enumerate() {
            let token = lex.next_token();

            assert_eq!(token.token_type, token_type.token_type);
            assert_eq!(token.literal, *token_type.literal);
        }
    }
}
