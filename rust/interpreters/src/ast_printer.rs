use crate::expr::{Expr, Visitor};

#[macro_export]
macro_rules! parenthesize_rpn {
    ($name:expr $(, $arg:expr)* ) => {{
       let mut s = String::new();
       $(
           let string_from: String = $arg.print_rpn();
           s.push_str(&string_from);
           s.push(' ');
        )*
        s.push_str(&$name);
        s
    }};
}

impl<'a> Expr<'a> {
    pub fn print(&self) -> String {
        let a: String = self.visit();
        a
    }
}

pub trait RPN {
    fn print_rpn(&self) -> String;
}

impl<'a> RPN for Expr<'a> {
    fn print_rpn(&self) -> String {
        match self {
            Expr::Binary(left_expr, ops, right_expr) => {
                parenthesize_rpn!(&ops.lexeme, &left_expr, &right_expr)
            }
            Expr::Grouping(expr) => parenthesize_rpn!(&String::from(""), &expr),
            Expr::LiteralExpr(literal) => match literal {
                Some(lit) => lit.to_string(),
                None => String::from("None"),
            },
            Expr::Unary(operator, right) => parenthesize_rpn!(&operator.lexeme, &right),
            Expr::Ternary(condition, first, second) => {
                parenthesize_rpn!("ternary", condition, first, second)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::expr::Expr::{Binary, Grouping, LiteralExpr, Unary};
    use crate::token::{Literal::Number, Token, TokenType};

    #[test]
    fn ast_printer_basics() {
        let tok_star = &Token::new(TokenType::STAR, "*", None, 1);
        let tok_minus = &Token::new(TokenType::MINUS, "-", None, 1);

        let expression = Binary(
            Box::new(Unary(
                tok_minus,
                Box::new(LiteralExpr(Some(&Number(123 as f64)))),
            )),
            &tok_star,
            Box::new(Grouping(Box::new(LiteralExpr(Some(&Number(45.67)))))),
        );

        assert_eq!("(* (- 123) (group 45.67))", expression.print());
    }

    #[test]
    fn ast_printer_rpn() {
        let tok_star = Token::new(TokenType::STAR, "*", None, 1);
        let tok_plus = Token::new(TokenType::PLUS, "+", None, 1);
        let tok_minus = Token::new(TokenType::MINUS, "-", None, 1);
        let expression = Binary(
            Box::new(Binary(
                Box::new(LiteralExpr(Some(&Number(1 as f64)))),
                &tok_plus,
                Box::new(LiteralExpr(Some(&Number(2 as f64)))),
            )),
            &tok_star,
            Box::new(Binary(
                Box::new(LiteralExpr(Some(&Number(4 as f64)))),
                &tok_minus,
                Box::new(LiteralExpr(Some(&Number(3 as f64)))),
            )),
        );

        assert_eq!("1 2 + 4 3 - *", expression.print_rpn());
    }
}
