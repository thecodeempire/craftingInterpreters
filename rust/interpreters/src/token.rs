use std::fmt;

#[macro_export]
macro_rules! enum_str {
    ($vis:vis enum $name:ident {
        $($variant:ident $(= $val:expr)?),*,
    }) => {
        #[allow(dead_code)]
        #[derive(Copy, Clone)]
        #[allow(non_camel_case_types)]
        #[derive(Debug)]
        $vis enum $name {
            $($variant $(= $val)? ),*
        }

        impl $name {
            pub fn to_string(&self) -> &'static str {
                match self {
                    $($name::$variant => stringify!($variant)),*
                }
            }
        }
    };
}

enum_str! {
pub enum TokenType {
    // ------ SINGLE CHARACTER TOKENS ----------
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    COLON,
    SLASH,
    STAR,
    QUESTION,

    // ------ ONE OR TWO CHARACTER TOKENS --------
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // ------ LITERALS ----------
    IDENTIFIER,
    STRING,
    NUMBER,

    // ------ KEYWORDS -------
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    // -------- OTHERS ---------
    EOF,
}
}

impl PartialEq for TokenType {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub enum Literal {
    Number(f64),
    Str(String),
    Bool(bool),
    Nil,
}

impl Literal {
    pub fn to_string(&self) -> String {
        match self {
            Literal::Bool(b) => b.to_string(),
            Literal::Nil => String::from("Nil"),
            Literal::Number(num) => num.to_string(),
            Literal::Str(st) => st.clone(),
        }
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self.to_string();
        write!(f, "{}", s)
    }
}

#[allow(dead_code)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: &str, literal: Option<Literal>, line: usize) -> Self {
        Token {
            token_type,
            lexeme: String::from(lexeme),
            literal,
            line,
        }
    }

    pub fn to_string(&self) -> String {
        let literal_str = match &self.literal {
            Some(literal) => literal.to_string(),
            None => "LitNone".to_string(),
        };

        format!(
            "({} | {} | {})",
            &self.token_type.to_string(),
            &self.lexeme,
            literal_str
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn display_test() {
        assert_eq!(TokenType::NUMBER.to_string(), "NUMBER");

        assert_eq!(Literal::Number(4.0).to_string(), "4");
        assert_eq!(Literal::Bool(true).to_string(), "true");
        assert_eq!(Literal::Str("Hello".to_string()).to_string(), "Hello");
        assert_eq!(Literal::Nil.to_string(), "Nil");
    }
}
