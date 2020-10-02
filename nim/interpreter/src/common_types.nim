import options
import tables

# token.nim
type
  Interpreter* = object
    env*: Environment
    globals*: Environment

  LiteralKind* = enum
    NUMBER_LIT,
    STRING_LIT,
    BOOLEAN_LIT
    NIL_LIT,
    FUNC_LIT
  Literal* = object
    case kind*: LiteralKind
    of NUMBER_LIT: numVal*: float
    of STRING_LIT: strVal*: string
    of BOOLEAN_LIT: boolVal*: bool
    of NIL_LIT: nilVal*: int
    of FUNC_LIT: funcVal*: LoxCallable

  EnvTable* = Table[string, Literal]
  Environment* = ref object
    values*: EnvTable
    enclosing*: Option[Environment]

  CallFunc* = proc (i: Interpreter, arguments: seq[Literal]): Literal
  ArityFunc* = proc (): int
  LoxCallable* = ref object of RootObj
    arity*: ArityFunc
    call*: CallFunc
  LoxFunction* = ref object of LoxCallable
    declaration*: Stmt
    closure*: Environment

  StmtKind* = enum
    BLOCKSTMT, EXPRSTMT, IFSTMT, WHILESTMT, PRINTSTMT, VARSTMT,
    FUNCSTMT, RETURNSTMT
  Stmt* = ref object
    case kind*: StmtKind
    of BLOCKSTMT: statements*: seq[Stmt]
    of EXPRSTMT: exprStmt*: Expr
    of PRINTSTMT: printStmt*: Expr
    of VARSTMT: varStmt*: tuple[name: Token, value: Expr]
    of IFSTMT: ifStmt*: tuple[condition: Expr, thenBranch: Stmt, elseBranch: Stmt]
    of WHILESTMT: whileStmt*: tuple[condition: Expr, body: Stmt]
    of FUNCSTMT: funcStmt*: tuple[name: Token, params: seq[Token], body: seq[Stmt]]
    of RETURNSTMT: returnStmt*: tuple[keyword: Token, value: Expr]

  ExprKind* = enum
    TERNARYEXPR, BINARYEXPR, GROUPINGEXPR, NILEXPR, CALLEXPR,
    LITERALEXPR, UNARYEXPR, ASSIGNEXPR, VAREXPR, LOGICALEXPR
  Expr* = ref object
    case kind*: ExprKind
    of TERNARYEXPR: ternaryVal*: tuple[condition: Expr, first: Expr, second: Expr, token: Token]
    of BINARYEXPR: binaryVal*: tuple[first: Expr, operation: Token, second: Expr]
    of GROUPINGEXPR: groupingVal*: Expr
    of LITERALEXPR: literalVal*: Literal
    of UNARYEXPR: unaryVal*: tuple[operator: Token, expr: Expr]
    of ASSIGNEXPR: assignVal*: tuple[name: Token, value: Expr]
    of VAREXPR: variableVal*: Token
    of NILEXPR: nilVal*: int
    of LOGICALEXPR: logicalVal*: tuple[first: Expr, operator: Token, second: Expr]
    of CALLEXPR: callVal*: tuple[callee: Expr, paren: Token, arguments: seq[Expr]]

  TokenType* = enum
    # ------ SINGLE CHARACTER TOKENS ----------
    LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE, COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR, QUESTION, COLON
    BREAK, CONTINUE,
    # ------ ONE OR TWO CHARACTER TOKENS --------
    BANG, BANG_EQUAL, EQUAL, EQUAL_EQUAL, GREATER, GREATER_EQUAL, LESS, LESS_EQUAL,
    # ------ LITERALS ----------
    IDENTIFIER, STRING, NUMBER,
    # ------ KEYWORDS -------
    AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR, PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,
    # -------- OTHERS ---------
    EOF,
    # ------- PLACEHOLDER - Only for compiler purpose ------
    UNDEFINED
  Token* = object
    token_type*: TokenType
    lexeme*: string
    literal*: Literal
    line*: int

let defaultEnvValues* = initTable[string, Literal]()
let DefaultLiteral* = Literal(kind: NIL_LIT, nilVal: 0)
let DefaultToken* = Token(token_type: NIL, line: 0)

# errors.nim
type
  RunTimeError* = ref object of ValueError
    message*: string
    token*: Token

# expression.nim
let DefaultExpr*: Expr = Expr(kind: NILEXPR, nilVal: 0)

# parser.nim
type
  Parser* = object
    tokens*: seq[Token]
    current*: int

# return_stmt
type
  ReturnError* = ref object of RunTimeError
    value*: Literal

# scanner.nim
type
  Scanner* = object
    source*: string
    tokens*: seq[Token]
    start*: int
    current*: int
    line*: int

let DefaultStmt* = Stmt(kind: BLOCKSTMT, statements: @[])

