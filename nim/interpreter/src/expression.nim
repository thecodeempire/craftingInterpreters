import token
import strformat
import errors

type
  ExprKind* = enum
    TERNARYEXPR, BINARYEXPR, GROUPINGEXPR, NILEXPR,
    LITERALEXPR, UNARYEXPR, ASSIGNEXPR, VAREXPR
  Expr* = ref object
    case kind*: ExprKind
    of TERNARYEXPR: ternaryVal*: tuple[condition: Expr, first: Expr, second: Expr, token: Token]
    of BINARYEXPR: binaryVal*: tuple[first: Expr, operation: Token, second: Expr]
    of GROUPINGEXPR: groupingVal*: Expr
    of LITERALEXPR: literalVal*: Literal
    of UNARYEXPR: unaryVal*: tuple[operator: Token, expr: Expr]
    of ASSIGNEXPR: assignVal*: tuple[name: Token, value: Expr]
    of VAREXPR: variableVal*: Token
    of NILEXPR: nilVal*: int

proc `$`*(expr: Expr): string=
  case expr.kind:
  of TERNARYEXPR: return fmt"""({$expr.ternaryVal.condition}, {$expr.ternaryVal.first}, {$expr.ternaryVal.second}, {$expr.ternaryVal.token}"""
  of BINARYEXPR: return fmt"""({$expr.binaryVal.first}, {$expr.binaryVal.operation}, {$expr.binaryVal.second})"""
  of GROUPINGEXPR: return $expr.groupingVal
  of LITERALEXPR: return $expr.literalVal
  of UNARYEXPR: return fmt"""({$expr.unaryVal.operator}, {$expr.unaryVal.expr})"""
  of ASSIGNEXPR: return fmt"""({$expr.assignVal.name}, {$expr.assignVal.value})"""
  of VAREXPR: return $expr.variableVal
  of NILEXPR: return $expr.nilVal


let DefaultExpr*: Expr = Expr(kind: NILEXPR, nilVal: 0)


proc errorLit*(operation: Token, message: string): Literal =
  compileTimeError(operation, message)
  return Literal(kind: NIL_LIT, nilVal: 0)

proc is_equal*(left: Literal, right: Literal, operation: Token): bool =
  if left.kind == right.kind:
    if left.kind == NIL_LIT and right.kind == NIL_LIT: return true
    if left.kind == STRING_LIT and right.kind == STRING_LIT:
      return left.strVal == right.strVal
    if left.kind == NUMBER_LIT and right.kind == NUMBER_LIT:
      return left.numVal == right.numVal
    if left.kind == BOOLEAN_LIT and right.kind == BOOLEAN_LIT:
      return left.boolVal == right.boolVal
  compileTimeError(operation, "Mismatched types, cannot compare the two operands")
  return false

proc check_num_operand*(right: Literal, operation: Token): Literal=
  if right.kind == NUMBER_LIT:
    return right
  return errorLit(operation, "Mismatched unary operation. Cannot perform operation on the following")


