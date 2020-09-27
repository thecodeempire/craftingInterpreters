import token

# mutable variables
var hadCompileTimeError* = false
var hadRunTimeError* = false

type
  RunTimeError* = ref object of ValueError
    message*: string
    token*: Token

proc report(line: int, where, message: string)=
  stderr.write "[line" & $line & "] Error" & where & ": " & message & "\n"
  hadCompileTimeError = true

proc runtimeError*(message: string, token: Token = DefaultToken)
  {.raises: [RunTimeError, IOError].} =
  stderr.write(message & "\n[line " & $token.line & "]" & " {at: " & $token.lexeme & "\n"  & "}")
  hadRunTimeError = true
  raise RunTimeError(message: message, token: token)

proc compileTimeError*(currToken: Token, message: string)=
  if currToken.token_type == EOF:
    report(currToken.line , " at end ", message)
  else:
    report(currToken.line, " at '" & currToken.lexeme & "'" , message)
