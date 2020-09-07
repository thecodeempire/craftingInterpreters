use crate::error::Error;
use crate::expr::Expr;
use crate::expr::Visitor;
use crate::token::Literal;
use crate::token::{Token, TokenType::*};
use crate::typer::Typer;
use crate::Runner;

pub struct Interpreter<'a> {
    runner: &'a Runner,
}

impl<'a> Visitor<Result<Typer, Box<Error<'a>>>> for Expr<'a> {
    fn visit(&self) -> Result<Typer, Box<Error<'a>>> {
        match self {
            Expr::Binary(left, ops, right) => self.visit_binary_expr(left, ops, right),
            Expr::Grouping(expr) => self.visit_grouping(expr),
            Expr::LiteralExpr(lit) => self.visit_literal(lit.unwrap()),
            Expr::Unary(operator, operand) => self.visit_unary(*operator, &operand),
            Expr::Ternary(condition, first, second, operator) => {
                self.visit_ternary(condition, first, second, operator)
            }
        }
    }
}

fn stringify(value: &Typer) -> String {
    value.to_string()
}

fn evaluate<'a>(value: &Expr<'a>) -> Result<Typer, Box<Error<'a>>> {
    value.visit()
}

fn check_num_operand<'a>(right: &Typer, operation: &'a Token) -> Result<Typer, Box<Error<'a>>> {
    match right {
        Typer::Number(num) => Ok(Typer::Number(*num)),
        _ => Err(Box::new(Error::RuntimeError {
            token: operation,
            message: String::from(
                "Mismatched unary operation. Cannot perform operation on the following.",
            ),
        })),
    }
}

fn is_truthy(val: &Typer) -> bool {
    match val {
        Typer::Boolean(b) => b.clone(),
        Typer::Nil => false,
        _ => true,
    }
}

