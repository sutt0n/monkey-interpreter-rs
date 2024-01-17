pub type TokenType = TokenEnum;

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub range: TokenRange,
}

#[derive(Clone, Debug)]
pub struct TokenRange {
    pub start: usize,
    pub end: usize,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum TokenEnum {
    ILLEGAL,
    EOF,

    // identifiers + literals
    IDENT { name: String }, // add, foobar, x, y, ...
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

    EQ,
    NEQ,

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
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}

pub fn lookup_ident(ident: &str) -> TokenType {
    match ident {
        "fn" => TokenEnum::FUNCTION,
        "let" => TokenEnum::LET,
        "true" => TokenEnum::TRUE,
        "false" => TokenEnum::FALSE,
        "if" => TokenEnum::IF,
        "else" => TokenEnum::ELSE,
        "return" => TokenEnum::RETURN,
        _ => TokenEnum::IDENT { name: ident.to_string() },
    }
}
