use std::collections::HashMap;

use crate::ast::*;
use crate::lexer::*;
use crate::token::*;
use std::mem::*;

pub type PrefixParseFn = fn(Token) -> Option<Expr>;
pub type InfixParseFn = fn(Expr) -> Option<Expr>;

pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
    errors: Vec<String>,
    prefix_parsers: HashMap<Discriminant<TokenType>, PrefixParseFn>,
    infix_parsers: HashMap<Discriminant<TokenType>, InfixParseFn>,
    precedences: HashMap<Discriminant<TokenType>, Precedence>,
}
impl Parser {
    pub fn new(input: String) -> Parser {
        let mut my_parser = Parser {
            lexer: Lexer::new(input),
            cur_token: Token::new(TokenType::ILLEGAL, "".to_string()),
            peek_token: Token::new(TokenType::ILLEGAL, "".to_string()),
            errors: Vec::new(),
            prefix_parsers: HashMap::new(),
            infix_parsers: HashMap::new(),
            precedences: HashMap::new(),
        };
        my_parser.add_prefix_parser(
            discriminant(&TokenType::IDENT('x'.to_string())),
            parse_identifier,
        );
        my_parser.add_prefix_parser(discriminant(&TokenType::INT(5)), parse_integer);
        my_parser.add_prefix_parser(discriminant(&TokenType::TRUE), parse_boolean);
        my_parser.add_prefix_parser(discriminant(&TokenType::FALSE), parse_boolean);

        my_parser
            .precedences
            .insert(discriminant(&TokenType::EQUAL), Precedence::EQUALS);
        my_parser
            .precedences
            .insert(discriminant(&TokenType::NOTEQUAL), Precedence::EQUALS);

        my_parser
            .precedences
            .insert(discriminant(&TokenType::LT), Precedence::LESSGREATER);
        my_parser
            .precedences
            .insert(discriminant(&TokenType::GT), Precedence::LESSGREATER);

        my_parser
            .precedences
            .insert(discriminant(&TokenType::PLUS), Precedence::SUM);
        my_parser
            .precedences
            .insert(discriminant(&TokenType::MINUS), Precedence::SUM);

        my_parser
            .precedences
            .insert(discriminant(&TokenType::ASTERISK), Precedence::PRODUCT);
        my_parser
            .precedences
            .insert(discriminant(&TokenType::SLASH), Precedence::PRODUCT);

        my_parser
            .precedences
            .insert(discriminant(&TokenType::LPAREN), Precedence::CALL);

        my_parser.next_token();
        my_parser.next_token();
        my_parser
    }

    // TODO: Refactor this piece of code to have only one function that access the precedences
    // hashMap
    pub fn cur_precedence(&mut self) -> Precedence {
        let token: Token = self.cur_token.clone();
        if let Some(prec) = self.precedences.get(&discriminant(&token.tokentype)) {
            prec.clone()
        } else {
            Precedence::LOWEST
        }
    }

    pub fn peek_precedence(&mut self) -> Precedence {
        let token: Token = self.peek_token.clone();
        if let Some(prec) = self.precedences.get(&discriminant(&token.tokentype)) {
            prec.clone()
        } else {
            Precedence::LOWEST
        }
    }

    //Pratt parser functions
    pub fn add_prefix_parser(&mut self, token_type: Discriminant<TokenType>, func: PrefixParseFn) {
        self.prefix_parsers.insert(token_type, func);
    }

    pub fn add_infix_parser(&mut self, token_type: Discriminant<TokenType>, func: InfixParseFn) {
        self.infix_parsers.insert(token_type, func);
    }

    /// Check wether errors are stored in the parser
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn add_error(&mut self, error: String) {
        self.errors.push(error);
    }

    pub fn get_errors(&self) -> Vec<String> {
        self.errors.clone()
    }

    /// Read the next token et places it in the peek_token
    pub fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parseprogramm(&mut self) -> Program {
        let mut myp: Program = Program::new();

        while self.cur_token.tokentype != TokenType::EOF {
            if let Some(stmt) = self.parse_statement() {
                myp.push(stmt);
            } else {
                println!("No stmt returned...");
            }
            self.next_token();
        }

        myp
    }

