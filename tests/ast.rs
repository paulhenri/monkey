use Monkey::*;
pub use Monkey::token::*;
#[cfg(test)]


#[test]
fn test_to_tring() {
    let let_var = Stmt::LET(Ident("myVar".to_string()), Expr::IDENTIFIER(Ident("anotherVar".to_string()) ) );

    assert_eq!(let_var.to_string(), "LET myVar = anotherVar;");
}

