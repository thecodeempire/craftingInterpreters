import strformat
import environment
import expression
import errors
import statement
import token
import loxFunction
import times
import return_stmt
import common_types

func newLoxCallable*(arity: ArityFunc, call: CallFunc): LoxCallable=
  return LoxCallable(
    arity: arity,
    call: call
  )

proc newInterpreter*(): Interpreter=
  var globalEnv = newEnv()
  let arity = proc (): int= return 0
  let call = proc(inter: Interpreter, arguments: seq[Literal]): Literal=
    return Literal(kind: NUMBER_LIT, numVal: now().toTime().toUnix().float())
  globalEnv.define("clock",
    Literal(
      kind: FUNC_LIT, funcVal: newLoxCallable(arity, call)
    )
  )
  return Interpreter(env: newEnv(), globals: globalEnv)

proc visit*(self: Stmt, interpreter: var Interpreter)

proc interpret*(self: var Interpreter, statements: seq[Stmt])=
  try:
    for statement in statements:
      statement.visit(self)
  except:
    runtimeError(getCurrentExceptionMsg() )

proc setEnv*(self: var Interpreter, env: Environment) = self.env = env

proc getEnv*(self: Interpreter): Environment = return self.env

proc visit*(self: Expr, interpreter: var Interpreter): Literal

proc executeBlock*(
  self: Stmt,
  statements: seq[Stmt],
  environment: Environment,
  interpreter: var Interpreter
)=
  let previousEnv = interpreter.env
  try:
    interpreter.setEnv(environment)
    for statement in statements:
        statement.visit(interpreter)
  finally:
    interpreter.setEnv(previousEnv)
  
proc visitBlockStmt(
    self: Stmt,
    statements: seq[Stmt],
    interpreter: var Interpreter
  )=
  self.executeBlock(statements, newEnv(interpreter.env), interpreter)

proc visitExpressionStmt(self: Stmt, expr: Expr, interpreter: var Interpreter)=
  discard expr.visit(interpreter)

proc visitPrintStmt(self: Stmt, expr: Expr, interpreter: var Interpreter)=
  let value = expr.visit(interpreter)
  echo value.toString()

proc visitVarStmt(self: Stmt, name: Token, value: Expr, interpreter: var Interpreter)=
  var exprVal: Literal = DefaultLiteral
  if value.kind != NILEXPR:
    exprVal = value.visit(interpreter)
  interpreter.env.define(name.lexeme, exprVal)

proc visitIfStmt(self: Stmt, condition: Expr, thenBranch: Stmt, elseBranch: Stmt, interpreter: var Interpreter)=
  if condition.visit(interpreter).isTruthy(): thenBranch.visit(interpreter)
  elif not elseBranch.stmtIsNull(): elseBranch.visit(interpreter)

proc visitWhileStmt(self: Stmt, condition: Expr, whileBody: Stmt, interpreter: var Interpreter)=
  while condition.visit(interpreter).isTruthy():
    whileBody.visit(interpreter)

proc visitFuncStmt(self: Stmt, name: Token, params: seq[Token], body: seq[Stmt], interpreter: var Interpreter)=
  let funcVal = newLoxFunction(self, interpreter.env)
  interpreter.env.define(
    self.funcStmt.name.lexeme,
    Literal(kind: FUNC_LIT, funcVal)
  )

proc visitReturnStmt(self: Stmt, value: Expr, interpreter: var Interpreter)=
  var valueLit: Literal = DefaultLiteral
  if value.kind != NILEXPR:
    valueLit = value.visit(interpreter)
  raise newReturnError(valueLit)

proc visit*(self: Stmt, interpreter: var Interpreter)=
  case self.kind:
  of BLOCKSTMT: self.visitBlockStmt(self.statements, interpreter)
  of EXPRSTMT: self.visitExpressionStmt(self.exprStmt, interpreter)
  of PRINTSTMT: self.visitPrintStmt(self.printStmt, interpreter)
  of VARSTMT: self.visitVarStmt(self.varStmt.name, self.varStmt.value, interpreter)
  of IFSTMT: self.visitIfStmt(self.ifStmt.condition, self.ifStmt.thenBranch, self.ifStmt.elseBranch, interpreter)
  of WHILESTMT: self.visitWhileStmt(self.whileStmt.condition, self.whileStmt.body, interpreter)
  of FUNCSTMT: self.visitFuncStmt(self.funcStmt.name, self.funcStmt.params, self.funcStmt.body, interpreter)
  of RETURNSTMT: self.visitReturnStmt(self.returnStmt.value, interpreter)

