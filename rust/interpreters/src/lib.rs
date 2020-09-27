pub mod ast_printer;
mod environment;
pub mod error;
pub mod expr;
pub mod interpreter;
pub mod parser;
pub mod scanner;
mod stmt;
pub mod token;
mod typer;

use crate::error::Error;
use core::cell::RefCell;
use interpreter::Interpreter;
use parser::Parser;
use scanner::Scanner;
use std::fs::File;
use std::io;
use std::io::prelude::{Read, Write};
use std::process;

trait Throw<E> {
    fn throw(&self, callback: impl Fn(&E) -> String);
}

impl<V, E> Throw<E> for Result<V, E> {
    fn throw(&self, callback: impl Fn(&E) -> String) {
        panic!("Error Type: {}", callback(self.as_ref().err().unwrap()));
    }
}

pub struct Runner {
    pub had_error: RefCell<bool>,
    pub had_runtime_error: RefCell<bool>,
}

impl Runner {
    pub fn new() -> Self {
        Runner {
            had_error: RefCell::new(false),
            had_runtime_error: RefCell::new(false),
        }
    }

    pub fn run(&self, source: String) -> Result<(), Error> {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens()?;

        for tok in &tokens {
            println!("{}", tok.to_string());
        }

        let parser = Parser::new(tokens);
        let expr = parser.parse()?;

        Interpreter::new()
            .interpret(expr)
            .unwrap_or_else(|_err| String::from("Error"));

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
        if *self.had_runtime_error.borrow() {
            process::exit(70);
        }

        Ok(())
    }

    pub fn run_prompt(&self) -> io::Result<()> {
        loop {
            print!("|> ");
            io::stdout().flush().unwrap();

            let mut line = String::new();
            io::stdin().read_line(&mut line)?;
            self.run(line)
                .throw(|err| format!("Err: {}", err.to_string()));
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

    pub fn runtime_error(&self, error: &Error) {
        *self.had_runtime_error.borrow_mut() = true;
        eprintln!("{}", error.to_string());
    }

    fn report(&self, line: usize, which: &str, message: &str) {
        eprintln!("[line {}] Error {} : {}", line, which, message);
        *self.had_error.borrow_mut() = true;
    }
}
