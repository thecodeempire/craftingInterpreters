import token
import strformat
import errors
import common_types

proc `$`*(expr: Expr): string

proc `$`(seqExpr: seq[Expr]): string=
  var str = ""
  for i, exp in seqExpr:
    str &= $exp
  return str

proc `$`*(expr: Expr): string=
  case expr.kind:
  of TERNARYEXPR: return fmt"""
    (TERNARYEXPR, {$expr.ternaryVal.condition}, {$expr.ternaryVal.first}, {$expr.ternaryVal.second}, {$expr.ternaryVal.token}"""
  of BINARYEXPR: return fmt"""
    (BINARYEXPR, {$expr.binaryVal.first}, {$expr.binaryVal.operation}, {$expr.binaryVal.second})"""
  of GROUPINGEXPR: return fmt"""
    (GROUPINGEXPR, {$expr.groupingVal})"""
  of LITERALEXPR: return fmt"""
    (LITERALEXPR, {$expr.literalVal})"""
  of UNARYEXPR: return fmt"""
    (UNARYEXPR, {$expr.unaryVal.operator}, {$expr.unaryVal.expr})"""
  of ASSIGNEXPR: return fmt"""
    (ASSIGNEXPR, {$expr.assignVal.name}, {$expr.assignVal.value})"""
  of VAREXPR: return fmt"""
    (VAREXPR, {$expr.variableVal})"""
  of NILEXPR: return $expr.nilVal
  of LOGICALEXPR: return fmt"""
    (LOGICALEXPR, {$expr.logicalVal.first}, {$expr.logicalVal.operator}, {$expr.logicalVal.second})"""
  of CALLEXPR: return fmt"""
    (CALLEXPR, {$expr.callVal.callee}, {$expr.callVal.paren}, {$expr.callVal.arguments})"""

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


