pub use Monkey::token::*;
#[cfg(test)]
use Monkey::Lexer;

#[test]
/// Check that the read_char function of the lexer works properly
fn read_char_works() {
    let mut my_lexer = Lexer::new("=+(){},;".to_string());
    let expected_results = vec![
        TokenType::ASSIGN,
        TokenType::PLUS,
        TokenType::LPAREN,
        TokenType::RPAREN,
        TokenType::LBRACE,
        TokenType::RBRACE,
        TokenType::COMMA,
        TokenType::SEMICOLON,
        TokenType::EOF,
    ];

    for tok in expected_results.iter() {
        let mytoken = my_lexer.nextToken();
        assert_eq!(mytoken.tokentype, *tok);
    }
}

/// Check if an identifier can be read properly
#[test]
fn read_identifier_works() {
    let input: String = " let five = 5;*+-/>< if(3=4){return}else{return}==!=".to_string();
    let mut my_lexer = Lexer::new(input);
    let expected_results = vec![
        TokenType::LET,
        TokenType::IDENT("five".to_string()),
        TokenType::ASSIGN,
        TokenType::INT(5),
        TokenType::SEMICOLON,
        TokenType::ASTERISK,
        TokenType::PLUS,
        TokenType::MINUS,
        TokenType::SLASH,
        TokenType::GT,
        TokenType::LT,
        TokenType::IF,
        TokenType::LPAREN,
        TokenType::INT(3),
        TokenType::ASSIGN,
        TokenType::INT(4),
        TokenType::RPAREN,
        TokenType::LBRACE,
        TokenType::RETURN,
        TokenType::RBRACE,
        TokenType::ELSE,
        TokenType::LBRACE,
        TokenType::RETURN,
        TokenType::RBRACE,
        TokenType::EQUAL,
        TokenType::NOTEQUAL,
    ];

    for tok in expected_results.iter() {
        let mytoken = my_lexer.nextToken();
        assert_eq!(mytoken.tokentype, *tok);
    }
}
