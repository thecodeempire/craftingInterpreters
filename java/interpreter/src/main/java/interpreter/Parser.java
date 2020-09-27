package interpreter;

import static interpreter.TokenType.*;

import java.util.ArrayList;
import java.util.List;

public class Parser {
  private final List<Token> tokens;
  private int current = 0;

  private static class ParseError extends RuntimeException {
  }

  Parser(final List<Token> tokens) {
    this.tokens = tokens;
  }

  List<Stmt> parse() {
    final List<Stmt> statements = new ArrayList<>();
    while (!isAtEnd())
      statements.add(declaration());

    return statements;
  }

  private Stmt declaration() {
    try {
      if (match(VAR))
        return varDeclaration();
      return statement();
    } catch (ParseError error) {
      synchronize();
      return null;
    }
  }

  private Stmt varDeclaration() {
    Token name = consume(IDENTIFIER, "Expect variable name.");
    Expr initializer = null;

    if (match(EQUAL))
      initializer = expression();

    consume(SEMICOLON, "Expect ';' after variable declaration.");
    return new Stmt.Var(name, initializer);
  }

  private Stmt statement() {
    if (match(PRINT))
      return printStatement();
    if (match(LEFT_BRACE))
      return new Stmt.Block(block());
    return expressionStatement();
  }

  private Stmt printStatement() {
    Expr value = expression();
    consume(SEMICOLON, "Expect ';' after expression.");
    return new Stmt.Print(value);
  }

  private Stmt expressionStatement() {
    Expr expr = expression();
    consume(SEMICOLON, "Expect ';' after expression.");
    return new Stmt.Expression(expr);
  }

  private List<Stmt> block() {
    List<Stmt> statements = new ArrayList<>();

    while (!check(RIGHT_BRACE) && !isAtEnd()) {
      statements.add(declaration());
    }

    consume(RIGHT_BRACE, "Expect '}' after block");
    return statements;
  }

  private Expr assignment() {
    Expr expr = equality();

    if (match(EQUAL)) {
      Token equals = previous();
      Expr value = assignment();

      if (expr instanceof Expr.Variable) {
        Token name = ((Expr.Variable) expr).name;
        return new Expr.Assign(name, value);
      }
      error(equals, "Invalid assignment target.");
    }

    return expr;
  }

  private Expr expression() {
    return assignment();
  }

  private Expr equality() {
    Expr expr = comparison();

    while (match(BANG_EQUAL, EQUAL_EQUAL)) {
      final Token operator = previous();
      final Expr right = comparison();
      expr = new Expr.Binary(expr, operator, right);
    }

    return expr;
  }

  private Expr comparison() {
    Expr expr = addition();

    while (match(GREATER, GREATER_EQUAL, LESS, LESS_EQUAL)) {
      final Token operator = previous();
      final Expr right = addition();
      expr = new Expr.Binary(expr, operator, right);
    }
    return expr;
  }

  private Expr addition() {
    Expr expr = multiplication();

    while (match(MINUS, PLUS)) {
      final Token operator = previous();
      final Expr right = multiplication();
      expr = new Expr.Binary(expr, operator, right);
    }

    return expr;
  }

  private Expr multiplication() {
    Expr expr = unary();

    while (match(SLASH, STAR)) {
      final Token operator = previous();
      final Expr right = unary();
      expr = new Expr.Binary(expr, operator, right);
    }

    return expr;
  }

  private Expr unary() {
    if (match(BANG, MINUS)) {
      final Token operator = previous();
      final Expr right = unary();
      return new Expr.Unary(operator, right);
    }

    return primary();
  }

  private Expr primary() {
    if (match(FALSE))
      return new Expr.Literal(false);
    if (match(TRUE))
      return new Expr.Literal(true);
    if (match(NIL))
      return new Expr.Literal(null);

    if (match(NUMBER, STRING))
      return new Expr.Literal(previous().literal);

    if (match(IDENTIFIER))
      return new Expr.Variable(previous());

    if (match(LEFT_PAREN)) {
      final Expr expr = expression();
      consume(RIGHT_PAREN, "Expect ) after expression");
      return new Expr.Grouping(expr);
    }

    throw error(peek(), "Expect expression..");
  }

  private void synchronize() {
    advance();

    while (!isAtEnd()) {
      if (previous().type == SEMICOLON)
        return;

      switch (peek().type) {
        case CLASS:
        case FUN:
        case VAR:
        case FOR:
        case IF:
        case WHILE:
        case PRINT:
        case RETURN:
          return;
        default:
          break;
      }

      advance();
    }
  }

  private ParseError error(final Token token, final String message) {
    App.error(token, message);
    return new ParseError();
  }

  private Token consume(final TokenType type, final String message) {
    if (check(type))
      return advance();
    throw error(peek(), message);

  }

  private boolean match(final TokenType... types) {
    for (final TokenType type : types) {
      if (check(type)) {
        advance();
        return true;
      }
    }

    return false;
  }

  private Token advance() {
    if (!isAtEnd())
      current++;
    return previous();
  }

  private boolean check(final TokenType type) {
    return isAtEnd() ? false : peek().type == type;
  }

  private Token peek() {
    return this.tokens.get(current);
  }

  private Token previous() {
    return tokens.get(current - 1);
  }

  private boolean isAtEnd() {
    return peek().type == EOF;
  }
}
