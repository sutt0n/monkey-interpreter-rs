pub type TokenType = TokenEnum;

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Token {
        Token { 
            token_type, 
            literal,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum TokenEnum {
    ILLEGAL,
    EOF,

    // identifiers + literals
    IDENT(String), // add, foobar, x, y, ...
    INT(u32), // 123456

    // operators
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,

    LT,
    GT,

    // delimeters
    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // keywords
    FUNCTION,
    LET,
}

pub fn lookup_ident(ident: &str) -> TokenType {
    match ident {
        "fn" => TokenEnum::FUNCTION,
        "let" => TokenEnum::LET,
        _ => TokenEnum::IDENT(ident.to_string()),
    }
}
