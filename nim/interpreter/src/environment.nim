import tables
import errors
import token
import options

type EnvTable = Table[string, Literal]

type
  Environment* = ref object
    values: EnvTable
    enclosing: Option[Environment]

proc newEnv*(): Environment=
  return Environment(
    values: initTable[string, Literal](), enclosing: none(Environment)
  )
proc newEnv*(enclosing: Environment): Environment=
  return Environment(
    values: initTable[string, Literal](),
    enclosing: some(enclosing)
  )

proc get*(self: Environment, name: Token): Literal
  {.raises: [IOError, RunTimeError, KeyError, UnpackError]}=
  if self.values.hasKey(name.lexeme): return self.values[name.lexeme]
  if self.enclosing.isSome: return self.enclosing.get().get(name)
  runtimeError("Undefined variable ", name)

proc define*(self: var Environment, name: string, value: Literal)=
  self.values[name] = value

proc assign*(self: Environment, name: Token, value: Literal)=
  if self.values.hasKey(name.lexeme):
    self.values[name.lexeme] = value
  else:
    if self.enclosing.isSome:
      self.enclosing.get().assign(name, value)
    else: runTimeError("Undefined variable", name)

