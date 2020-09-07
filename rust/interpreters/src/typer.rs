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
