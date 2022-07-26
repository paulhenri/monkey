use Monkey::*;
pub use Monkey::token::*;
#[cfg(test)]

use Monkey::Parser;

#[test]
fn test_let_statement() {
    let input : String = " 
            let x = 5;
            let y = 10;
            let z = 8383333;
        ".to_string();
    let mut parser = Parser::new(input);
   let program : Program = parser.parseprogramm();

   assert_eq!(3, program.len());

   let  input_array : [(TokenType, String); 3] = [(TokenType::LET, "x".to_string()), (TokenType::LET, "y".to_string()), (TokenType::LET, "foobar".to_string())];
   for i in 0..program.len()-1 { 
       let (_toktype, name) = &input_array[i];
       assert!(assert_let_statement(program[i].clone(), name));
   }

}

fn assert_let_statement(test_stmt: Stmt, name: &str) -> bool {

    if let Stmt::LET(var_name, _expr) = test_stmt {
    //First, we check that our statement has a LET Token
    var_name.eq(&Ident(name.to_string()))
    }else {
      false 
    }

}
#[test]
fn test_return_statement() {
    let input : String = "
        return 5;
        return 10;
        return 995;
        ".to_string();
    let mut parser = Parser::new(input);
    let program : Program = parser.parseprogramm();
    assert_eq!(3, program.len());
    for i in 0..program.len()-1 {
       match program[i]  {
            Stmt::RETURN(_) => {},
            _ => panic!("This is not a return statement !"),
       } 

    }

}

#[test]
fn test_identifier_expression(){
    let input = "foobar;".to_string();
    let mut parser = Parser::new(input);
    let program = parser.parseprogramm();
    assert_eq!(1, program.len());

    let is_identifier = match &program[0] {
        Stmt::EXPRESSION(Expr::IDENTIFIER(Ident(_x))) => true,
        _ => false
    };
    assert!(is_identifier); 
}


#[test]
fn test_integer_literal_exp(){
     let input = "5;".to_string();
    let mut parser = Parser::new(input);
    let program = parser.parseprogramm();
    assert_eq!(1, program.len());

     let is_integer_lit = match &program[0] {
        Stmt::EXPRESSION(Expr::INTEGER(_x)) => true,
        _ => panic!("Expression is not ok... {:?}", program[0])
            
    };
    assert!(is_integer_lit); 
}

#[test]
fn test_prefix_exp(){
        let input = "!5; 
                     -18;".to_string();
        let mut parser = Parser::new(input);
        let program = parser.parseprogramm();
        assert_eq!(2, program.len());
        let is_bang_exp = match &program[0]{
            Stmt::EXPRESSION(Expr::BANG(boxed_exp)) => {
                if let Expr::INTEGER(_x) = **boxed_exp {
                    true
                }else {
                    panic!("Expression is not a bang / integer")
                }
            }
            _ => panic!("Expression is not a bang exp...")
        };
        assert!(is_bang_exp);
}

#[test]
fn test_operator_precedence(){
    type ProgramInput = (String, String); 
    let mut programs_inputs : Vec<ProgramInput> = Vec::new();
    programs_inputs.push(("-a * b".to_string(), "((-a) * b)".to_string()));
    programs_inputs.push(("!-a".to_string(), "(!(-a))".to_string()));
    programs_inputs.push(("a+b+c".to_string(), "((a + b) + c)".to_string()));
    programs_inputs.push(("a+b-c".to_string(), "((a + b) - c)".to_string()));
    programs_inputs.push(("a*b*c".to_string(), "((a * b) * c)".to_string()));
    programs_inputs.push(("a*b/c".to_string(), "((a * b) / c)".to_string()));



    let inputs_iter = programs_inputs.iter();
    for (given, expected) in inputs_iter {
        let mut parser = Parser::new(given.clone());
        let program = parser.parseprogramm();
        assert_eq!(*expected, program[0].to_string());
    }

}


