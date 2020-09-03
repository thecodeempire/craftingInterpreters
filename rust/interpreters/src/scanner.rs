use crate::token::{Literal, Token, TokenType, TokenType::*};
use crate::Runner;
use std::collections::HashMap;

pub struct Scanner<'a> {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    pub keywords: HashMap<String, TokenType>,
    runner: &'a Runner,
}

macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert(String::from($key), $value);
            )+
            m
        }
     };
);

impl<'a> Scanner<'a> {
    pub fn new(source: String, runner: &Runner) -> Scanner {
        let keywords = map! {
            "class" => CLASS,
            "else" => ELSE,
            "false" => FALSE,
            "for" => FOR,
            "fun" => FUN,
            "if" => IF,
            "nil" => NIL,
            "or" => OR,
            "print" => PRINT,
            "return" => RETURN,
            "super" => SUPER,
            "this" => THIS,
            "true" => TRUE,
            "var" => VAR,
            "while" => WHILE,
            "and" => AND
        };

        Scanner {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 0,
            keywords,
            runner,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()
        }
        self.tokens.push(Token::new(EOF, "EOF", None, self.line));
        &self.tokens
    }

    pub fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add_token(LEFT_PAREN, None),
            ')' => self.add_token(RIGHT_PAREN, None),
            '{' => self.add_token(LEFT_BRACE, None),
            '}' => self.add_token(RIGHT_BRACE, None),
            ',' => self.add_token(COMMA, None),
            '.' => self.add_token(DOT, None),
            '-' => self.add_token(MINUS, None),
            '+' => self.add_token(PLUS, None),
            ';' => self.add_token(SEMICOLON, None),
            '*' => self.add_token(STAR, None),
            ':' => self.add_token(COLON, None),
            '?' => self.add_token(QUESTION, None),
            '!' => {
                let tok = if self.matching('=') { BANG_EQUAL } else { BANG };
                self.add_token(tok, None);
            }
            '=' => {
                let tok = if self.matching('=') {
                    EQUAL_EQUAL
                } else {
                    EQUAL
                };
                self.add_token(tok, None);
            }
            '<' => {
                let tok = if self.matching('=') { LESS_EQUAL } else { LESS };
                self.add_token(tok, None);
            }
            '>' => {
                let tok = if self.matching('=') {
                    GREATER_EQUAL
                } else {
                    GREATER
                };
                self.add_token(tok, None);
            }
            '/' => {
                if self.peek_next() == '/' {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else if self.peek_next() == '*' {
                    self.handle_multi_line_comments();
                } else {
                    self.add_token(SLASH, None);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string(),
            _ => {
                if self.is_digit(c) {
                    self.number();
                } else if self.is_alpha(c) {
                    self.identifier();
                } else {
                    self.runner.error(
                        &Token::new(
                            TokenType::NIL,
                            &c.clone().to_string().as_str(),
                            None,
                            self.line,
                        ),
                        "Unexpected character",
                    );
                }
            }
        };
    }

    fn handle_multi_line_comments(&mut self) {
        while !self.is_at_end() {
            if self.peek() == '*' && self.peek_next() == '/' {
                self.advance();
                self.advance();
                break;
            } else {
                self.advance();
            }
        }
    }

    fn is_alpha(&self, c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c == '_')
    }

    fn identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }
        let text = self.source[self.start..self.current - 1].to_string();
        let token_type = self.keywords.get(&text);

        let tt: TokenType = if token_type.is_none() {
            IDENTIFIER
        } else {
            let t = *(token_type.clone()).unwrap();
            t
        };

        self.add_token(tt, None);
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' {
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }

            self.current -= 1;
        } else {
            self.current -= 1;
        }

        if self.is_alpha(self.peek_next()) {
            self.runner.error(
                &Token::new(
                    TokenType::NIL,
                    &self.peek_next().clone().to_string(),
                    None,
                    self.line,
                ),
                "Unexpected character check your number!",
            );
        }

        // this is to normalize the advanced current value
        let num = match self.source[self.start..self.current]
            .trim()
            .parse::<f64>()
            .ok()
        {
            None => 0.0,
            Some(num) => num,
        };

        self.add_token(NUMBER, Some(Literal::Number(num)))
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.get_char_at(self.current)
    }

    fn get_char_at(&self, index: usize) -> char {
        self.source.chars().nth(index).unwrap()
    }

    fn is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn string(&mut self) {
        while self.peek() != '"' && self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current - 1).unwrap()
    }

    fn matching(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.peek_next() != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let text = self.source[self.start..self.current].to_string();
        let text = text.trim();
        let line = self.line;
        self.tokens
            .push(Token::new(token_type, &text, literal, line));
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
