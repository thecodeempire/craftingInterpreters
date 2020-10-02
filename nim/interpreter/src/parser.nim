import statement
import errors
import common_types

proc isAtEnd(self: Parser): bool
proc declaration(self: var Parser): Stmt
proc parseStatement(self: var Parser): Stmt
proc ifStatement(self: var Parser): Stmt
proc forStatement(self: var Parser): Stmt
proc whileStatement(self: var Parser): Stmt
proc varDeclaration(self: var Parser): Stmt
proc printStatement(self: var Parser): Stmt
proc expressionStatement(self: var Parser): Stmt
proc functionStatement(self: var Parser, kind: string): Stmt
proc blockStmt(self: var Parser): seq[Stmt]
proc assignment(self: var Parser): Expr
proc parseExpression(self: var Parser): Expr
proc comma(self: var Parser): Expr
proc ternary(self: var Parser): Expr
proc equality(self: var Parser): Expr
proc comparison(self: var Parser): Expr
proc addition(self: var Parser): Expr
proc multiplication(self: var Parser): Expr
proc peek(self: Parser): Token
proc previous(self: Parser): Token
proc orExpr(self: var Parser): Expr
proc andExpr(self: var Parser): Expr
proc check(self: Parser, token_type: TokenType): bool
proc advance(self: var Parser): Token
proc match(self: var Parser, types: varargs[TokenType]): bool
proc consume(self: var Parser, token_type: TokenType, message: string): Token
proc error(self: Parser, token: Token, message: string)
proc synchronize(self: var Parser)
proc primary(self: var Parser): Expr
proc unary(self: var Parser): Expr

proc newParser*(tokens: seq[Token]): Parser=
  return Parser(tokens: tokens, current: 0)

proc parse*(self: var Parser): seq[Stmt]=
  var statements = newSeq[Stmt](0)
  while not self.isAtEnd():
    let decl = self.declaration()
    statements.add(decl)
  return statements

proc declaration(self: var Parser): Stmt=
  try:
    if self.match(FUN): return self.functionStatement("function")
    if self.match(VAR): return self.varDeclaration()
    return self.parseStatement()
  except:
    self.synchronize()

proc functionStatement(self: var Parser, kind: string): Stmt=
  let name = self.consume(IDENTIFIER, "Expect " & kind & " name.")
  discard self.consume(LEFT_PAREN, "Expect '(' after " & kind & " name.")
  var parameters = newSeq[Token]()

  if not self.check(RIGHT_PAREN):
    while true:
      if parameters.len() >= 255:
        self.error(self.peek(), "Cannot have more than 255 parameters")
      parameters.add(self.consume(IDENTIFIER, "Expect parameter name"))
      if not self.match(COMMA): break

  discard self.consume(RIGHT_PAREN, "Expect ')' after parameters")
  discard self.consume(LEFT_BRACE, "Expect '{' before " & kind & " body.")
  let body = self.blockStmt()
  return Stmt(kind: FUNCSTMT, funcStmt: (name, parameters, body))

proc parseStatement(self: var Parser): Stmt=
  if self.match IF: return self.ifStatement()
  if self.match FOR: return self.forStatement()
  if self.match WHILE: return self.whileStatement()
  if self.match PRINT: return self.printStatement()
  if self.match LEFT_BRACE: return Stmt(kind: BLOCKSTMT, statements: self.blockStmt())
  return self.expressionStatement()

proc varDeclaration(self: var Parser):Stmt=
  let name = self.consume(IDENTIFIER, "Expect variable name")
  var initializer = DefaultExpr
  if self.match EQUAL:
    initializer = self.parseExpression()
  discard self.consume(SEMICOLON, "Expect ';' after variable declaration")
  return Stmt(kind: VARSTMT, varStmt: (name, initializer))

proc whileStatement(self: var Parser): Stmt=
  discard self.consume(LEFT_PAREN, "Expect '(' after 'while'.")
  let condition = self.parseExpression()
  discard self.consume(RIGHT_PAREN, "Expect ')' after condition.")
  let body = self.parseStatement()
  return Stmt(kind: WHILESTMT, whileStmt: (condition, body))

