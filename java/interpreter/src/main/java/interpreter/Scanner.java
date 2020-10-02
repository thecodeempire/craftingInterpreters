package interpreter;

import static interpreter.TokenType.AND;
import static interpreter.TokenType.BANG;
import static interpreter.TokenType.BANG_EQUAL;
import static interpreter.TokenType.CLASS;
import static interpreter.TokenType.COMMA;
import static interpreter.TokenType.DOT;
import static interpreter.TokenType.ELSE;
import static interpreter.TokenType.EOF;
import static interpreter.TokenType.EQUAL;
import static interpreter.TokenType.EQUAL_EQUAL;
import static interpreter.TokenType.FALSE;
import static interpreter.TokenType.FOR;
import static interpreter.TokenType.FUN;
import static interpreter.TokenType.GREATER;
import static interpreter.TokenType.GREATER_EQUAL;
import static interpreter.TokenType.IDENTIFIER;
import static interpreter.TokenType.IF;
import static interpreter.TokenType.LEFT_BRACE;
import static interpreter.TokenType.LEFT_PAREN;
import static interpreter.TokenType.LESS;
import static interpreter.TokenType.LESS_EQUAL;
import static interpreter.TokenType.MINUS;
import static interpreter.TokenType.NIL;
import static interpreter.TokenType.NUMBER;
import static interpreter.TokenType.OR;
import static interpreter.TokenType.PLUS;
import static interpreter.TokenType.PRINT;
import static interpreter.TokenType.RETURN;
import static interpreter.TokenType.RIGHT_BRACE;
import static interpreter.TokenType.RIGHT_PAREN;
import static interpreter.TokenType.SEMICOLON;
import static interpreter.TokenType.SLASH;
import static interpreter.TokenType.STAR;
import static interpreter.TokenType.STRING;
import static interpreter.TokenType.SUPER;
import static interpreter.TokenType.THIS;
import static interpreter.TokenType.TRUE;
import static interpreter.TokenType.VAR;
import static interpreter.TokenType.WHILE;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class Scanner {
  private final String source;
  private final List<Token> tokens = new ArrayList<>();
  private int start;
  private int current;
  private int line;

  private static final Map<String, TokenType> keywords;

  static {
    keywords = new HashMap<>();
    keywords.put("and", AND);
    keywords.put("class", CLASS);
    keywords.put("else", ELSE);
    keywords.put("false", FALSE);
    keywords.put("for", FOR);
    keywords.put("fun", FUN);
    keywords.put("if", IF);
    keywords.put("nil", NIL);
    keywords.put("or", OR);
    keywords.put("print", PRINT);
    keywords.put("return", RETURN);
    keywords.put("super", SUPER);
    keywords.put("this", THIS);
    keywords.put("true", TRUE);
    keywords.put("var", VAR);
    keywords.put("while", WHILE);
  }

  Scanner(final String source) {
    this.source = source;
  }

  List<Token> scanTokens() {
    while (!isAtEnd()) {
      // We are at the beginning of the next lexeme
      start = current;
      scanToken();
    }

    tokens.add(new Token(EOF, "", null, line));
    return tokens;
  }

  // recognizing lexemes
  private void scanToken() {
    final char c = advance();

    // --------------single character lexemes -----------
    switch (c) {
      case '(':
        addToken(LEFT_PAREN);
        break;
      case ')':
        addToken(RIGHT_PAREN);
        break;
      case '{':
        addToken(LEFT_BRACE);
        break;
      case '}':
        addToken(RIGHT_BRACE);
        break;
      case ',':
        addToken(COMMA);
        break;
      case '.':
        addToken(DOT);
        break;
      case '-':
        addToken(MINUS);
        break;
      case '+':
        addToken(PLUS);
        break;
      case ';':
        addToken(SEMICOLON);
        break;
      case '*':
        addToken(STAR);
        break;

      // ----------- multi token OPERATORS (!=, <=, etc) -----------
      case '!':
        addToken(match('=') ? BANG_EQUAL : BANG);
        break;
      case '=':
        addToken(match('=') ? EQUAL_EQUAL : EQUAL);
        break;
      case '<':
        addToken(match('=') ? LESS_EQUAL : LESS);
        break;
      case '>':
        addToken(match('=') ? GREATER_EQUAL : GREATER);
        break;
      case '/':
        if (match('/')) {
          // a comment goes until the end of the line
          while (peek() != '\n' && !isAtEnd())
            advance();
        } else if (match('*')) {
          // multi-line comment
          while (!isAtEnd()) {
            if (peek() == '*' && peekNext() == '/') {
              advance();
              advance();
              break;
            } else {
              advance();
            }
          }
        } else {
          addToken(SLASH);
        }
        break;

      case ' ':
      case '\r':
      case '\t':
        // ignore whitespace
        break;

      case '\n':
        line++;
        break;

      case '"':
        string();
        break;

      default:
        if (isDigit(c)) {
          number();
        } else if (isAlpha(c)) {
          identifier();
        } else {
          App.error(new Token(null, String.valueOf(c), null, this.line), "Unexpected character.");
        }
        break;
    }
  }

  private boolean isAlpha(final char c) {
    return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c == '_');
  }

  private void identifier() {
    while (isAlphaNumeric(peek()))
      advance();

    // See if the identifier is a reserved word.
    String text = source.substring(start, current);

    TokenType type = keywords.get(text);
    if (type == null)
      type = IDENTIFIER;

    addToken(type);
  }

  private boolean isAlphaNumeric(final char c) {
    return isAlpha(c) || isDigit(c);
  }

  private void number() {
    while (isDigit(peek()))
      advance();

    // Look for a fractional part.
    if (peek() == '.' && isDigit(peekNext())) {
      // Consume the '.'
      advance();
      while (isDigit(peek()))
        advance();
    }

    addToken(NUMBER, Double.parseDouble(source.substring(start, current)));
  }

  private char peekNext() {
    if (current + 1 >= source.length())
      return '\0';
    return source.charAt(current + 1);
  }

  private boolean isDigit(final char c) {
    return c >= '0' && c <= '9';
  }

  private void string() {
    while (peek() != '"' && !isAtEnd()) {
      if (peek() == '\n')
        line++;
      advance();
    }

    // Unterminated String
    if (isAtEnd()) {
      App.error(new Token(null, "EOF", null, this.line), "Unterminated String");
      return;
    }

    // the closing ".
    advance();

    // Trim the surrounding quotes
    final String value = source.substring(start + 1, current - 1);
    addToken(STRING, value);
  }

  private char peek() {
    if (isAtEnd())
      return '\0';
    return source.charAt(current);
  }

  private boolean match(final char expected) {
    if (isAtEnd())
      return false;
    if (source.charAt(current) != expected) {
      return false;
    }
    current++;
    return true;
  }

  private char advance() {
    this.current++;
    return source.charAt(current - 1);
  }

  private void addToken(final TokenType type) {
    addToken(type, null);
  }

  private void addToken(final TokenType type, final Object literal) {
    final String text = source.substring(start, current);
    tokens.add(new Token(type, text, literal, line));
  }

  private boolean isAtEnd() {
    return current >= source.length();
  }

}
