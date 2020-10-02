import common_types
import errors
import strutils
import token

# ------- declarations -------
proc isAtEnd(self: Scanner): bool
proc identifier(self: var Scanner)
proc peek(self: Scanner): char
proc peekNext(self: Scanner): char
proc parseStr(self: var Scanner)
proc parseNum(self: var Scanner)
proc match(self: var Scanner, expected: char): bool
proc advance(self: var Scanner): char
proc addToken(self: var Scanner, token_type: TokenType, literal: Literal = Literal(kind: NIL_LIT, nilVal: 0))
# ----------------------------

proc newScanner*(source: string): Scanner=
  return Scanner(source: source, tokens: @[], start: 0, current: 0, line: 0)

proc getKeyword(str: string): TokenType=
  case str:
  of "and": return AND
  of "class": return CLASS
  of "else": return ELSE
  of "false": return FALSE
  of "for": return FOR
  of "fun": return FUN
  of "if": return IF
  of "nil": return NIL
  of "or": return OR
  of "print": return PRINT
  of "return": return RETURN
  of "super": return SUPER
  of "this": return THIS
  of "true": return TRUE
  of "var": return VAR
  of "while": return WHILE
  else: return UNDEFINED

proc matcher(self: var Scanner, c: char, a: TokenType, b: TokenType): TokenType=
    return if self.match(c): a else: b
   
proc scanToken(self: var Scanner)=
  let c = self.advance()

  case c:
  of '(': self.addToken(LEFT_PAREN)
  of ')': self.addToken(RIGHT_PAREN)
  of '{': self.addToken(LEFT_BRACE)
  of '}': self.addToken(RIGHT_BRACE)
  of ',': self.addToken(COMMA)
  of '.': self.addToken(DOT)
  of '-': self.addToken(MINUS)
  of '+': self.addToken(PLUS)
  of ';': self.addToken(SEMICOLON)
  of '*': self.addToken(STAR)
  of '!': self.addToken(self.matcher('=', BANG_EQUAL, BANG))
  of '=': self.addToken(self.matcher('=', EQUAL_EQUAL, EQUAL))
  of '<': self.addToken(self.matcher('=', LESS_EQUAL, LESS))
  of '>': self.addToken(self.matcher('=', GREATER_EQUAL, GREATER))
  of '?': self.addToken(QUESTION)
  of ':': self.addToken(COLON)
  of '/':
    if self.match('/'):
      while (self.peek() != '\n') and (not self.isAtEnd()):
        discard self.advance()
    elif self.match('*'):
      while not self.isAtEnd():
        if self.peek() == '*' and self.peekNext() == '/':
          discard self.advance()
          discard self.advance()
          break
        else: discard self.advance()
    else: self.addToken(SLASH)
  of ' ', '\r', '\t': discard
  of '\n': self.line += 1
  of '"': self.parseStr()
  else:
    if c.isDigit(): self.parseNum()
    elif c.isAlphaAscii() or c == '_': self.identifier()
    else: compileTimeError(newToken(NIL, self.line, $c), "Unexpected character.")

proc scanTokens*(self: var Scanner): seq[Token]=
  while not self.isAtEnd():
    self.start = self.current
    self.scanToken()

  self.tokens.add(newToken(EOF, self.line))
  return self.tokens

proc isAtEnd(self: Scanner): bool=
  return self.current >= self.source.len()

proc identifier(self: var Scanner)=
  while self.peek().isAlphaNumeric():
    discard self.advance()
  let text = self.source.substr(self.start, self.current).strip()
  var token_type = getKeyword(text)
  if token_type == UNDEFINED:
    token_type = IDENTIFIER
  self.addToken(token_type)

proc peek(self: Scanner): char=
  if self.isAtEnd():
    return '\0'
  return self.source[self.current]

proc peekNext(self: Scanner): char=
  if self.current + 1 >= self.source.len():
    return '\0'
  return self.source[self.current + 1]

proc parseStr(self: var Scanner)=
  while self.peek() != '"' and not self.isAtEnd():
    if self.peek() == '\n':
      self.line += 1
    discard self.advance()

  if self.isAtEnd():
    compileTimeError(newToken(EOF, self.line, "EOF"), "Unterminated String")
    return

  discard self.advance()

  let value = self.source.substr(self.start + 1, self.current - 2)
  self.addToken(STRING, Literal(kind: STRING_LIT, strVal: value))

proc parseNum(self: var Scanner)=
  var isFloat = false
  while self.peek().isDigit():
    discard self.advance()
  if self.peek() == '.' and self.peekNext().isDigit():
    isFloat = true
    discard self.advance()
    while self.peek().isDigit():
      discard self.advance()

  let strNum = self.source.substr(self.start, self.current - 1)
  let num = if isFloat: strNum.parseFloat() else: float(strNum.parseInt())
  self.addToken(NUMBER, Literal(kind: NUMBER_LIT, numVal: num))

proc match(self: var Scanner, expected: char): bool=
  if self.isAtEnd() or self.source[self.current] != expected:
    return false

  self.current += 1
  return true

proc advance(self: var Scanner): char=
  self.current += 1
  return self.source[self.current - 1]

proc addToken(self: var Scanner, token_type: TokenType, literal: Literal = Literal(kind: NIL_LIT, nilVal: 0))=
  let text = self.source.substr(self.start, self.current - 1)
  self.tokens.add(newToken(token_type, self.line, text, literal))

