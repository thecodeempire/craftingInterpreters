import strformat
import common_types

proc toString*(self: Literal): string=
  case self.kind:
  of NUMBER_LIT: return $self.numVal
  of STRING_LIT: return self.strVal
  of BOOLEAN_LIT: return $self.boolVal
  of NIL_LIT: return "nil"
  of FUNC_LIT: return "<func>"

proc `$`*(self: Literal): string = return self.toString()

proc newToken*(token_type: TokenType, line: int, lexeme: string = "", literal: Literal = Literal(kind: NIL_LIT, nilVal: 0)): Token=
  return Token(token_type: token_type, line: line, literal: literal, lexeme: lexeme)

proc `$`*(tok: Token): string=
  return fmt"""(TOKEN: {$tok.token_type}, {tok.lexeme}, {$tok.literal}, {$tok.line})"""

proc `$`*(self: seq[Token]): string=
  for i, tok in self: result &= $tok

proc isTruthy*(self: Literal): bool=
  case self.kind
  of NUMBER_LIT: return self.numVal != 0
  of STRING_LIT: return true
  of BOOLEAN_LIT: return self.boolVal
  of NIL_LIT: return false
  of FUNC_LIT: return true

