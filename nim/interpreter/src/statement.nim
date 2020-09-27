import token
import expression

type
  StmtKind* = enum
    BLOCKSTMT, EXPRSTMT, PRINTSTMT, VARSTMT
  Stmt* = ref object
    case kind*: StmtKind
    of BLOCKSTMT: statements*: seq[Stmt]
    of EXPRSTMT: exprStmt*: Expr
    of PRINTSTMT: printStmt*: Expr
    of VARSTMT: varStmt*: tuple[name: Token, value: Expr]

proc `$`*(stmt: Stmt): string=
  case stmt.kind:
  of BLOCKSTMT:
    for i, statement in stmt.statements: return $statement
  of EXPRSTMT: return $stmt.exprStmt
  of PRINTSTMT: return "print->" & $stmt.printStmt
  of VARSTMT: return "(" & $stmt.varStmt.name & ", " & $stmt.varStmt.value & ")"

proc printStmts*(stmts: seq[Stmt])=
  for i, stmt in stmts:
    echo $stmt
