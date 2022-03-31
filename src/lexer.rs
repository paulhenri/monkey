#![warn(missing_docs)]
use crate::token::{Token, TokenType};
use crate::TokenBuilder;

pub struct Lexer {
    input: Vec<u8>,
    position: usize,
    readPosition: usize,
    ch: u8,
    tok_builder: TokenBuilder,
}
impl Lexer {
    /// Builder of the lexer - Returns a Lexer struct
    /// * `input` - Code that neeeds to be parsed.
    pub fn new(input: String) -> Lexer {
        let mut mylexer = Lexer {
            input: input.as_bytes().to_vec(),
            position: 0,
            readPosition: 0,
            ch: 0,
            tok_builder: TokenBuilder::default(),
        };
        mylexer.read_char();
        mylexer
    }

    /// Read the next char and advance the reading position
    pub fn read_char(&mut self) {
        if let Some(&ch) = self.input.get(self.readPosition) {
            self.ch = ch;
        } else {
            self.ch = 0;
        }
        self.position = self.readPosition;
        self.readPosition += 1;
    }

    /// Read the next char but does not advance the reading position. Usefull to recognise 2 or 3
    /// characters keywords (e.g.  ==, != )
    pub fn peek_char(&mut self) -> u8 {
        if let Some(&next_char) = self.input.get(self.readPosition) {
            return next_char;
        } else {
            return 0;
        }
    }

    /// Processes the next char(s) and return the next token
    pub fn nextToken(&mut self) -> Token {
        let mut tok_type: TokenType = TokenType::ILLEGAL;
        let mut lit: String = String::from_utf8(vec![self.ch]).unwrap();
        self.skip_whitespace();
        match (self.ch) {
            0x2b => tok_type = TokenType::PLUS,
            0x2c => tok_type = TokenType::COMMA,
            0x2d => tok_type = TokenType::MINUS,
            0x2a => tok_type = TokenType::ASTERISK,
            0x2f => tok_type = TokenType::SLASH,
            0x21 => {
                if self.peek_char() == 0x3d {
                    tok_type = TokenType::NOTEQUAL;
                    self.read_char();
                } else {
                    tok_type = TokenType::BANG;
                }
            }
            0x3d => {
                if self.peek_char() == 0x3d {
                    tok_type = TokenType::EQUAL;
                    self.read_char(); //Bien penser à avancer car il s'agit d'un token sur deux chars
                } else {
                    tok_type = TokenType::ASSIGN;
                }
            }
            0x3b => tok_type = TokenType::SEMICOLON,
            0x28 => tok_type = TokenType::LPAREN,
            0x29 => tok_type = TokenType::RPAREN,
            0x7b => tok_type = TokenType::LBRACE,
            0x7d => tok_type = TokenType::RBRACE,

            0x3c => tok_type = TokenType::LT,
            0x3e => tok_type = TokenType::GT,
            0x0 => tok_type = TokenType::EOF,
            _ => {
                if is_valid_letter(self.ch) {
                    lit = self.read_identifier();
                    tok_type = token_from_identifier(lit.clone());
                    return Token::new(tok_type, lit);
                } else if is_valid_number(self.ch) {
                    lit = self.read_number();
                    tok_type = TokenType::INT(lit.parse().unwrap());
                    return Token::new(tok_type, lit);
                } else {
                    tok_type = TokenType::ILLEGAL;
                }
            }
        }

        self.read_char();
        return Token::new(tok_type, lit);
    }

    /// Loop through the input until a non number character is found
    /// Currently only support numbers composed by chars ranging from 0 to 9
    /// That is only integer support for now
    pub fn read_number(&mut self) -> String {
        let mut my_number: Vec<u8> = vec![];
        while is_valid_number(self.ch) {
            my_number.push(self.ch);
            self.read_char();
        }
        return String::from_utf8(my_number).unwrap();
    }

    /// Read an identifier from the current position until something else than a character is found
    pub fn read_identifier(&mut self) -> String {
        let mut char_bytes: Vec<u8> = vec![];
        while is_valid_letter(self.ch) {
            char_bytes.push(self.ch);
            self.read_char();
        }
        return String::from_utf8(char_bytes).unwrap();
    }

    /// Reads chars as long as a whitespace is encountered.
    pub fn skip_whitespace(&mut self) {
        while self.ch == 0x20 {
            self.read_char();
        }
    }
}

/// Check is a given character (Given as UTF8 byte) is a valid letter for identifier / function
pub fn is_valid_letter(letter: u8) -> bool {
    return (letter >= 0x41 && letter <= 0x5a)
        || (letter >= 0x61 && letter <= 0x7a)
        || letter == 0x5f;
}

/// Checks if the current character is a valid number from the UTF-8 table.
pub fn is_valid_number(letter: u8) -> bool {
    return letter >= 0x30 && letter <= 0x39;
}

/// Create a token from a identifier given as parameter
/// Token mades from identifier are what can be considered as keywords.
pub fn token_from_identifier(ident: String) -> TokenType {
    match ident.as_str() {
        "fn" => TokenType::FUNCTION,
        "FN" => TokenType::FUNCTION,
        "let" => TokenType::LET,
        "LET" => TokenType::LET,
        "if" => TokenType::IF,
        "IF" => TokenType::IF,
        "else" => TokenType::ELSE,
        "ELSE" => TokenType::ELSE,
        "return" => TokenType::RETURN,
        "RETURN" => TokenType::RETURN,
        "TRUE" => TokenType::TRUE,
        "true" => TokenType::TRUE,
        "FALSE" => TokenType::FALSE,
        "false" => TokenType::FALSE,

        _ => TokenType::IDENT(ident.clone()),
    }
}
