import tables
import errors
import options
import common_types

proc newEnv*(): Environment=
  return Environment(values: defaultEnvValues, enclosing: none(Environment))

proc newEnv*(enclosing: Environment): Environment=
  return Environment(values: defaultEnvValues, enclosing: some(enclosing))

proc get*(self: Environment, name: Token): Literal
  {.raises: [IOError, RunTimeError, KeyError, UnpackError]}=
  if self.values.hasKey(name.lexeme):
    return self.values[name.lexeme]
  if self.enclosing.isSome:
    let lit = self.enclosing.get().get(name)
    return lit
  runtimeError("Undefined variable ", name)

proc define*(self: var Environment, name: string, value: Literal)=
  self.values[name] = value

proc assign*(self: var Environment, name: Token, value: Literal)=
  if self.values.hasKey(name.lexeme):
    self.values[name.lexeme] = value
  else:
    if self.enclosing.isSome:
      self.enclosing.get().assign(name, value)
    else: runTimeError("Undefined variable", name)

