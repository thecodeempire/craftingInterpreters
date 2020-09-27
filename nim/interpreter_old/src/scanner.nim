import token
import options

type
  Scanner* = object
    source: string
    tokens: seq[Token]
    start: int
    current: int
    line: int

proc getTokenType(lexeme: string): TokenType=
  case lexeme:
  of "and": AND
  of "class": CLASS
  of "else": ELSE
  of "false": FALSE
  of "for": FOR
  of "fun": FUN
  of "if": IF
  of "nil": NIL
  of "or": OR
  of "print": PRINT
  of "return": RETURN
  of "super": SUPER
  of "this": THIS
  of "true": TRUE
  of "var": VAR
  of "while": WHILE
  else: NIL

# FEW FUNCTION DECLARATIONS
proc isAtEnd(self: Scanner): bool
proc isAlpha(self: Scanner): bool
proc identifier(self: Scanner)
proc number(self: Scanner)
proc peekNext(self: Scanner)
proc string(self: var Scanner)
proc peek(self: Scanner): char
proc match(self: var Scanner, expected: char): bool
proc advance(self: var Scanner): char
proc addToken(self: var Scanner, token_type: TokenType)
proc addToken(self: var Scanner, token_type: TokenType, literal: Option[Literal])
proc scanToken(scanner: var Scanner)

proc scanTokens(scanner: var Scanner): seq[Token]=
  while not isAtEnd(scanner):
    scanner.start = scanner.current
    scanToken(scanner)

  scanner.tokens.add(newToken(EOF, none(Literal), scanner.line))
  return scanner.tokens


proc scanToken(self: var Scanner)=
  let c = advance(self)

  proc matcher(c: char, f: TokenType, s: TokenType):TokenType= return if match(self, c): f else: s

  case c:
  of '(': addToken(self, LEFT_PAREN)
  of ')': addToken(self, RIGHT_PAREN)
  of '{': addToken(self, LEFT_BRACE)
  of '}': addToken(self, RIGHT_BRACE)
  of ',': addToken(self, COMMA)
  of '.': addToken(self, DOT)
  of '-': addToken(self, MINUS)
  of '+': addToken(self, PLUS)
  of ';': addToken(self, SEMICOLON)
  of '*': addToken(self, STAR)

  of '!': addToken(self, matcher('=', BANG_EQUAL, BANG))
  of '=': addToken(self, matcher('=', EQUAL_EQUAL, EQUAL))
  of '<': addToken(self, matcher('=', LESS_EQUAL, LESS))
  of '>': addToken(self, matcher('=', GREATER_EQUAL, GREATER))
  of '/':
    if match(self, '/'):
      while peek(self) != '\n' and not isAtEnd(self):
        self.advance(self.peek())
    else if match(self, '*'):
      while not isAtEnd(self):
        if peek(self) == '*' and peekNext(self) == '/':
          self.advance()

    addToken(self, LEFT_PAREN)

  of '{': addToken(self, LEFT_PAREN)
  of '{': addToken(self, LEFT_PAREN)
  of '{': addToken(self, LEFT_PAREN)
  of '{': addToken(self, LEFT_PAREN)
  of '{': addToken(self, LEFT_PAREN)
  of '}'
  
  discard

proc isAlpha(self: Scanner): bool=
  return true

proc identifier(self: Scanner)=
  discard

proc number(self: Scanner)=
  discard

proc peekNext(self: Scanner)=
  discard

proc string(self: var Scanner)=
  discard

proc peek(self: Scanner): char=
  discard

proc match(self: var Scanner, expected: char): bool=
  discard

proc advance(self: var Scanner): char=
  return ' ';

proc addToken(self: var Scanner, token_type: TokenType)=
  discard

proc addToken(self: var Scanner, token_type: TokenType, literal: Option[Literal])=
  discard

proc isAtEnd(self: Scanner): bool=
  return true
 
