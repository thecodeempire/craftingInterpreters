# Crafting Interpreters

## [Crafting Interpreters Site](https://craftinginterpreters.com/)

---

This repo contains the code used while learning how to build an interpreter from the materials in https://craftinginterpreters.com/.

The code-base is in [Java](https://java.com), [C](https://cprogramming.com), [Rust](https://rust-lang.org), [Nim](https://nim-lang.org) and [Haskell](https://haskell.org).

---

The Lox Grammar:

```
program          -> declaration* EOF;
declaration      -> funDecl | varDecl | statement;
funDecl          -> "fun" function ;
function         -> IDENTIFIER "(" parameters? ")" block ;
parameters       -> IDENTIFIER ( "," IDENTIFIER )* ;
statement        -> exprStmt | ifStmt | forStmt | printStmt | whileStmt | block | returnStmt;
returnStmt       -> "return" expression? ";" ;
ifStmt           -> "if" "(" expression ")" statement ( "else" statement )?;
forStmt          -> "for" "(" ( varDecl | exprStmt | ";") expression? ";" expression? ")" statement ;
whileStmt        -> "while" "(" expression ")" statement ;
printStmt        -> "print" expression ";"
block            -> "{" declaration "}"
varDecl          -> "var" IDENTIFIER ("=" expression)? ";"
exprStmt         -> expression ";"
expression       -> assignment ;
assignment       -> (IDENTIFIER "=" assignment) | comma
comma            -> ternary "," ternary
ternary          -> logic_or ("?" ternary ":" ternary)?
logic_or         -> logic_and ( "or" logic_and )* ;
logic_and        -> equality ( "and" equality )* ;
equality         -> comparison (("!=" | "==") comparison)*
comparison       -> addition ((">" | ">=" | "<" | "<=") addition)*;
addition         -> multiplication (("-" | "+") multiplication)*;
multiplication   -> unary (("/" | "*") unary)*;
unary            -> ("!" | "-") unary | primary;
call             -> primary ( "(" arguments? ")" )* ;
arguments        -> expression ( "," expression )* ;
primary          -> NUMBER | STRING | IDENTIFIER | "false" | "true" | "nil" | "("expression")";
```

---

## Running the code - Java

```rust
fn println()  {
println!("Hello World");
}
```

```sh
cd java/<chapter_name>
nodemon
```

### Running the code - C

```sh
cd c/<chapter_name>
nodemon
```

### Running the code - Rust

```sh
cd rust/<chapter_name>
nodemon
```

### Running the code - Nim
```sh
cd nim/<chapter_name>
nimble run <chapter_name> sample.lox
```

### Running the code - Haskell

```sh
cd haskell/<chapter_name>
nodemon
```

---

## Happy Writing Interpreters!!
