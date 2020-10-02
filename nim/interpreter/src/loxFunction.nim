import statement
import environment
import token
import intrptr
import return_stmt
import common_types

proc newLoxFunction*(declaration: Stmt, closure: Environment): LoxFunction=
  return LoxFunction(declaration: declaration, closure: closure)

proc call*(self: LoxFunction, interpreter: var Interpreter, arguments: seq[Literal]): Literal=
  var env = newEnv(self.closure)
  for i, param in self.declaration.funcStmt.params:
    env.define(param.lexeme, arguments[i])
  try:
    self.declaration.executeBlock(self.declaration.funcStmt.body, env, interpreter)
  except ReturnError as e:
    return e.value
  return DefaultLiteral

proc arity*(self: LoxFunction): int=
  return self.declaration.funcStmt.params.len()

proc `$`*(self: LoxFunction): string=
  return "<fn " & self.declaration.funcStmt.name.lexeme & " >"

