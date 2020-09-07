use crate::token::Token;
use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum Error<'a> {
    #[snafu(display("Syntax Error in line: {}. Error: {}", token.to_string(), message))]
    CompileTimeError { token: &'a Token, message: String },

    #[snafu(display(
        "Error evaluating in line: {}, place: {}. Error: {}",
        token.line,
        token.lexeme,
        message
    ))]
    RuntimeError { token: &'a Token, message: String },
}

impl<'a> Error<'a> {}
