import os
import scanner
import rdstdin
import errors
import parser
import intrptr

# any function or proc that writes `arg: ArgType` gets the arg as immutable
proc run(source: string)=
  var scanner = newScanner(source)
  let tokens = scanner.scanTokens()

  #for i, token in tokens:
  #  echo token

  var parser = newParser(tokens)
  let statements = parser.parse()

#  for i, statement in statements:
#    echo $statement

  if hadCompileTimeError and hadRunTimeError: return

  var interpreter = newInterpreter()
  interpreter.interpret(statements)

proc runFile(path: string)=
  let source = readFile(path)
  run(source)
  if hadCompileTimeError:
    quit(65)
  if hadRunTimeError:
    quit(70)

proc runPrompt()=
  while true:
    let input = readLineFromStdin("|>|> ")
    if input.len() == 0:
      continue
    run(input)
    hadCompileTimeError = false
  

# Compile time if!
when isMainModule:
  if paramCount() > 1:
    echo "Usage nimlox [script]"
    quit 64
  elif paramCount() == 1:
    runFile(paramStr(1))
  else:
    runPrompt()
