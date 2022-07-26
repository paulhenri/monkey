use crate::lexer::*;
use crate::token::*;
use std::io::Error;
use std::io::{Read, Write};

pub struct REPL {
    __stdin: std::io::Stdin,
}

impl REPL {
    pub fn new() -> REPL {
        REPL {
            __stdin: std::io::stdin(),
        }
    }
    pub fn run(&mut self) -> Result<(), Error> {
        self.greeting();
        let mut buffer = String::new();

        self.__stdin.read_line(&mut buffer)?;
        while buffer != ":exit".to_string() {
            let mut mylexer: Lexer = Lexer::new(buffer.clone());
            let mut my_token: Token = Token::new(TokenType::EOF, "".to_string());
            my_token = mylexer.next_token();
            while my_token.tokentype != TokenType::EOF {
                println!("{:?}", my_token);
                my_token = mylexer.next_token();
            }
            println!("{:?}", TokenType::EOF);
            self.print_line_from_repl(">>".to_string());
            self.__stdin.read_line(&mut buffer)?;
        }

        return Ok(());
    }

    pub fn greeting(&self) {
        println!(">> Welcome to MonkeyRust REPL ! ");
        println!(">> You can type instruction and see if the world burns...");
    }

    pub fn print_line_from_repl(&self, line_to_print: String) {
        println!("{}", line_to_print);
    }
}
