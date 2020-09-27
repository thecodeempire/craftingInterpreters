use crate::environment::Environment;
use crate::error::Error;
use crate::expr::Expr;
use crate::expr::Visitor;
use crate::stmt::Stmt;
use crate::token::Literal;
use crate::token::{Token, TokenType::*};
use crate::typer::Typer;
use std::cell::RefCell;

fn stringify(value: &Typer) -> String {
    value.to_string()
}

pub fn evaluate(value: Expr) -> Result<Typer, Error> {
    value.visit()
}

fn check_num_operand(right: &Typer, operation: &Token) -> Result<Typer, Error> {
    match right {
        Typer::Number(num) => Ok(Typer::Number(num.clone())),
        _ => Err(Error::RuntimeError {
            token: Some(operation.clone()),
            message: String::from(
                "Mismatched unary operation. Cannot perform operation on the following.",
            ),
        }),
    }
}

fn is_truthy(val: &Typer) -> bool {
    match val {
        Typer::Boolean(b) => b.clone(),
        Typer::Nil => false,
        _ => true,
    }
}

fn is_equal(left: Typer, right: Typer, ops: Token) -> Result<bool, Error> {
    let err = |s: &str| Error::RuntimeError {
        token: Some(ops.clone()),
        message: String::from(s),
    };
    match (left, right) {
        (Typer::Nil, Typer::Nil) => Ok(true),
        (Typer::Nil, _) => Ok(false),
        (Typer::Number(left), Typer::Number(right)) => Ok(left == right),
        (Typer::Str(left), Typer::Str(right)) => Ok(left == right),
        (Typer::Boolean(left), Typer::Boolean(right)) => Ok(left == right),
        (_, _) => Err(err("Mismatched types, cannot compare the two operands")),
    }
}

impl Expr {
    pub fn visit_binary_expr(self, left: Expr, ops: Token, right: Expr) -> Result<Typer, Error> {
        let left = evaluate(left)?;
        let right = evaluate(right)?;

        let err = |s: &str| Error::RuntimeError {
            token: Some(ops.clone()),
            message: String::from(s),
        };

        match ops.token_type {
            PLUS => match (left, right) {
                (Typer::Number(left), Typer::Number(right)) => Ok(Typer::Number(left + right)),
                (Typer::Str(left), Typer::Str(right)) => {
                    Ok(Typer::Str(format!("{}{}", left, right)))
                }
                (Typer::Str(left), Typer::Number(right)) => {
                    Ok(Typer::Str(format!("{}{}", left, right)))
                }
                (Typer::Str(left), Typer::Boolean(right)) => {
                    Ok(Typer::Str(format!("{}{}", left, right)))
                }
                _ => Err(err("Mismatched types. Cannot add the two operands")),
            },
            MINUS => match (left, right) {
                (Typer::Number(left), Typer::Number(right)) => Ok(Typer::Number(left - right)),
                _ => Err(err("Mismatched types. Cannot subtract the two operands")),
            },
            SLASH => match (left, right) {
                (Typer::Number(left), Typer::Number(right)) => {
                    if right == 0.0 {
                        return Err(err(
                            "Divide by zero error. The denominator is equal to zero!",
                        ));
                    }
                    Ok(Typer::Number(left / right))
                }
                _ => Err(err("Mismatched types. Cannot subtract the two operands")),
            },
            STAR => match (left, right) {
                (Typer::Number(left), Typer::Number(right)) => Ok(Typer::Number(left * right)),
                _ => Err(err("Mismatched types. Cannot multiply the two operands")),
            },
            GREATER => match (left, right) {
                (Typer::Number(left), Typer::Number(right)) => Ok(Typer::Boolean(left > right)),
                (Typer::Str(left), Typer::Str(right)) => Ok(Typer::Boolean(left > right)),
                _ => Err(err("Mismatched types. Cannot compare the two operands")),
            },
            GREATER_EQUAL => match (left, right) {
                (Typer::Number(left), Typer::Number(right)) => Ok(Typer::Boolean(left >= right)),
                (Typer::Str(left), Typer::Str(right)) => Ok(Typer::Boolean(left >= right)),
                _ => Err(err("Mismatched types. Cannot compare the two operands")),
            },
            LESS => match (left, right) {
                (Typer::Number(left), Typer::Number(right)) => Ok(Typer::Boolean(left < right)),
                (Typer::Str(left), Typer::Str(right)) => Ok(Typer::Boolean(left < right)),
                _ => Err(err("Mismatched types. Cannot compare the two operands")),
            },
            LESS_EQUAL => match (left, right) {
                (Typer::Number(left), Typer::Number(right)) => Ok(Typer::Boolean(left <= right)),
                (Typer::Str(left), Typer::Str(right)) => Ok(Typer::Boolean(left <= right)),
                _ => Err(err("Mismatched types. Cannot compare the two operands")),
            },
            BANG_EQUAL => match is_equal(left, right, ops) {
                Ok(b) => Ok(Typer::Boolean(!b)),
                Err(err) => Err(err),
            },
            EQUAL_EQUAL => match is_equal(left, right, ops) {
                Ok(b) => Ok(Typer::Boolean(b)),
                Err(err) => Err(err),
            },
            _ => Err(err(
                "Cannot fathom the binary operation. Check your code again",
            )),
        }
    }

    pub fn visit_grouping(&self, expr: Expr) -> Result<Typer, Error> {
        evaluate(expr)
    }

    pub fn visit_literal(&self, lit: Literal) -> Result<Typer, Error> {
        match lit {
            Literal::Bool(b) => Ok(Typer::Boolean(b.clone())),
            Literal::Str(b) => Ok(Typer::Str(b.clone())),
            Literal::Number(b) => Ok(Typer::Number(b.clone())),
            Literal::Nil => Ok(Typer::Nil),
        }
    }

