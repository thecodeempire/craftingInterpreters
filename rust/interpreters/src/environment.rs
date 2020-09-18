use crate::error::Error;
use crate::token::Token;
use crate::typer::Typer;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Environment {
    values: HashMap<String, Typer>,
    enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new_empty_env() -> Self {
        Self {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn new(enclosing: Self) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: Some(Box::new(enclosing)),
        }
    }

    pub fn get(&self, name: &Token) -> Result<&Typer, Error> {
        match self.values.get(&name.lexeme) {
            Some(value) => return Ok(value),
            None => match &self.enclosing {
                Some(enclosing) => return enclosing.get(name),
                None => Err(Error::RuntimeError {
                    token: Some(name.clone()),
                    message: format!("Undefined variable '{}'.", name.lexeme),
                }),
            },
        }
    }

    pub fn define(&mut self, name: String, value: Option<Typer>) {
        if let Some(value) = value {
            self.values.insert(name, value);
        }
    }

    pub fn assign(&mut self, name: &Token, value: Typer) -> Result<(), Error> {
        match self.values.get(&name.lexeme) {
            Some(prev_value) => {
                self.values.insert(name.lexeme, value);
            }
            None => {
                return match &mut self.enclosing {
                    Some(enclosing) => enclosing.assign(name, value),
                    None => Err(Error::RuntimeError {
                        token: Some(name.clone()),
                        message: format!("Undefined variable: {}", name.lexeme),
                    }),
                }
            }
        };

        Ok(())
    }
}
