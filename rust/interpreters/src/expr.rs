use crate::error::Error;
use crate::token::{Literal, Token};
use crate::typer::Typer;

#[allow(dead_code)]
pub enum Expr {
    Ternary(Box<Expr>, Box<Expr>, Box<Expr>, Token),
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    LiteralExpr(Option<Literal>),
    Unary(Token, Box<Expr>),
    Variable(Token),
    Assign { name: Token, value: Box<Expr> },
}

pub trait Visitor<T> {
    fn visit(&self) -> T;
}

#[macro_export]
macro_rules! parenthesize {
    ($name:expr $(, $arg:expr)* ) => {{
        let mut s = String::from("(");
        s.push_str($name);
        $(
            s.push(' ');
            let string_from: String = ($arg).visit_string();
            s.push_str(&string_from);
        )*
            s.push(')');
        s
    }};
}

impl Expr {
    pub fn visit_string(self) -> String {
        match self {
            Expr::Ternary(condition, first, second, _operation) => {
                parenthesize!("ternary", condition, first, second)
            }
            Expr::Binary(left, operator, right) => parenthesize!(&operator.lexeme, left, right),
            Expr::Grouping(expr) => parenthesize!(&String::from("group"), expr),
            Expr::Unary(operator, right) => parenthesize!(&operator.lexeme, right),
            Expr::LiteralExpr(lit) => match lit {
                Some(lit) => lit.to_string(),
                None => "None".to_string(),
            },
            Expr::Variable(var) => parenthesize!(&String::from(var.lexeme)),
            Expr::Assign { name, value } => parenthesize!(&name.lexeme, value),
        }
    }
}

impl Expr {
    pub fn visit(self) -> Result<Typer, Error> {
        match self {
            Expr::Binary(left, ops, right) => self.visit_binary_expr(*left, ops, *right),
            Expr::Grouping(expr) => self.visit_grouping(*expr),
            Expr::LiteralExpr(lit) => self.visit_literal(lit.unwrap()),
            Expr::Unary(operator, operand) => self.visit_unary(operator, *operand),
            Expr::Ternary(condition, first, second, operator) => {
                self.visit_ternary(*condition, *first, *second, operator)
            }
            Expr::Variable(var) => self.visit_variable(var),
            Expr::Assign { name, value } => self.visit_assign(*value, name),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Expr::*;
    use super::Literal::*;
    use super::*;
    use crate::token::TokenType::*;

    #[test]
    fn basic() {
        let tok_minus = Token::new(MINUS, "-", None, 1);
        let tok_star = Token::new(STAR, "*", None, 1);
        let expression = Expr::Binary(
            Box::new(Expr::Unary(
                tok_minus,
                Box::new(Expr::LiteralExpr(Some(Literal::Number(123 as f64)))),
            )),
            tok_star,
            Box::new(Expr::Grouping(Box::new(Expr::LiteralExpr(Some(
                Literal::Number(45.67),
            ))))),
        );

        let stringified_expr: String = expression.visit_string();
        assert_eq!("(* (- 123) (group 45.67))", stringified_expr);
    }
    macro_rules! bx {
        ($n: expr) => {
            Box::new($n)
        };
    }

    #[test]
    fn more_ast_test() {
        let expression = Binary(
            bx!(Unary(
                Token::new(MINUS, "-", None, 1),
                bx!(LiteralExpr(Some(Number(123 as f64))))
            )),
            Token::new(STAR, "*", None, 1),
            bx!(Grouping(bx!(LiteralExpr(Some(Number(45.67)))))),
        );
        let stringified_expr: String = expression.visit_string();
        assert_eq!("(* (- 123) (group 45.67))", stringified_expr);
    }

    #[test]
    fn more_ast_test2() {
        let tok_minus = Token::new(MINUS, "-", None, 1);
        let tok_star = Token::new(STAR, "*", None, 1);
        let tok_plus = Token::new(PLUS, "+", None, 1);

        let expression = Binary(
            bx!(Grouping(bx!(Binary(
                bx!(Grouping(bx!(Binary(
                    bx!(Unary(tok_minus, bx!(LiteralExpr(Some(Number(1.2)))))),
                    tok_plus,
                    bx!(LiteralExpr(Some(Number(3.0))))
                )))),
                tok_star,
                bx!(Grouping(bx!(Binary(
                    bx!(LiteralExpr(Some(Number(4.1)))),
                    tok_minus.clone(),
                    bx!(LiteralExpr(Some(Number(3.1))))
                ))))
            )))),
            tok_star,
            bx!(LiteralExpr(Some(Number(4.1)))),
        );

        // STRING FORM: ((-1.2 + 3) * (4.1 - 3.1)) * 4.1

        let stringified_expr: String = expression.visit_string();
        assert_eq!(
            "(* (group (* (group (+ (- 1.2) 3)) (group (- 4.1 3.1)))) 4.1)",
            stringified_expr
        );
    }

    #[test]
    fn ternary_comma() {
        // EXPRESSION: 1 ? 2 : 0 ? 1 ? 0 : 3: 33
        let token = Token::new(TERNARY, "?..:", None, 33);
        let expr = Ternary(
            bx!(LiteralExpr(Some(Number(1.0)))),
            bx!(LiteralExpr(Some(Number(2.0)))),
            bx!(Ternary(
                bx!(LiteralExpr(Some(Number(0.0)))),
                bx!(Ternary(
                    bx!(LiteralExpr(Some(Number(1.0)))),
                    bx!(LiteralExpr(Some(Number(0.0)))),
                    bx!(LiteralExpr(Some(Number(3.0)))),
                    token,
                )),
                bx!(LiteralExpr(Some(Number(33.0)))),
                token,
            )),
            token,
        );

        let stringified_expr: String = expr.visit_string();
        assert_eq!(
            "(ternary 1 2 (ternary 0 (ternary 1 0 3) 33))",
            stringified_expr
        );
    }
}
