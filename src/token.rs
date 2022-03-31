#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenType {
    ILLEGAL,
    EOF,
    ASSIGN,
    IDENT(String),
    INT(usize),
    PLUS,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    FUNCTION,
    LET,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    RETURN,
    TRUE,
    FALSE,
    IF,
    ELSE,
    LT,
    GT,
    EQUAL,
    NOTEQUAL,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    pub tokentype: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(tktype: TokenType, literal: String) -> Token {
        Token {
            tokentype: tktype,
            literal: literal,
        }
    }
}