fn is_equal<'a>(left: Typer, right: Typer, ops: &'a Token) -> Result<bool, Box<Error<'a>>> {
    let err = |s: &str| {
        Box::new(Error::RuntimeError {
            token: ops,
            message: String::from(s),
        })
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

impl<'a> Expr<'a> {
    fn visit_binary_expr(
        &self,
        left: &Expr<'a>,
        ops: &'a Token,
        right: &Expr<'a>,
    ) -> Result<Typer, Box<Error<'a>>> {
        let left = evaluate(left)?;
        let right = evaluate(right)?;

        let err = |s: &str| {
            Box::new(Error::RuntimeError {
                token: ops,
                message: String::from(s),
            })
        };

        match ops.token_type {
            PLUS => match (left, right) {
                (Typer::Number(left), Typer::Number(right)) => Ok(Typer::Number(left + right)),
                (Typer::Str(left), Typer::Str(right)) => {
                    Ok(Typer::Str(format!("{}{}", left, right)))
                }
                _ => Err(err("Mismatched types. Cannot add the two operands")),
            },
            MINUS => match (left, right) {
                (Typer::Number(left), Typer::Number(right)) => Ok(Typer::Number(left - right)),
                _ => Err(err("Mismatched types. Cannot subtract the two operands")),
            },
            SLASH => match (left, right) {
                (Typer::Number(left), Typer::Number(right)) => Ok(Typer::Number(left / right)),
                _ => Err(err("Mismatched types. Cannot subtract the two operands")),
            },
            STAR => match (left, right) {
                (Typer::Number(left), Typer::Number(right)) => Ok(Typer::Number(left * right)),
                _ => Err(err("Mismatched types. Cannot multiply the two operands")),
            },
            GREATER => match (left, right) {
                (Typer::Number(left), Typer::Number(right)) => Ok(Typer::Boolean(left > right)),
                _ => Err(err("Mismatched types. Cannot compare the two operands")),
            },
            GREATER_EQUAL => match (left, right) {
                (Typer::Number(left), Typer::Number(right)) => Ok(Typer::Boolean(left >= right)),
                _ => Err(err("Mismatched types. Cannot compare the two operands")),
            },
            LESS => match (left, right) {
                (Typer::Number(left), Typer::Number(right)) => Ok(Typer::Boolean(left < right)),
                _ => Err(err("Mismatched types. Cannot compare the two operands")),
            },
            LESS_EQUAL => match (left, right) {
                (Typer::Number(left), Typer::Number(right)) => Ok(Typer::Boolean(left <= right)),
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

    fn visit_grouping(&self, expr: &Expr<'a>) -> Result<Typer, Box<Error<'a>>> {
        evaluate(expr)
    }

    fn visit_literal(&self, lit: &Literal) -> Result<Typer, Box<Error<'a>>> {
        match lit {
            Literal::Bool(b) => Ok(Typer::Boolean(b.clone())),
            Literal::Str(b) => Ok(Typer::Str(b.clone())),
            Literal::Number(b) => Ok(Typer::Number(b.clone())),
            Literal::Nil => Ok(Typer::Nil),
        }
    }

    fn visit_unary(
        &self,
        operator: &'a Token,
        operand: &Expr<'a>,
    ) -> Result<Typer, Box<Error<'a>>> {
        let right = evaluate(operand)?;
        match operator.token_type {
            BANG => Ok(Typer::Boolean(!is_truthy(&right))),
            MINUS => check_num_operand(&right, operator),
            _ => Ok(Typer::Nil),
        }
    }

    fn visit_ternary(
        &self,
        condition: &Expr<'a>,
        first: &Expr<'a>,
        second: &Expr<'a>,
        operator: &'a Token,
    ) -> Result<Typer, Box<Error<'a>>> {
        let condition = evaluate(condition)?;
        let first = evaluate(first)?;
        let second = evaluate(second)?;

        match condition {
            Typer::Boolean(b) => Ok(if b { first } else { second }),
            Typer::Nil => Ok(second),
            _ => Err(Box::new(Error::RuntimeError {
                token: &operator,
                message: format!("ternary operation failed."),
            })),
        }
    }
}

impl<'a> Interpreter<'a> {
    pub fn new(runner: &'a Runner) -> Self {
        Self { runner }
    }

    pub fn interpret(&self, expr: &Expr<'a>) -> Result<String, ()> {
        match evaluate(expr) {
            Ok(value) => Ok(stringify(&value)),
            Err(err) => Err(self.runner.runtime_error(&err)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Expr::*;
    use super::*;
    use crate::parser::Parser;
    use crate::scanner::Scanner;
    use crate::token::Literal::*;

    macro_rules! bx {
        ($n: expr) => {
            Box::new($n)
        };
    }

    macro_rules! num {
        ($n: literal) => {
            LiteralExpr(Some(&Number($n)))
        };
    }

    fn get_str(expr: &Expr) -> String {
        Interpreter::new(&Runner::new())
            .interpret(&expr)
            .unwrap_or_else(|_err| String::new())
    }

    fn get_interpreted_str_from_expr_str(st: &str) -> String {
        let runner = Runner::new();
        let mut scanner = Scanner::new(String::from(st), &runner);
        let tokens = scanner.scan_tokens();
        let parser = Parser::new(tokens, &runner);
        let expr = parser.parse().unwrap();
        let st = get_str(&expr);

        st
    }

    #[test]
    fn test_basic_numeric_ops() {
        let add = Token::new(PLUS, "+", None, 1);
        let minus = Token::new(MINUS, "-", None, 1);
        let star = Token::new(STAR, "*", None, 1);

        let expr = Binary(bx!(num!(12.0)), &add, bx!(num!(24.0)));
        let st = get_str(&expr);
        assert_eq!(st, String::from("36"));

        let expr = Binary(bx!(num!(1.2)), &minus, bx!(num!(36.11)));
        let st = get_str(&expr);
        assert_eq!(st, String::from("-34.91"));

        let expr = Binary(
            bx!(Binary(bx!(num!(1.3)), &add, bx!(num!(45.0)))),
            &star,
            bx!(num!(2.0)),
        );

        assert_eq!(get_str(&expr), String::from("92.6"));
    }

    #[test]
    fn test_interpretion_string_form() {
        assert_eq!(get_interpreted_str_from_expr_str("4"), "4");
        assert_eq!(get_interpreted_str_from_expr_str("4 + 5 "), "9");
        assert_eq!(
            get_interpreted_str_from_expr_str("(3 / 2 * 6 + 9 - 5) + (4 * 1.2) * 7"),
            "46.6"
        );
    }
}
