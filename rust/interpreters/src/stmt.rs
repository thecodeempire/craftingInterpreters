use crate::environment::Environment;
use crate::error::Error;
use crate::expr::Expr;
use crate::interpreter::{evaluate, Interpreter};
use crate::token::Token;
use crate::typer::Typer;

pub enum Stmt {
    Block(Box<Vec<Stmt>>), // statements
    Expression(Box<Expr>), // expr
    Print(Box<Expr>),
    Var(Box<Token>, Option<Expr>),
}

impl Stmt {
    pub fn visit(self, interpreter: &Interpreter) -> Result<(), Error> {
        match self {
            Stmt::Block(statements) => self.visitBlockStmt(*statements, interpreter),
            Stmt::Expression(expr) => self.visitExpressionStmt(*expr),
            Stmt::Print(expr) => self.visitPrintStmt(*expr),
            Stmt::Var(token, expr) => self.visitVarStmt(*token, expr, interpreter),
        }
    }

    fn execute_block(
        &self,
        statements: Vec<Self>,
        environment: Environment,
        interpreter: &Interpreter,
    ) -> Result<(), Error> {
        let previous_env = interpreter.get_environment();
        interpreter.set_environment(environment);
        for statement in statements {
            match interpreter.execute(statement) {
                Ok(()) => {}
                Err(err) => {
                    interpreter.set_environment(previous_env);
                    return Err(err);
                }
            };
        }
        interpreter.set_environment(previous_env);
        Ok(())
    }

    fn visitBlockStmt(
        &self,
        statements: Vec<Stmt>,
        interpreter: &Interpreter,
    ) -> Result<(), Error> {
        self.execute_block(
            statements,
            Environment::new(interpreter.environment.borrow()),
            interpreter,
        );
        Ok(())
    }

    fn visitExpressionStmt(&self, expr: Expr) -> Result<(), Error> {
        evaluate(expr)?;
        Ok(())
    }

    fn visitPrintStmt(&self, expr: Expr) -> Result<(), Error> {
        let value = evaluate(expr)?;
        println!("{}", value);
        Ok(())
    }

    fn visitVarStmt(
        &self,
        name: Token,
        initializer: Option<Expr>,
        interpreter: &Interpreter,
    ) -> Result<(), Error> {
        let mut value: Option<Typer> = None;
        if let Some(initializer) = initializer {
            value = Some(evaluate(initializer)?);
        }
        let mut value = &value;
        interpreter
            .environment
            .borrow_mut()
            .define(name.lexeme.clone(), *value);

        Ok(())
    }
}