proc visitTernary(
    self: Expr,
    condition: Expr,
    first: Expr,
    second: Expr,
    token: Token,
    interpreter: var Interpreter
): Literal=
  let conditionLit = condition.visit(interpreter)
  let firstLit = first.visit(interpreter)
  let secondLit = second.visit(interpreter)
  
  if conditionLit.kind == BOOLEAN_LIT:
    if conditionLit.boolVal: return firstLit
    else: return secondLit
  elif conditionLit.kind == NIL_LIT: return secondLit
  else:
    runtimeError("Ternary operator failed. ", token)
    return Literal(kind: NIL_LIT, nilVal: 0)

proc visitBinary(self: Expr, first: Expr,
    operation: Token, second: Expr,
    interpreter: var Interpreter
  ): Literal=
  let left = first.visit(interpreter)
  let right = second.visit(interpreter)
  
  case operation.token_type:
  of PLUS:
    if left.kind == NUMBER_LIT and right.kind == NUMBER_LIT:
      return Literal(kind: NUMBER_LIT, numVal: left.numVal + right.numVal)
    if left.kind == STRING_LIT:
      case right.kind:
      of STRING_LIT: return Literal(kind: STRING_LIT, strVal: left.strVal & right.strVal)
      of NUMBER_LIT: return Literal(kind: STRING_LIT, strVal: left.strVal & $right.numVal)
      of BOOLEAN_LIT: return Literal(kind: STRING_LIT, strVal: left.strVal & $right.boolVal)
      of NIL_LIT, FUNC_LIT:
        return errorLit(operation, "Mismatched types. Cannot add the two operands")
  of MINUS:
    if left.kind == NUMBER_LIT and right.kind == NUMBER_LIT:
      return Literal(kind: NUMBER_LIT, numVal: left.numVal - right.numVal)
    return errorLit(operation, "Mismatched types. Cannot subtract the two operands")
  of SLASH:
    if left.kind == NUMBER_LIT and right.kind == NUMBER_LIT:
      if right.numVal == 0.0:
        return errorLit(operation, "Divide by zero error. The denominator is equal to zero!")
      return Literal(kind: NUMBER_LIT, numVal: left.numVal / right.numVal)
    return errorLit(operation, "Mismatched types. Cannot divide the two operands")
  of STAR:
    if left.kind == NUMBER_LIT and right.kind == NUMBER_LIT:
      return Literal(kind: NUMBER_LIT, numVal: left.numVal * right.numVal)
    return errorLit(operation, "Mismatched types. Cannot multiple the two operands")
  of GREATER:
    if left.kind == NUMBER_LIT and right.kind == NUMBER_LIT:
      return Literal(kind: BOOLEAN_LIT, boolVal: left.numVal > right.numVal)
    if left.kind == STRING_LIT and right.kind == STRING_LIT:
      return Literal(kind: BOOLEAN_LIT, boolVal: left.strVal > right.strVal)
    return errorLit(operation, "Mismatched types. Cannot compare the two operands")
  of GREATER_EQUAL:
    if left.kind == NUMBER_LIT and right.kind == NUMBER_LIT:
      return Literal(kind: BOOLEAN_LIT, boolVal: left.numVal >= right.numVal)
    if left.kind == STRING_LIT and right.kind == STRING_LIT:
      return Literal(kind: BOOLEAN_LIT, boolVal: left.strVal >= right.strVal)
    return errorLit(operation, "Mismatched types. Cannot compare the two operands")
  of LESS:
    if left.kind == NUMBER_LIT and right.kind == NUMBER_LIT:
      return Literal(kind: BOOLEAN_LIT, boolVal: left.numVal < right.numVal)
    if left.kind == STRING_LIT and right.kind == STRING_LIT:
      return Literal(kind: BOOLEAN_LIT, boolVal: left.strVal <= right.strVal)
    return errorLit(operation, "Mismatched types. Cannot compare the two operands")
  of LESS_EQUAL:
    if left.kind == NUMBER_LIT and right.kind == NUMBER_LIT:
      return Literal(kind: BOOLEAN_LIT, boolVal: left.numVal < right.numVal)
    if left.kind == STRING_LIT and right.kind == STRING_LIT:
      return Literal(kind: BOOLEAN_LIT, boolVal: left.strVal <= right.strVal)
    return errorLit(operation, "Mismatched types. Cannot compare the two operands")
  of BANG_EQUAL:
    return Literal(kind: BOOLEAN_LIT, boolVal: is_equal(left, right, operation))
  of EQUAL_EQUAL:
    return Literal(kind: BOOLEAN_LIT, boolVal: is_equal(left, right, operation))
  else:
    return errorLit(operation, "Cannot fathom the binary operation")

