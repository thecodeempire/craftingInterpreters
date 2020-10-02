import token
import strformat
import expression
import common_types

proc `$`*(stmt: Stmt): string

proc `$`*(stmts: seq[Stmt]): string=
  for i, statement in stmts: result &= $statement

proc `$`*(stmt: Stmt): string=
  case stmt.kind:
  of BLOCKSTMT:
    return fmt"""
    (BLOCKSTMT: {$stmt.statements})"""
  of EXPRSTMT: return fmt"""
    (EXPRSTMT: {$stmt.exprStmt})"""
  of PRINTSTMT: return fmt"""
    (PRINTSTMT: {$stmt.printStmt})"""
  of VARSTMT: return fmt"""
    (VARSTMT: {$stmt.varStmt})"""
  of IFSTMT: return fmt"""
    (IFSTMT: {$stmt.ifStmt.condition}, {$stmt.ifStmt.thenBranch}, {$stmt.ifStmt.elseBranch})"""
  of WHILESTMT: return fmt"""
    (WHILESTMT: {$stmt.whileStmt.condition}, {$stmt.whileStmt.body})"""
  of FUNCSTMT: return fmt"""
    (FUNCSTMT: {$stmt.funcStmt.name}, {$stmt.funcStmt.params}, ${stmt.funcStmt.body})"""
  of RETURNSTMT: return fmt"""
    (RETURNSTMT: {$stmt.returnStmt.keyword}, {stmt.returnStmt.value})"""

proc stmtIsNull*(self: Stmt): bool=
  return self.kind == BLOCKSTMT and self.statements.len() == 0

proc printStmts*(stmts: seq[Stmt])= echo $stmts
