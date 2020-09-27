type
  TokenType* = enum
    # ------ SINGLE CHARACTER TOKENS ----------
    LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE, COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR, QUESTION, COLON
    # ------ ONE OR TWO CHARACTER TOKENS --------
    BANG, BANG_EQUAL, EQUAL, EQUAL_EQUAL, GREATER, GREATER_EQUAL, LESS, LESS_EQUAL,
    # ------ LITERALS ----------
    IDENTIFIER, STRING, NUMBER,
    # ------ KEYWORDS -------
    AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR, PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,
    # -------- OTHERS ---------
    EOF,
    # ------- PLACEHOLDER - Only for compiler purpose ------
    UNDEFINED

type
  LiteralKind* = enum
    NUMBER_LIT,
    STRING_LIT,
    BOOLEAN_LIT
    NIL_LIT
  Literal* = object
    case kind*: LiteralKind
    of NUMBER_LIT: numVal*: float
    of STRING_LIT: strVal*: string
    of BOOLEAN_LIT: boolVal*: bool
    of NIL_LIT: nilVal*: int

proc toString*(self: Literal): string=
  case self.kind:
  of NUMBER_LIT: return $self.numVal
  of STRING_LIT: return self.strVal
  of BOOLEAN_LIT: return $self.boolVal
  of NIL_LIT: return "nil"

let DefaultLiteral* = Literal(kind: NIL_LIT, nilVal: 0)

type
  Token* = object
    token_type*: TokenType
    lexeme*: string
    literal*: Literal
    line*: int

proc newToken*(token_type: TokenType, line: int, lexeme: string = "", literal: Literal = Literal(kind: NIL_LIT, nilVal: 0)): Token=
  return Token(token_type: token_type, line: line, literal: literal, lexeme: lexeme)

let DefaultToken* = Token(token_type: NIL, line: 0)
