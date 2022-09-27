use Monkey::*;
pub use Monkey::token::*;
#[cfg(test)]

use Monkey::Parser;

#[test]
fn test_let_statement() {
    let input : String = " 
            let x = 5;
            let y = true;
            let z = foobar;
        ".to_string();
    let mut parser = Parser::new(input);
   let program : Program = parser.parseprogramm();

   assert_eq!(3, program.len());

   let  input_array : [(TokenType, String, String); 3] = [(TokenType::LET, "x".to_string(), "5".to_string()), (TokenType::LET, "y".to_string(), "true".to_string()), (TokenType::LET, "z".to_string(), "foobar".to_string())];
   for i in 0..program.len()-1 { 
       let (_toktype, name, expr) = &input_array[i];
       assert!(assert_let_statement(program[i].clone(), name, expr));
   }

}

fn assert_let_statement(test_stmt: Stmt, name: &str, expected_expr: &str) -> bool {

    if let Stmt::LET(var_name, expr) = test_stmt {
    // First, we check that our statement has a LET Token
    // The method **seems** quite imperfect as the test only check if the to_string()
    // representation is equal. Collisions could occur and give false positives.
    // A more thorough method would be to check for Expr types.
    var_name.eq(&Ident(name.to_string())) && expr.to_string().eq(&expected_expr.to_string())
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
    for val in program.into_iter() {
       match val  {
            Stmt::RETURN(_) => {},
            _ => panic!("This is not a return statement !"),
       } 

    }

}

fn assert_identifier(test_stmt: Stmt, name: &str) -> bool {
    let cmp_stmt = Stmt::EXPRESSION(Expr::IDENTIFIER(Ident(name.to_string())));
    matches!(cmp_stmt, test_stmt)
}


fn assert_identifier_expr(test_expr: Expr, name: &str) -> bool{
    let cmp_stmt = Expr::IDENTIFIER(Ident(name.to_string()));
    matches!(cmp_stmt, test_stmt)
}

//#[test]
fn test_boolean_expression(){
    let input = "true; false; let foobar = true; let barfoo = false;".to_string();
    let mut parser = Parser::new(input);
    let program = parser.parseprogramm();
    assert_eq!(4, program.len());
}


#[test]
fn test_identifier_expression(){
    let input = "foobar;".to_string();
    let mut parser = Parser::new(input);
    let program = parser.parseprogramm();
    assert_eq!(1, program.len());

    assert!(assert_identifier(program[0].clone(), "foobar"));
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
    programs_inputs.push(("1+(2+3)+4".to_string(), "((1 + (2 + 3)) + 4)".to_string()));



    let inputs_iter = programs_inputs.iter();
    for (given, expected) in inputs_iter {
        let mut parser = Parser::new(given.clone());
        let program = parser.parseprogramm();
        assert_eq!(*expected, program[0].to_string());
    }

}

#[test]
fn test_ifelse(){
    let input = "if (x < y) { x }".to_string();
    let mut parser = Parser::new(input.clone());
    let program = parser.parseprogramm();
    assert_eq!(input, program[0].to_string());
}

#[test]
fn test_functions(){
    type ProgramInput = (String, usize, Vec<String>);
    let mut program_inputs : Vec<ProgramInput> = Vec::new();
    program_inputs.push(("fn(x, y) { x + y}".to_string(), 2, vec!["x".to_string(), "y".to_string()]));
    program_inputs.push(("fn(x) { x + y}".to_string(), 1, vec!["x".to_string()]));
    program_inputs.push(("fn() { x + y}".to_string(), 0, vec![]));


    for (input, args_nb, args_vec) in program_inputs.iter(){
        let mut parser = Parser::new(input.clone());
        let program = parser.parseprogramm();
        assert_eq!(program.len(), 1);

        if let Stmt::EXPRESSION(Expr::FUNC(Parameters(param_list), _stmt)) = program[0].clone() {
            assert_eq!(param_list.len(), *args_nb);

        //Now let's compare identifiers and strings in the vector
        for i in 0..args_vec.len() {
            if let Expr::IDENTIFIER(Ident(identifier_name)) = param_list[i].clone() {
                assert_eq!(identifier_name, args_vec[i]);
            }
        }
        }else {
            println!("Expected an Express, go {:?} instead", program[0].clone());
            panic!() 
        }




    }

}


#[test]
fn test_function_call(){
    type ProgramInput = (String, usize, Vec<String>);
    let mut program_inputs : Vec<ProgramInput> = Vec::new();
    program_inputs.push(("add(1, 2 * 5, 4 + 5)".to_string(), 3, vec!["1".to_string(), "(2 * 5)".to_string(), "(4 + 5)".to_string()]));

    // Here we should use herlpers already coded to test infix on args and identifiers for the first arg - A full automated suite 
    // can be hard to design at first glance so baby steps to ensure that a basic case is covered.
    for (input, _args_nb, test_args) in program_inputs.iter() {
        let mut parser = Parser::new(input.clone());
        let program = parser.parseprogramm();
        assert_eq!(program.len(), 1);
        if let Stmt::EXPRESSION(Expr::CALL(func_identifier, Parameters(args))) = program[0].clone(){
            assert!(assert_identifier_expr(*func_identifier, "add"));
            assert_eq!(args.len(), test_args.len());
            for i in 0..test_args.len() {
                assert_eq!(test_args[i], args[i].to_string()); 
            }
        
        }else {
             println!("Expected an Express, go {:?} instead", program[0].clone());
            panic!() 
        }
    }
}

