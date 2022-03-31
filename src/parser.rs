use crate::ast::*;
use crate::lexer::*;
use crate::token::*;

pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
}
impl Parser {
    pub fn new(input: String) -> Parser {
        let mut my_parser = Parser {
            lexer: Lexer::new(input),
            cur_token: Token::new(TokenType::ILLEGAL, "".to_string()),
            peek_token: Token::new(TokenType::ILLEGAL, "".to_string()),
        };
        my_parser.next_token();
        my_parser.next_token();
        my_parser
    }

    /// Read the next token et places it in the peek_token
    pub fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.nextToken();
    }

    pub fn parseprogramm(&mut self) -> Program {
        let mut myp: Program = Program::new();

        while self.cur_token.tokentype != TokenType::EOF {
            if let Some(stmt) = self.parse_statement() {
                myp.statements.push(stmt);
            }
            self.next_token();
        }

        myp
    }

    pub fn parse_statement(&mut self) -> Option<Statement> {
        match self.cur_token.tokentype {
            TokenType::LET => return self.parse_let_statement(),
            _ => return None,
        }

        return None;
    }

    // Parse a let statement (let x = 3; for instance)
    pub fn parse_let_statement(&mut self) -> Option<Statement> {
        let let_statement: LetStatement = LetStatement::new(
            self.cur_token.clone(),
            self.cur_token.literal.clone(),
            Expression::new(),
        );

        if self.expect_next_token(&TokenType::IDENT("".to_string())) == false {
            return None;
        }

        if self.expect_next_token(&TokenType::ASSIGN) == false {
            return None;
        }

        while self.cur_token_is(&TokenType::SEMICOLON) == false {
            self.next_token();
        }

        return Some(let_statement);
    }

    // return wether the current token if of the type passed in parameter
    pub fn cur_token_is(&self, t: &TokenType) -> bool {
        return self.cur_token.tokentype.eq(t);
    }

    // Check the next expected token (peeked token) for a certain type of token
    pub fn expect_next_token(&mut self, t: &TokenType) -> bool {
        if self.peek_token.tokentype.eq(t) {
            self.next_token();
            true
        } else {
            false
        }
    }
}
