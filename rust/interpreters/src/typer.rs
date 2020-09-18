use std::fmt::{self, Display};

#[derive(Clone)]
pub enum Typer {
    Number(f64),
    Str(String),
    Boolean(bool),
    Nil,
}

impl Typer {
    pub fn to_string(&self) -> String {
        match self {
            Typer::Boolean(b) => b.to_string(),
            Typer::Nil => String::from("Nil"),
            Typer::Number(num) => num.to_string(),
            Typer::Str(st) => st.clone(),
        }
    }
}

impl Display for Typer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Typer::Boolean(b) => write!(f, "{}", b),
            Typer::Nil => write!(f, "Nil"),
            Typer::Number(num) => write!(f, "{}", num),
            Typer::Str(st) => write!(f, "{}", st),
        }
    }
}