proc ifStatement(self: var Parser): Stmt=
  discard self.consume(LEFT_PAREN, "Expect '(' after 'if'.")
  let condition = self.parseExpression()

  discard self.consume(RIGHT_PAREN, "Expect ')' after if condition.")
  let thenBranch = self.parseStatement()

  var elseBranch = DefaultStmt
  if self.match(ELSE):
    elseBranch = self.parseStatement()
  return Stmt(kind: IFSTMT, ifStmt: (condition, thenBranch, elseBranch))
  
proc forStatement(self: var Parser): Stmt=
  discard self.consume(LEFT_PAREN, "Expect '(' after 'for'.")
  var initializer = DefaultStmt

  if self.match(SEMICOLON): initializer = DefaultStmt
  elif self.match(VAR): initializer = self.varDeclaration()
  else: initializer = self.expressionStatement()

  var condition = DefaultExpr
  if not self.check(SEMICOLON): condition = self.parseExpression()
  discard self.consume(SEMICOLON, "Expect ';' after loop condition.")

  var increment = DefaultExpr
  if not self.check(RIGHT_PAREN): increment = self.parseExpression()
  discard self.consume(RIGHT_PAREN, "Expect ')' after for clauses")

  var body = self.parseStatement()
  if increment != DefaultExpr:
    body = Stmt(kind: BLOCKSTMT, statements: @[body, Stmt(kind: EXPRSTMT, exprStmt: increment)])

  if condition.kind == NILEXPR: condition = Expr(kind: LITERALEXPR, literalVal: Literal(kind: BOOLEAN_LIT, boolVal: true))
  body = Stmt(kind: WHILESTMT, whileStmt: (condition, body))

  if initializer.stmtIsNull:
    body = Stmt(kind: BLOCKSTMT, statements: @[initializer, body])

  return body

proc printStatement(self: var Parser): Stmt=
  let value = self.parseExpression()
  discard self.consume(SEMICOLON, "Expect ';' after expression.")
  return Stmt(kind: PRINTSTMT, printStmt: value)

proc expressionStatement(self: var Parser): Stmt=
  let expr = self.parseExpression()
  discard self.consume(SEMICOLON, "Expect ';' after expression")
  return Stmt(kind: EXPRSTMT, exprStmt: expr)

proc blockStmt(self: var Parser): seq[Stmt]=
  var statements = newSeq[Stmt]()
  while not self.check(RIGHT_BRACE) and not self.isAtEnd():
    statements.add(self.declaration())
  discard self.consume(RIGHT_BRACE, "Expect '}' after block")
  return statements

proc assignment(self: var Parser): Expr=
  let expr = self.comma()
  if self.match EQUAL:
    let equals = self.previous()
    let value = self.assignment()

    if expr.kind == VAREXPR:
      let name = expr.variableVal
      return Expr(kind: ASSIGNEXPR, assignVal: (name, value))
    self.error(equals, "Invalid assignment target.")

  return expr

proc comma(self: var Parser): Expr=
  var expr = self.ternary()
  while self.match(COMMA):
    let operator = self.previous()
    let right = self.ternary()
    expr = Expr(kind: BINARYEXPR, binaryVal: (expr, operator, right))
  return expr

proc ternary(self: var Parser): Expr=
  var expr = self.orExpr()
  if self.match(QUESTION):
    let first = self.ternary()
    if self.match(COLON):
      let second = self.ternary()
      expr = Expr(kind: TERNARYEXPR, ternaryVal: (expr, first, second, self.previous()))
  return expr

proc orExpr(self: var Parser): Expr=
  var expr = self.andExpr()
  while self.match(OR):
    let operator = self.previous()
    let right = self.andExpr()
    expr = Expr(kind: LOGICALEXPR, logicalVal: (expr, operator, right))
  return expr
 
