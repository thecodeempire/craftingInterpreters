use crate::error::Error;
use crate::expr::Expr;
use crate::stmt::Stmt;
use crate::token::{
    Literal::*,
    Token,
    TokenType::{self, *},
};
use std::cell::RefCell;

pub struct Parser {
    tokens: Vec<Token>,
    current: RefCell<usize>,
}

type Result<T, E = Error> = std::result::Result<T, E>;

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: RefCell::new(0),
        }
    }

    pub fn parse(&self) -> Result<Vec<Stmt>> {
        let mut declarations: Vec<Stmt> = Vec::new();

        while !self.is_at_end() {
            let declaration = match self.declaration() {
                Ok(stmt) => declarations.push(stmt),
                Err(err) => return Err(err),
            };
        }
        Ok(declarations)
    }

    fn declaration(&self) -> Result<Stmt, Error> {
        if self.matching(&[VAR]) {
            return match self.var_declaration() {
                Ok(decl) => Ok(decl),
                Err(err) => {
                    self.synchronize();
                    return Err(err);
                }
            };
        }

        self.statement()
    }

    fn var_declaration(&self) -> Result<Stmt, Error> {
        let name = self.consume(&IDENTIFIER, "Expect variable name")?;
        let mut initializer: Option<Expr> = None;

        if self.matching(&[EQUAL]) {
            initializer = Some(self.expression()?);
        }

        Ok(Stmt::Var(Box::new((name).unwrap()), initializer))
    }

    fn statement(&self) -> Result<Stmt, Error> {
        if self.matching(&[PRINT]) {
            return self.print_statement();
        }
        if self.matching(&[LEFT_BRACE]) {
            return Ok(Stmt::Block(Box::new(self.block()?)));
        }
        self.expression_statement()
    }

    fn print_statement(&self) -> Result<Stmt, Error> {
        let value = self.expression()?;
        self.consume(&SEMICOLON, "Expect ';' after expression");
        Ok(Stmt::Print(Box::new(value)))
    }

    fn expression_statement(&self) -> Result<Stmt, Error> {
        let expr = self.expression()?;
        self.consume(&SEMICOLON, "Expect ';' after expression");
        Ok(Stmt::Expression(Box::new(expr)))
    }

    fn block(&self) -> Result<Vec<Stmt>, Error> {
        let statements = Vec::new();
        while !self.check(&RIGHT_BRACE) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        self.consume(&RIGHT_BRACE, "Expect '}' after block");
        Ok(statements)
    }

    fn expression(&self) -> Result<Expr, Error> {
        self.assignment()
    }

    fn assignment(&self) -> Result<Expr, Error> {
        let expr = self.comma()?;

        if self.matching(&[EQUAL]) {
            let equals = self.previous();
            let value = self.assignment()?;

            return match expr {
                Expr::Variable(var) => {
                    let name = var;
                    let res = Ok(Expr::Assign {
                        name,
                        value: Box::new(value),
                    });
                    res
                }
                _ => Err(self.error(equals.clone(), "Invalid assignment target")),
            };
        }

        Ok(expr)
    }

    fn comma(&self) -> Result<Expr, Error> {
        let mut expr = self.ternary()?;

        while self.matching(&[COMMA]) {
            let operator = self.previous();
            let right = self.ternary()?;
            expr = Expr::Binary(Box::new(expr), operator.clone(), Box::new(right));
        }

        Ok(expr)
    }

    fn ternary(&self) -> Result<Expr, Error> {
        let mut expr = self.equality()?;

        if self.matching(&[QUESTION]) {
            let first = self.ternary()?;
            if self.matching(&[COLON]) {
                let second = self.ternary()?;
                expr = Expr::Ternary(
                    Box::new(expr),
                    Box::new(first),
                    Box::new(second),
                    self.previous().clone(),
                );
            }
        }

        Ok(expr)
    }

    fn equality(&self) -> Result<Expr, Error> {
        let mut expr = self.comparison()?;

        while self.matching(&[EQUAL_EQUAL, BANG_EQUAL]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary(Box::new(expr), operator.clone(), Box::new(right));
        }

        Ok(expr)
    }

    fn comparison(&self) -> Result<Expr, Error> {
        let mut expr = self.addition()?;

        while self.matching(&[GREATER, GREATER_EQUAL, LESS, LESS_EQUAL]) {
            let operator = self.previous();
            let right = self.addition()?;
            expr = Expr::Binary(Box::new(expr), operator.clone(), Box::new(right));
        }

        Ok(expr)
    }

    fn addition(&self) -> Result<Expr, Error> {
        let mut expr = self.multiplication()?;

        while self.matching(&[MINUS, PLUS]) {
            let operator = self.previous();
            let right = self.multiplication()?;
            expr = Expr::Binary(Box::new(expr), operator.clone(), Box::new(right));
        }

        Ok(expr)
    }

    fn multiplication(&self) -> Result<Expr, Error> {
        let mut expr = self.unary()?;

        while self.matching(&[SLASH, STAR]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary(Box::new(expr), operator.clone(), Box::new(right));
        }
        Ok(expr)
    }

    fn unary(&self) -> Result<Expr, Error> {
        if self.matching(&[BANG, MINUS]) {
            let operator = self.previous();
            let right = self.expression()?;
            return Ok(Expr::Unary(operator.clone(), Box::new(right)));
        }
        self.primary()
    }

    fn primary(&self) -> Result<Expr, Error> {
        if self.matching(&[FALSE]) {
            return Ok(Expr::LiteralExpr(Some(Bool(false))));
        }
        if self.matching(&[TRUE]) {
            return Ok(Expr::LiteralExpr(Some(Bool(true))));
        }
        if self.matching(&[NIL]) {
            return Ok(Expr::LiteralExpr(Some(Nil)));
        }
        if self.matching(&[NUMBER, STRING]) {
            let lit = self.previous().literal;
            return Ok(Expr::LiteralExpr(lit));
        }
        if self.matching(&[LEFT_PAREN]) {
            let expr = self.expression().unwrap();
            self.consume(&RIGHT_PAREN, "Expect ) after expression").ok();
            return Ok(Expr::Grouping(Box::new(expr)));
        }
        if self.is_at_end() {
            return Ok(Expr::LiteralExpr(Some(Nil)));
        }

        Err(self.error(self.peek().clone(), "Unable to resolve token here"))
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

    fn error(&self, token: Token, message: &str) -> Error {
        Error::CompileTimeError {
            token: Some(token),
            message: String::from(message),
        }
    }

    fn consume(&self, token_type: &TokenType, message: &str) -> Result<Option<Token>, Error> {
        if self.check(token_type) {
            return Ok(self.advance());
        }
        Err(self.error(self.peek().clone(), message))
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
            self.current.replace(*self.current.borrow() + 1);
        }
        None
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        (token_type) == (&self.peek().token_type)
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
