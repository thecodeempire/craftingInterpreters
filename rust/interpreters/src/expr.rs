use crate::token::{Literal, Token};

#[allow(dead_code)]
pub enum Expr<'a> {
    Ternary(Box<Expr<'a>>, Box<Expr<'a>>, Box<Expr<'a>>),
    Binary(Box<Expr<'a>>, &'a Token, Box<Expr<'a>>),
    Grouping(Box<Expr<'a>>),
    LiteralExpr(Option<&'a Literal>),
    Unary(&'a Token, Box<Expr<'a>>),
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
           let string_from: String = (&$arg).visit();
           s.push_str(&string_from);
        )*
       s.push(')');
       s
    }};
}

impl<'a> Visitor<String> for Expr<'a> {
    fn visit(&self) -> String {
        match self {
            Expr::Ternary(condition, first, second) => {
                parenthesize!("ternary", condition, first, second)
            }
            Expr::Binary(left, operator, right) => parenthesize!(&operator.lexeme, left, right),
            Expr::Grouping(expr) => parenthesize!(&String::from("group"), expr),
            Expr::Unary(operator, right) => parenthesize!(&operator.lexeme, right),
            Expr::LiteralExpr(lit) => match lit {
                Some(lit) => lit.to_string(),
                None => "None".to_string(),
            },
        }
    }
}

impl<'a> Visitor<bool> for Expr<'a> {
    fn visit(&self) -> bool {
        true
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
                &tok_minus,
                Box::new(Expr::LiteralExpr(Some(&Literal::Number(123 as f64)))),
            )),
            &tok_star,
            Box::new(Expr::Grouping(Box::new(Expr::LiteralExpr(Some(
                &Literal::Number(45.67),
            ))))),
        );

        let stringified_expr: String = expression.visit();
        assert_eq!("(* (- 123) (group 45.67))", stringified_expr);
    }
    macro_rules! bx {
        ($n: expr) => {
            Box::new($n)
        };
    }

    #[test]
    fn more_ast_test() {
        let tok_minus = Token::new(MINUS, "-", None, 1);
        let tok_star = Token::new(STAR, "*", None, 1);

        let expression = Binary(
            bx!(Unary(
                &tok_minus,
                bx!(LiteralExpr(Some(&Number(123 as f64))))
            )),
            &tok_star,
            bx!(Grouping(bx!(LiteralExpr(Some(&Number(45.67)))))),
        );
        let stringified_expr: String = expression.visit();
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
                    bx!(Unary(&tok_minus, bx!(LiteralExpr(Some(&Number(1.2)))))),
                    &tok_plus,
                    bx!(LiteralExpr(Some(&Number(3.0))))
                )))),
                &tok_star,
                bx!(Grouping(bx!(Binary(
                    bx!(LiteralExpr(Some(&Number(4.1)))),
                    &tok_minus,
                    bx!(LiteralExpr(Some(&Number(3.1))))
                ))))
            )))),
            &tok_star,
            bx!(LiteralExpr(Some(&Number(4.1)))),
        );

        // STRING FORM: ((-1.2 + 3) * (4.1 - 3.1)) * 4.1

        let stringified_expr: String = expression.visit();
        assert_eq!(
            "(* (group (* (group (+ (- 1.2) 3)) (group (- 4.1 3.1)))) 4.1)",
            stringified_expr
        );
    }

    #[test]
    fn ternary_comma() {
        // EXPRESSION: 1 ? 2 : 0 ? 1 ? 0 : 3: 33
        let expr = Ternary(
            bx!(LiteralExpr(Some(&Number(1.0)))),
            bx!(LiteralExpr(Some(&Number(2.0)))),
            bx!(Ternary(
                bx!(LiteralExpr(Some(&Number(0.0)))),
                bx!(Ternary(
                    bx!(LiteralExpr(Some(&Number(1.0)))),
                    bx!(LiteralExpr(Some(&Number(0.0)))),
                    bx!(LiteralExpr(Some(&Number(3.0))))
                )),
                bx!(LiteralExpr(Some(&Number(33.0))))
            )),
        );

        let stringified_expr: String = expr.visit();
        assert_eq!(
            "(ternary 1 2 (ternary 0 (ternary 1 0 3) 33))",
            stringified_expr
        );
    }
}