proc andExpr(self: var Parser): Expr=
  var expr = self.equality()
  while self.match(AND):
    let  operator = self.previous()
    let right = self.equality()
    expr = Expr(kind: LOGICALEXPR, logicalVal: (expr, operator, right))
  return expr

proc parseExpression(self: var Parser): Expr=
  return self.assignment()

proc equality(self: var Parser): Expr=
  var expr = self.comparison()
  while self.match(BANG_EQUAL, EQUAL_EQUAL):
    let operator = self.previous()
    let right = self.comparison()
    expr = Expr(kind: BINARYEXPR, binaryVal: (expr, operator, right))
  return expr

proc comparison(self: var Parser): Expr=
  var expr = self.addition()
  while self.match(GREATER, GREATER_EQUAL, LESS, LESS_EQUAL):
    let operator = self.previous()
    let right = self.addition()
    expr = Expr(kind: BINARYEXPR, binaryVal: (expr, operator, right))
  return expr
  
proc addition(self: var Parser): Expr=
  var expr = self.multiplication()
  while self.match(MINUS, PLUS):
    let operator = self.previous()
    let right = self.multiplication()
    expr = Expr(kind: BINARYEXPR, binaryVal: (expr, operator, right))
  return expr
  
proc multiplication(self: var Parser): Expr=
  var expr = self.unary()
  while self.match(SLASH, STAR):
    let operator = self.previous()
    let right = self.unary()
    expr = Expr(kind: BINARYEXPR, binaryVal: (expr, operator, right))
  return expr

proc unary(self: var Parser): Expr=
  if self.match(BANG, MINUS):
    let operator = self.previous()
    let right = self.unary()
    return Expr(kind: UNARYEXPR, unaryVal: (operator, right))
  return self.primary()

proc primary(self: var Parser): Expr=
  if self.match FALSE: return Expr(kind: LITERALEXPR, literalVal: Literal(kind: BOOLEAN_LIT, boolVal: false))
  if self.match TRUE: return Expr(kind: LITERALEXPR, literalVal: Literal(kind: BOOLEAN_LIT, boolVal: true))
  if self.match NIL: return Expr(kind: LITERALEXPR, literalVal: DefaultLiteral)
  if self.match(NUMBER, STRING): return Expr(kind: LITERALEXPR, literalVal: self.previous().literal)
  if self.match IDENTIFIER: return Expr(kind: VAREXPR, variableVal: self.previous())
  if self.match LEFT_PAREN:
    let expr = self.parseExpression()
    discard self.consume(RIGHT_PAREN, "Expect ) after expression")
    return Expr(kind: GROUPINGEXPR, groupingVal: expr)

  self.error(self.peek(), "Expect expression..")
  return DefaultExpr

proc peek(self: Parser): Token=
  return self.tokens[self.current]

proc previous(self: Parser): Token=
  return self.tokens[self.current - 1]

proc check(self: Parser, token_type: TokenType): bool=
  if self.isAtEnd(): return false
  else: self.peek().token_type == token_type

proc advance(self: var Parser): Token=
  if not self.isAtEnd(): self.current += 1
  return self.previous()

proc match(self: var Parser, types: varargs[TokenType]): bool=
  for token_type in types:
    if self.check token_type:
      self.current += 1
      return true
  return false

proc consume(self: var Parser, token_type: TokenType, message: string): Token=
  if self.check token_type:
    return self.advance()

  self.error(self.peek(), message)
  return DefaultToken

proc error(self: Parser, token: Token, message: string)=
  runtimeError(message, token)

proc synchronize(self: var Parser)=
  discard self.advance()

  while not self.isAtEnd():
    if self.previous().token_type == SEMICOLON:
      return

    case self.peek().token_type:
    of CLASS, FUN, VAR, FOR, IF, WHILE, PRINT, RETURN:
      return
    else: break

    discard self.advance()

proc isAtEnd(self: Parser): bool=
  return self.peek().token_type == EOF

