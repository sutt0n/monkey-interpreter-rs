use core::fmt;

pub type TokenType = TokenEnum;

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub range: TokenRange,
}

#[derive(Copy, Clone, Debug)]
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

impl fmt::Display for TokenEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenEnum::ILLEGAL => write!(f, "ILLEGAL"),
            TokenEnum::EOF => write!(f, "EOF"),
            TokenEnum::IDENT { name } => write!(f, "{}", name),
            TokenEnum::INT(int) => write!(f, "{}", int),
            TokenEnum::ASSIGN => write!(f, "="),
            TokenEnum::PLUS => write!(f, "+"),
            TokenEnum::MINUS => write!(f, "-"),
            TokenEnum::BANG => write!(f, "!"),
            TokenEnum::ASTERISK => write!(f, "*"),
            TokenEnum::SLASH => write!(f, "/"),
            TokenEnum::LT => write!(f, "<"),
            TokenEnum::GT => write!(f, ">"),
            TokenEnum::EQ => write!(f, "=="),
            TokenEnum::NEQ => write!(f, "!="),
            TokenEnum::COMMA => write!(f, ","),
            TokenEnum::SEMICOLON => write!(f, ";"),
            TokenEnum::LPAREN => write!(f, "("),
            TokenEnum::RPAREN => write!(f, ")"),
            TokenEnum::LBRACE => write!(f, "{{"),
            TokenEnum::RBRACE => write!(f, "}}"),
            TokenEnum::FUNCTION => write!(f, "fn"),
            TokenEnum::LET => write!(f, "let"),
            TokenEnum::TRUE => write!(f, "true"),
            TokenEnum::FALSE => write!(f, "false"),
            TokenEnum::IF => write!(f, "if"),
            TokenEnum::ELSE => write!(f, "else"),
            TokenEnum::RETURN => write!(f, "return"),
        }
    }
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