    pub fn visit_unary(&self, operator: Token, operand: Expr) -> Result<Typer, Error> {
        let right = evaluate(operand)?;
        match operator.token_type {
            BANG => Ok(Typer::Boolean(!is_truthy(&right))),
            MINUS => {
                check_num_operand(&right, &operator)?;
                match right {
                    Typer::Number(num) => Ok(Typer::Number(-num)),
                    _ => Ok(Typer::Nil),
                }
            }
            _ => Ok(Typer::Nil),
        }
    }

    pub fn visit_ternary(
        &self,
        condition: Expr,
        first: Expr,
        second: Expr,
        operator: Token,
    ) -> Result<Typer, Error> {
        let condition = evaluate(condition)?;
        let first = evaluate(first)?;
        let second = evaluate(second)?;

        match condition {
            Typer::Boolean(b) => Ok(if b { first } else { second }),
            Typer::Nil => Ok(second),
            _ => Err(Error::RuntimeError {
                token: Some(operator.clone()),
                message: format!("ternary operation failed."),
            }),
        }
    }

    pub fn visit_assign(&self, name: Expr, token: Token) -> Result<Typer, Error> {
        Ok(Typer::Nil)
    }

    pub fn visit_variable(&self, var: Token) -> Result<Typer, Error> {
        Ok(Typer::Nil)
    }
}

pub struct Interpreter {
    pub environment: RefCell<Environment>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: RefCell::new(Environment::new_empty_env()),
        }
    }

    pub fn interpret(&self, statements: Vec<Stmt>) -> Result<String, Error> {
        for statement in statements {
            self.execute(statement)?;
        }
        Ok("".to_string())
    }

    pub fn set_environment(&self, env: Environment) -> Result<(), Error> {
        self.environment.replace(env);
        Ok(())
    }

    pub fn get_environment(&self) -> Environment {
        *self.environment.borrow()
    }

    pub fn execute(&self, statement: Stmt) -> Result<(), Error> {
        statement.visit(self)
    }
}

#[cfg(test)]
mod tests {
    // use super::Expr::*;
    // use super::*;
    // use crate::parser::Parser;
    // use crate::scanner::Scanner;
    // use crate::token::Literal::*;

    // macro_rules! bx {
    //     ($n: expr) => {
    //         Box::new($n)
    //     };
    // }

    // macro_rules! num {
    //     ($n: literal) => {
    //         LiteralExpr(Some(&Number($n)))
    //     };
    // }

    // fn get_str(expr: &Expr) -> String {
    //     Interpreter::new(&Runner::new())
    //         .interpret(&expr)
    //         .unwrap_or_else(|_err| String::from("error"))
    // }

    // fn get_interpreted_str_from_expr_str(st: &str) -> String {
    //     let runner = Runner::new();
    //     let mut scanner = Scanner::new(String::from(st), &runner);
    //     let tokens = scanner.scan_tokens();
    //     let parser = Parser::new(tokens, &runner);
    //     let expr = parser.parse().unwrap();
    //     let st = get_str(&expr);
    //     st
    // }

    // #[test]
    // fn test_basic_numeric_ops() {
    //     let add = Token::new(PLUS, "+", None, 1);
    //     let minus = Token::new(MINUS, "-", None, 1);
    //     let star = Token::new(STAR, "*", None, 1);

    //     let expr = Binary(bx!(num!(12.0)), &add, bx!(num!(24.0)));
    //     let st = get_str(&expr);
    //     assert_eq!(st, String::from("36"));

    //     let expr = Binary(bx!(num!(1.2)), &minus, bx!(num!(36.11)));
    //     let st = get_str(&expr);
    //     assert_eq!(st, String::from("-34.91"));

    //     let expr = Binary(
    //         bx!(Binary(bx!(num!(1.3)), &add, bx!(num!(45.0)))),
    //         &star,
    //         bx!(num!(2.0)),
    //     );

    //     assert_eq!(get_str(&expr), String::from("92.6"));
    // }

    // #[test]
    // fn test_interpreting_string_form() {
    //     assert_eq!(get_interpreted_str_from_expr_str("4"), "4");
    //     assert_eq!(get_interpreted_str_from_expr_str("4 + 5 "), "9");
    //     assert_eq!(
    //         get_interpreted_str_from_expr_str("(3 / 2 * 6 + 9 - 5) + (4 * 1.2) * 7"),
    //         "46.6"
    //     );
    //     assert_eq!(
    //         get_interpreted_str_from_expr_str("\"abc\" + \", yo\""),
    //         "abc, yo"
    //     );
    // }

    // #[test]
    // fn test_unary_operators() {
    //     assert_eq!(get_interpreted_str_from_expr_str("-4"), "-4");
    //     assert_eq!(get_interpreted_str_from_expr_str("!true"), "false");
    //     assert_eq!(get_interpreted_str_from_expr_str("!nil"), "true");
    // }

    // #[test]
    // fn exercises() {
    //     // comparison of strings
    //     assert_eq!(
    //         get_interpreted_str_from_expr_str("\"hello\" > \"hell\""),
    //         "true"
    //     );
    //     assert_eq!(
    //         get_interpreted_str_from_expr_str("\"thing\" > \"thug\""),
    //         "false"
    //     );

    //     // checking if string + float/boolean works
    //     assert_eq!(
    //         get_interpreted_str_from_expr_str("\"hello\" + 56.9"),
    //         "hello56.9"
    //     );
    //     assert_eq!(
    //         get_interpreted_str_from_expr_str("\"hello\" + true"),
    //         "hellotrue"
    //     );

    //     // divide by zero error
    //     assert_eq!(get_interpreted_str_from_expr_str("4/0"), "error")
    // }
}
