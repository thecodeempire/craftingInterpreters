use crate::enum_str;
use crate::expr::Expr;
use crate::token::{
    Literal::*,
    Token,
    TokenType::{self, *},
};
use crate::Runner;
use core::cell::RefCell;

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: RefCell<usize>,
    runner: &'a Runner,
}

enum_str! {
    pub enum ParseError {
        PARSE_ERROR1,
        PARSE_ERROR2,
    }
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>, runner: &'a Runner) -> Self {
        Self {
            tokens,
            current: RefCell::new(0),
            runner,
        }
    }

    pub fn parse(&self) -> Result<Expr, ParseError> {
        self.expression()
    }

    fn expression(&self) -> Result<Expr, ParseError> {
        self.comma()
    }

    fn comma(&self) -> Result<Expr, ParseError> {
        let mut expr = self.ternary()?;

        while self.matching(&[COMMA]) {
            let operator = self.previous();
            let right = self.ternary()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn ternary(&self) -> Result<Expr, ParseError> {
        let mut expr = self.equality()?;

        if self.matching(&[QUESTION]) {
            let first = self.ternary()?;
            if self.matching(&[COLON]) {
                let second = self.ternary()?;
                expr = Expr::Ternary(Box::new(expr), Box::new(first), Box::new(second));
            }
        }

        Ok(expr)
    }

    fn equality(&self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;

        while self.matching(&[EQUAL_EQUAL, BANG_EQUAL]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn comparison(&self) -> Result<Expr, ParseError> {
        let mut expr = self.addition()?;

        while self.matching(&[GREATER, GREATER_EQUAL, LESS, LESS_EQUAL]) {
            let operator = self.previous();
            let right = self.addition()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn addition(&self) -> Result<Expr, ParseError> {
        let mut expr = self.multiplication()?;

        while self.matching(&[MINUS, PLUS]) {
            let operator = self.previous();
            let right = self.multiplication()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn multiplication(&self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;

        while self.matching(&[SLASH, STAR]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn unary(&self) -> Result<Expr, ParseError> {
        if self.matching(&[BANG, MINUS]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Expr::Unary(operator, Box::new(right)));
        }
        self.primary()
    }

    fn primary(&self) -> Result<Expr, ParseError> {
        if self.matching(&[FALSE]) {
            return Ok(Expr::LiteralExpr(Some(&Bool(false))));
        }
        if self.matching(&[TRUE]) {
            return Ok(Expr::LiteralExpr(Some(&Bool(true))));
        }
        if self.matching(&[NIL]) {
            return Ok(Expr::LiteralExpr(Some(&Nil)));
        }
        if self.matching(&[NUMBER, STRING]) {
            let lit = self.previous().literal.as_ref();
            return Ok(Expr::LiteralExpr(lit));
        }
        if self.matching(&[LEFT_PAREN]) {
            let expr = self.expression().unwrap();
            self.consume(&RIGHT_PAREN, "Expect ) after expression").ok();
            return Ok(Expr::Grouping(Box::new(expr)));
        }

        Err(ParseError::PARSE_ERROR1)
    }

    #[allow(dead_code)]
    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            match self.previous().token_type {
                TokenType::SEMICOLON => return,
                _ => {}
            };

            match self.peek().token_type {
                CLASS | FUN | VAR | FOR | IF | WHILE | PRINT | RETURN => {
                    return;
                }
                _ => {
                    self.advance();
                }
            }
        }
    }

    #[allow(dead_code)]
    fn error(&self, token: &Token, message: &str) -> ParseError {
        self.runner.error(token, message);
        ParseError::PARSE_ERROR1
    }

    #[allow(dead_code)]
    fn consume(&self, token_type: &TokenType, message: &str) -> Result<Option<Token>, ParseError> {
        if self.check(token_type) {
            return Ok(self.advance());
        }
        Err(self.error(self.peek(), message))
    }

    fn matching(&self, types: &[TokenType]) -> bool {
        for token_type in types.iter() {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn advance(&self) -> Option<Token> {
        if !self.is_at_end() {
            let borrowed = self.current.borrow().clone();
            *self.current.borrow_mut() = borrowed + 1;
        }
        None
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        token_type == &self.peek().token_type
    }

    fn peek(&self) -> &Token {
        &self.tokens.get(self.current.borrow().clone()).unwrap()
    }

    fn previous(&self) -> &Token {
        self.tokens.get(self.current.borrow().clone() - 1).unwrap()
    }

    fn is_at_end(&self) -> bool {
        match self.peek().token_type {
            EOF => true,
            _ => false,
        }
    }
}