    pub fn parse_statement(&mut self) -> Option<Stmt> {
        match self.cur_token.tokentype {
            TokenType::LET => self.parse_let_statement(),
            TokenType::RETURN => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    pub fn parse_grouped_expression(&mut self) -> Option<Expr> {
        self.next_token();
        let exp = self.parse_expression(Precedence::LOWEST);
        if !self.expect_next_token(&TokenType::RPAREN) {
            None
        } else {
            exp
        }
    }

    pub fn parse_expression_statement(&mut self) -> Option<Stmt> {
        let expr_stmt = self.parse_expression(Precedence::LOWEST);

        if self.cur_token_is(&TokenType::SEMICOLON) {
            self.next_token();
        }

        expr_stmt.map(|e| Stmt::EXPRESSION(e))
    }

    pub fn parse_prefix_expression(&mut self, token: TokenType) -> Option<Expr> {
        self.next_token();
        if let Some(right_exp) = self.parse_expression(Precedence::PREFIX) {
            match token {
                TokenType::BANG => Some(Expr::BANG(Box::new(right_exp))),
                TokenType::MINUS => Some(Expr::MINUS(Box::new(right_exp))),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn parse_block_statement(&mut self) -> Option<BlockStatement> {
        let mut stmts = Vec::new();
        self.next_token();
        while !self.cur_token_is(&TokenType::RBRACE) && !self.cur_token_is(&TokenType::EOF) {
            if let Some(statement) = self.parse_statement() {
                stmts.push(statement);
            }
            self.next_token();
        }
        if stmts.len() > 0 {
            Some(BlockStatement(stmts))
        } else {
            None
        }
    }

    pub fn parse_if_expression(&mut self) -> Option<Expr> {
        println!("IF.1 - Entering IF parsing function");
        if !self.expect_next_token(&TokenType::LPAREN) {
            println!("IF.2 - Exiting IF parsing, no LPAREN token found");
            return None;
        }
        self.next_token();
        let condition = self.parse_expression(Precedence::LOWEST);
        match condition {
            Some(condition) => {
                println!("IF.3 - Condition has been parsed.");
                if !self.expect_next_token(&TokenType::RPAREN) {
                    return None;
                } else {
                    if !self.expect_next_token(&TokenType::LBRACE) {
                        return None;
                    } else {
                        let conseq = self.parse_block_statement();
                        match conseq {
                            Some(conseq) => {
                                println!("Returning IF Expression");
                                return Some(Expr::IF(
                                    Box::new(condition),
                                    conseq.clone(),
                                    conseq.clone(),
                                ));
                            }
                            None => return None,
                        }
                    }
                }
            }
            None => return None,
        }
    }

    pub fn is_infixable(&self, tok: &TokenType) -> bool {
        matches!(
            tok,
            TokenType::EQUAL
                | TokenType::NOTEQUAL
                | TokenType::GT
                | TokenType::LT
                | TokenType::PLUS
                | TokenType::MINUS
                | TokenType::SLASH
                | TokenType::ASTERISK
                | TokenType::LPAREN
        )
    }

    pub fn parse_expression(&mut self, precedence: Precedence) -> Option<Expr> {
        let mut left_expr = match self.cur_token.tokentype.clone() {
            TokenType::BANG | TokenType::MINUS => {
                println!("157: Returning a BANG OR MINUS Expression as left member");
                self.parse_prefix_expression(self.cur_token.tokentype.clone())
            }
            TokenType::LPAREN => self.parse_grouped_expression(),
            TokenType::IF => {
                println!("Parsing IF expression");
                self.parse_if_expression()
            }
            TokenType::FUNCTION => self.parse_function_literal(),
            _ => {
                println!(
                    "Parsing token for left_expr    {:?}",
                    &self.cur_token.clone().tokentype
                );
                let prefix = self
                    .prefix_parsers
                    .get(&discriminant(&self.cur_token.tokentype));
                match prefix {
                    Some(prefix_func) => prefix_func(self.cur_token.clone()),
                    None => {
                        println!(
                            "No parsing function found for {:?}",
                            self.cur_token.clone().tokentype
                        );
                        None
                    }
                }
            }
        };

        if left_expr == None {
            println!("No left expression, returning none...");
            None
        } else {
            println!("9. Left expression found, looking for something more");
            while !self.expect_next_token(&TokenType::SEMICOLON)
                && precedence < self.peek_precedence()
            {
                let peek_token = &self.peek_token.tokentype.clone();

                // TODO: This part should be revamped to use the vector of infix functions
                if self.is_infixable(peek_token) {
                    if self.peek_token_is(&TokenType::LPAREN){
                        self.next_token();
                       left_expr = self.parse_call_expression(left_expr.unwrap());
                    }else{
                    println!("10. Infixable Peek_token == {:?}", peek_token);
                    self.next_token();
                    left_expr =
                        self.parse_infix_expression(self.cur_token.clone(), left_expr.unwrap());
                    }
                } else {
                    println!("11. Token is not Infixable, returning left_expr as is");
                    return left_expr;
                }
            }
            println!("12. Returning lext_expr as is. {:?}", left_expr);
            left_expr
        }
    }

    pub fn parse_call_expression(&mut self,func_call: Expr) -> Option<Expr>{
        let call_expr = Expr::CALL(Box::new(func_call), self.parse_function_params());
        Some(call_expr)
    }



    pub fn parse_infix_expression(&mut self, tok: Token, left_expression: Expr) -> Option<Expr> {
        let precedence = self.cur_precedence();
        self.next_token();
        println!(
            "13. Parsing right expression, left is {:?}",
            left_expression
        );
        let right_expr = self.parse_expression(precedence).unwrap();
        Some(Expr::INFIX(
            Box::new(left_expression),
            tok.tokentype.into(),
            Box::new(right_expr),
        ))
    }

    pub fn parse_return_statement(&mut self) -> Option<Stmt> {
        let expr = self.parse_expression(Precedence::LOWEST).unwrap();
        if !self.peek_token_is(&TokenType::SEMICOLON){
           self.next_token();
           }
        Some(Stmt::RETURN(expr))
    }

    // Parse a let statement (let x = 3; for instance)
    pub fn parse_let_statement(&mut self) -> Option<Stmt> {
        if !self.expect_next_token(&TokenType::IDENT("x".to_string())) {
            return None;
        }
        let var_name: String = self.cur_token.literal.clone();
        if !self.expect_next_token(&TokenType::ASSIGN) {
            return None;
        }

        self.next_token();
        let result =    self.parse_expression(Precedence::LOWEST)
            .map(|n| Stmt::LET(Ident(var_name), n));
        if self.peek_token_is(&TokenType::SEMICOLON){
            self.next_token();
        }
        result
    }

    // return wether the current token if of the type passed in parameter
    pub fn cur_token_is(&self, t: &TokenType) -> bool {
        std::mem::discriminant(&self.cur_token.tokentype) == std::mem::discriminant(t)
    }

    pub fn peek_token_is(&self, t: &TokenType) -> bool {
        std::mem::discriminant(&self.peek_token.tokentype) == std::mem::discriminant(t)
    }

    pub fn parse_function_literal(&mut self) -> Option<Expr> {
        if !self.expect_next_token(&TokenType::LPAREN) {
            None
        } else {
            let param_list = self.parse_function_params();
            if !self.expect_next_token(&TokenType::LBRACE) {
                println!(
                    "2.1 - No LBRACE expected, instead found : {:?}",
                    &self.peek_token.tokentype
                );
                None
            } else {
                let blck_stmt = self.parse_block_statement();
                if let Some(block) = blck_stmt {
                    Some(Expr::FUNC(param_list, block))
                } else {
                    None
                }
            }
        }
    }

    pub fn parse_function_params(&mut self) -> Parameters {
        // <Identifier> <CommaToken> <Identifier>
        println!("1.2 - Entering parameter parsing");
        let mut param_list: Vec<Expr> = Vec::new();
        if !self.expect_next_token(&TokenType::RPAREN) {
            self.next_token();

            let myparam = self.parse_expression(Precedence::LOWEST);
            if let Some(param) = myparam {
                println!("1.1 - Found function args");
                param_list.push(param);
            } else {
                return Parameters(param_list);
            }

            // Careful here, we don't want to advance the token but we just want to peek at it.
            while self.peek_token_is(&TokenType::COMMA) {
                self.next_token();
                self.next_token();
                let myparam = self.parse_expression(Precedence::LOWEST);
                if let Some(param) = myparam {
                    println!("1.1 - Found function args");
                    param_list.push(param);
                } else {
                    self.add_error(format!(
                        "Could not parse arg in function / {:?}",
                        &self.cur_token
                    ));
                    panic!()
                }
            }
            if self.expect_next_token(&TokenType::RPAREN) {
                println!("1.1 Exiting params parsing");
                Parameters(param_list)
            } else {
                Parameters(Vec::new())
            }
        } else {
            Parameters(param_list)
        }
    }
    // Check the next expected token (peeked token) for a certain type of token
    // Keep in mind that the function advance the tolken pointer and calls self.next_token()
    pub fn expect_next_token(&mut self, t: &TokenType) -> bool {
        if std::mem::discriminant(t) == std::mem::discriminant(&self.peek_token.tokentype) {
            self.next_token();
            true
        } else {
            let error = format!(
                "Expected token {:?} was not found - Found {:?} instead",
                t.clone(),
                &self.peek_token.clone()
            );
            self.add_error(error);
            false
        }
    }
}

pub fn parse_identifier(token: Token) -> Option<Expr> {
    println!("Return an Identifier Expression");
    Some(Expr::IDENTIFIER(Ident(token.literal)))
}

pub fn parse_integer(token: Token) -> Option<Expr> {
    if let TokenType::INT(int_value) = token.tokentype {
        Some(Expr::INTEGER(int_value))
    } else {
        None
    }
}

pub fn parse_boolean(token: Token) -> Option<Expr> {
    match token.tokentype {
        TokenType::TRUE => Some(Expr::BOOLEAN(true)),
        TokenType::FALSE => Some(Expr::BOOLEAN(false)),
        _ => None,
    }
}
