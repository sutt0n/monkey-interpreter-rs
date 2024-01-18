use crate::token::token::{Token, TokenEnum, lookup_ident, TokenRange};

#[derive(Clone, Debug)]
pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    read_position: usize,
    ch: char,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Self { 
            input, 
            position: 0, 
            read_position: 0, 
            ch: 0 as char
        };

        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0 as char;
        } else {
            if let Some(ch) = self.input.chars().nth(self.read_position) {
                self.ch = ch;
            } else {
                panic!("read_char failed: read out of range")
            }
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token_type = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();

                    TokenEnum::EQ
                } else {
                    TokenEnum::ASSIGN
                }
            },
            ';' => TokenEnum::SEMICOLON,
            '(' => TokenEnum::LPAREN,
            ')' => TokenEnum::RPAREN,
            ',' => TokenEnum::COMMA,
            '+' => TokenEnum::PLUS,
            '{' => TokenEnum::LBRACE,
            '}' => TokenEnum::RBRACE,
            '-' => TokenEnum::MINUS,
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();

                    TokenEnum::NEQ
                } else {
                    TokenEnum::BANG
                }
            },
            '*' => TokenEnum::ASTERISK,
            '/' => TokenEnum::SLASH,
            '<' => TokenEnum::LT,
            '>' => TokenEnum::GT,
            '\0' => TokenEnum::EOF,
            _ => {
                if self.ch.is_alphabetic() {
                    let (literal, start, end)  = self.read_identifier();
                    return Token {
                        token_type: lookup_ident(&literal),
                        range: TokenRange { start, end  }
                    };
                } else if self.ch.is_digit(10) {
                    let (int, start, end) = self.read_int();
                    return Token {
                        token_type: TokenEnum::INT(int),
                        range: TokenRange { start, end }
                    };
                } else {
                    return Token {
                        token_type: TokenEnum::ILLEGAL,
                        range: TokenRange { start: self.position, end: self.position }
                    };
                }
            },
        };

        self.read_char();
        Token {
            token_type,
            range: TokenRange { start: self.position - 1, end: self.read_position - 1 }
        }
    }

    fn read_int(&mut self) -> (u32, usize, usize) {
        let pos = self.position;

        while self.ch.is_digit(10) {
            self.read_char();
        }

        let input_slice = self.input.get(pos..self.position).unwrap();

        (input_slice.to_string().parse::<u32>().unwrap(), pos, self.position)
    }

    fn read_identifier(&mut self) -> (String, usize, usize) {
        let pos = self.position;

        while self.ch.is_alphabetic() {
            self.read_char();
        }

        let input_slice = self.input.get(pos..self.position).unwrap();

        (input_slice.to_string(), pos, self.position)
    }

    fn peek_char(&mut self) -> char {
        let read_pos = usize::try_from(self.read_position).unwrap();

        println!("read_pos {}, {}", read_pos, self.input.as_bytes()[read_pos] as char);

        if read_pos >= self.input.len() {
            '\0'
        } else {
            self.input.as_bytes()[read_pos as usize] as char
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // fn test_token_set(l: &mut Lexer) -> Vec<Token> {
    //     let mut token_vs: Vec<Token> = vec![];
    //     loop {
    //         let t = l.next_token();
    //         if t.token_type == TokenEnum::EOF {
    //             token_vs.push(t);
    //             break;
    //         } else {
    //             token_vs.push(t);
    //         }
    //     }
    //     token_vs
    // }
    //
    // pub fn test_lexer_common(name: &str, input: &str, expected: Vec<Token>) {
    //     let mut l = Lexer::new(input);
    //     let token_vs = test_token_set(&mut l);
    //
    //     for (idx, token) in token_vs.iter().enumerate() {
    //
    //     }
    // }

    #[test]
    fn test_next_token_extra() {
        println!("test_next_token_extra");
        let tests: Vec<TokenEnum> = [
            // let five = 5;
            TokenEnum::LET,
            TokenEnum::IDENT { name: "five".to_string() },
            TokenEnum::ASSIGN,
            TokenEnum::INT(5),
            TokenEnum::SEMICOLON,
            // let ten = 10;
            TokenEnum::LET,
            TokenEnum::IDENT { name: "ten".to_string() },
            TokenEnum::ASSIGN,
            TokenEnum::INT(10),
            TokenEnum::SEMICOLON,
            // let add = fn(x, y) { x + y; };
            TokenEnum::LET,
            TokenEnum::IDENT { name: "add".to_string() },
            TokenEnum::ASSIGN,
            TokenEnum::FUNCTION,
            TokenEnum::LPAREN,
            TokenEnum::IDENT { name: "x".to_string() },
            TokenEnum::COMMA,
            TokenEnum::IDENT { name: "y".to_string() },
            TokenEnum::RPAREN,
            TokenEnum::LBRACE,
            TokenEnum::IDENT { name: "x".to_string() },
            TokenEnum::PLUS,
            TokenEnum::IDENT { name: "y".to_string() },
            TokenEnum::SEMICOLON,
            TokenEnum::RBRACE,
            TokenEnum::SEMICOLON,
            // let result = add(five, ten);
            TokenEnum::LET,
            TokenEnum::IDENT { name: "result".to_string() },
            TokenEnum::ASSIGN,
            TokenEnum::IDENT { name: "add".to_string() },
            TokenEnum::LPAREN,
            TokenEnum::IDENT { name: "five".to_string() },
            TokenEnum::COMMA,
            TokenEnum::IDENT { name: "ten".to_string() },
            TokenEnum::RPAREN,
            TokenEnum::SEMICOLON,
            TokenEnum::EOF,
        ].to_vec();

        let mut lex = Lexer::new(r"
        let five = 5;
        let ten = 10;
        let add = fn(x, y) {
            x + y;
        };
        let result = add(five, ten);");

        for (_, token_type) in tests.iter().enumerate() {
            let token = lex.next_token();

            assert_eq!(token.token_type, *token_type);
        }
    }

    #[test]
    fn test_extended_operators() {
        println!("test_extended_operators");
        let tests: Vec<TokenEnum> = [
            // !-/*5;
            TokenEnum::BANG,
            TokenEnum::MINUS,
            TokenEnum::SLASH,
            TokenEnum::ASTERISK,
            TokenEnum::INT(5),
            TokenEnum::SEMICOLON,
            // 5 < 10 > 5;
            TokenEnum::INT(5),
            TokenEnum::LT,
            TokenEnum::INT(10),
            TokenEnum::GT,
            TokenEnum::INT(5),
            TokenEnum::SEMICOLON,
            // if (5 < 10) {
            TokenEnum::IF,
            TokenEnum::LPAREN,
            TokenEnum::INT(5),
            TokenEnum::LT,
            TokenEnum::INT(10),
            TokenEnum::RPAREN,
            TokenEnum::LBRACE,
            // return true;
            TokenEnum::RETURN,
            TokenEnum::TRUE,
            TokenEnum::SEMICOLON,
            // } else {
            TokenEnum::RBRACE,
            TokenEnum::ELSE,
            TokenEnum::LBRACE,
            // return false;
            TokenEnum::RETURN,
            TokenEnum::FALSE,
            TokenEnum::SEMICOLON,
            // }
            TokenEnum::RBRACE,
            // 10 == 10;
            TokenEnum::INT(10),
            TokenEnum::EQ,
            TokenEnum::INT(10),
            TokenEnum::SEMICOLON,
            // 10 != 9;
            TokenEnum::INT(10),
            TokenEnum::NEQ,
            TokenEnum::INT(9),
            TokenEnum::SEMICOLON,
        ].to_vec();

        let mut lex = Lexer::new(r"
        !-/*5;
        5 < 10 > 5;

        if (5 < 10) {
            return true;
        } else {
            return false;
        }

        10 == 10;
        10 != 9;");

        for (_, token_type) in tests.iter().enumerate() {
            let token = lex.next_token();

            assert_eq!(token.token_type, *token_type);
        }
    }
}
