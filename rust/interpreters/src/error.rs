use crate::token::Token;
use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Syntax Error in line: {}. Error: {}", match token { Some(token) => token.to_string(), None => "None".to_string() }, message))]
    CompileTimeError {
        token: Option<Token>,
        message: String,
    },

    #[snafu(display(
        "Error evaluating in line: {}, place: {}. Error: {}",
        match token { Some(token) => token.line , None => 0 },
        match token { Some(token) => token.lexeme , None => String::from("") },
        message
    ))]
    RuntimeError {
        token: Option<Token>,
        message: String,
    },
}

impl Error {}
