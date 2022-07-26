use core::fmt;

use crate::TokenType;

pub type Program = Vec<Stmt>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Stmt {
    LET(Ident, Expr),
    RETURN(Expr),
    EXPRESSION(Expr) 
}
impl fmt::Display for Stmt {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
      match self {
          Stmt::LET(var_name, expr) => write!(f, "LET {} = {};", var_name, expr),
          Stmt::RETURN(expr) => write!(f, "RETURN {};", expr),
          Stmt::EXPRESSION(expr) => write!(f, "{}", expr)
      }
  }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        match self{
            Expr::IDENTIFIER(ident) => write!(f, "{}", ident),
            Expr::INTEGER(integ) => write!(f, "{}", integ),
            Expr::BANG(expr) => write!(f, "(!{})", expr),
            Expr::MINUS(expr) => write!(f, "(-{})", expr),
            Expr::INFIX(inf_expr, operator, post_expr) => write!(f, "({} {} {})", *inf_expr, operator ,*post_expr),    
        }
    }
}


#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expr{
    IDENTIFIER(Ident),
    INTEGER(usize),
    BANG(Box<Expr>),
    MINUS(Box<Expr>),
    INFIX(Box<Expr>, Infix,Box<Expr>)

}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Ident(pub String);

impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
         let Ident(value) = self;
            write!(f, "{}", value) 
    }
}


#[derive(Debug, PartialEq, PartialOrd, Eq, Clone)]
pub enum Precedence{
    LOWEST,
    EQUALS,
    LESSGREATER,
    SUM,
    PRODUCT,
    PREFIX,
    CALL
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone)]
pub enum Infix{
    PLUS,
    MINUS,
    MULTIPLY,
    DIVIDE,
    GT,
    LT,
    EQUAL,
    NOTEQUAL,
    ILLEGAL
}

impl From<TokenType> for Infix{
    fn from(tok: TokenType) -> Self {
        match tok {
            TokenType::ASTERISK => Infix::MULTIPLY,
            TokenType::MINUS => Infix::MINUS,
            TokenType::PLUS => Infix::PLUS,
            TokenType::SLASH => Infix::DIVIDE,
            TokenType::GT => Infix::GT,
            TokenType::LT => Infix::LT,
            TokenType::EQUAL => Infix::EQUAL,
            TokenType::NOTEQUAL => Infix::NOTEQUAL,
            _ => Infix::ILLEGAL
    }
}
}

impl fmt::Display for Infix{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        match self {
            Infix::PLUS => write!(f, "+"),
            Infix::MINUS => write!(f, "-"),
            Infix::MULTIPLY => write!(f, "*"),
            Infix::DIVIDE => write!(f, "/"),
            Infix::GT => write!(f, ">"),
            Infix::LT => write!(f, "<"),
            Infix::EQUAL => write!(f, "=="),
            Infix::NOTEQUAL => write!(f, "!="),
            Infix::ILLEGAL => write!(f, "$$ILLEGALCARACT$$")
        }
    }
}
