use crate::token::*;

pub enum Node {
    PROGRAM(Program),
    STATEMENT(Statement),
    EXPRESSION(Expression),
}

pub enum Statement {
    LET(LetStatement),
}

pub struct Expression {}
impl Expression {
    pub fn new() -> Expression {
        Expression {}
    }
}

pub struct Program {
    pub statements: Vec<Statement>,
}
impl Program {
    pub fn new() -> Program {
        Program {
            statements: Vec::new(),
        }
    }
}

pub struct LetStatement {
    token: Token,
    name: String,
    value: Expression,
}
impl LetStatement {
    pub fn new(token: Token, name: String, value: Expression) -> LetStatement {
        LetStatement { token, name, value }
    }
}

pub struct Express {}
