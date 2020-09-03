pub mod ast_printer;
pub mod expr;
pub mod parser;
pub mod scanner;
pub mod token;

use crate::parser::ParseError;
use core::cell::RefCell;
use parser::Parser;
use scanner::Scanner;
use std::fs::File;
use std::io;
use std::io::prelude::{Read, Write};
use std::process;

pub struct Runner {
    pub had_error: RefCell<bool>,
}

trait Throw<E> {
    fn throw(&self, callback: impl Fn(&E) -> String);
}

impl<V, E> Throw<E> for Result<V, E> {
    fn throw(&self, callback: impl Fn(&E) -> String) {
        panic!("Error Type: {}", callback(self.as_ref().err().unwrap()));
    }
}

impl<'a> Runner {
    pub fn new() -> Self {
        Runner {
            had_error: RefCell::new(false),
        }
    }

    fn run(&self, source: String) -> Result<(), ParseError> {
        let mut scanner = Scanner::new(source, self);
        let tokens = scanner.scan_tokens();

        for tok in tokens {
            println!("{}", tok.to_string());
        }

        let parser = Parser::new(tokens, self);
        let expr: String = parser.parse()?.print();
        if *self.had_error.borrow() {
            return Err(ParseError::PARSE_ERROR1);
        }
        println!("{}", expr);
        Ok(())
    }

    pub fn run_file(&self, file_path: &String) -> io::Result<()> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        self.run(contents)
            .throw(|err| format!("Err: {}", err.to_string()));

        if *self.had_error.borrow() {
            process::exit(65);
        }

        Ok(())
    }

    pub fn run_prompt(&self) -> io::Result<()> {
        loop {
            print!("|> ");
            io::stdout().flush().unwrap();

            let mut line = String::new();
            io::stdin().read_line(&mut line)?;
            self.run(line).expect("Error in running the line");
            *self.had_error.borrow_mut() = false;
        }
    }

    pub fn error(&self, token_local: &token::Token, message: &str) {
        match token_local.token_type {
            token::TokenType::EOF => self.report(token_local.line, " at end ", message),
            _ => self.report(
                token_local.line,
                format!(" at '{}'", token_local.lexeme.as_str()).as_str(),
                message,
            ),
        };
    }

    fn report(&self, line: usize, which: &str, message: &str) {
        println!("[line {}] Error {} : {}", line, which, message);
        *self.had_error.borrow_mut() = true;
    }
}