proc visitLiteral(self: Expr, val: Literal): Literal=
  return self.literalVal

proc visitUnary(self: Expr, operator: Token, expr: Expr, interpreter: var Interpreter): Literal=
  let right = expr.visit(interpreter)
  proc is_truthy(val: Literal): bool=
    case val.kind:
    of BOOLEAN_LIT: return val.boolVal
    of NIL_LIT: return false
    else: return true

  case operator.token_type:
  of BANG: return Literal(kind: BOOLEAN_LIT, boolVal: right.isTruthy())
  of MINUS:
    if check_num_operand(right, operator).kind == NUMBER_LIT:
      return Literal(kind: NUMBER_LIT, numVal: -right.numVal)
    return errorLit(operator, "Mismatched unary operation. Cannot negatify random Operation!")
  else: return errorLit(operator, "Mismatched unary operation. Check the operand!")

proc visitAssign(self: Expr, name: Token, value: Expr, interpreter: var Interpreter): Literal=
  var tempVal = DefaultLiteral
  if value != DefaultExpr:
    tempVal = value.visit(interpreter)
  interpreter.env.assign(name, tempVal)
  return tempVal

proc visitVariable(self: Expr, variable: Token, interpreter: var Interpreter): Literal=
  return interpreter.env.get(variable)

proc visitLogic(self: Expr, first: Expr, operator: Token, second: Expr, interpreter: var Interpreter): Literal=
  let left = first.visit(interpreter)
  if operator.token_type == OR:
    if left.isTruthy(): return left
  else:
    if not left.isTruthy(): return left
  return second.visit(interpreter)

proc visitCallExpr(self: Expr,
  callee: Expr,
  paren: Token,
  arguments: seq[Expr],
  interpreter: var Interpreter
): Literal=
  let calleeLit = callee.visit(interpreter)
  let args = newSeq[Literal]()

  for arg in arguments:
    args.add(arg.visit(interpreter))

  if not callee of LoxCallable:
    runtimeError(paren, "Can only call functoins and classes.")
  let function = callee
  if args.len() != function.arity():
    runtimeError(paren, fmt"""Expected {function.arity()} arguments but got {args.len()} arguments.""")
  return function.call(self, arguments)


proc visit*(self: Expr, interpreter: var Interpreter): Literal=
  case self.kind:
  of TERNARYEXPR: return self.visitTernary(
    self.ternaryVal.condition, self.ternaryVal.first,
    self.ternaryVal.second, self.ternaryVal.token, interpreter
  )
  of BINARYEXPR: return self.visitBinary(
    self.binaryVal.first, self.binaryVal.operation,
    self.binaryVal.second, interpreter
  )
  of GROUPINGEXPR: return self.visit(interpreter)
  of LITERALEXPR: return  self.visitLiteral(self.literalVal)
  of UNARYEXPR: return self.visitUnary(self.unaryVal.operator, self.unaryVal.expr, interpreter)
  of ASSIGNEXPR: return self.visitAssign(self.assignVal.name, self.assignVal.value, interpreter)
  of VAREXPR: return self.visitVariable(self.variableVal, interpreter)
  of LOGICALEXPR: return self.visitLogic(self.logicalVal.first, self.logicalVal.operator, self.logicalVal.second, interpreter)
  of CALLEXPR: return self.visitCallExpr(self.callVal.callee, self.callVal.paren, self.callVal.arguments, interpreter)
  of NILEXPR: return DefaultLiteral
