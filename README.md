# Crafting Interpreters

## [Crafting Interpreters Site](https://craftinginterpreters.com/)

---

This repo contains the code used while learning how to build an interpreter from the materials in https://craftinginterpreters.com/.

The code-base is in [Java](https://java.com), [C](https://cprogramming.com), [Rust](https://rust-lang.org) and [Haskell](https://haskell.org).

---

The Lox Grammar:

```
program          -> declaration* EOF
declaration      -> varDecl | statement
statement        -> exprStmt | printStmt | block
exprStmt         -> expression ";"
printStmt        -> "print" expression ";"
block            -> "{" declaration "}"
varDecl          -> "var" IDENTIFIER ("=" expression)? ";"
expression       -> assignment
assignment       -> (IDENTIFIER "=" assignment) | comma
comma            -> ternary "," ternary
ternary          -> equality ("?" ternary ":" ternary)?
equality         -> comparison (("!=" | "==") comparison)*
comparison       -> addition ((">" | ">=" | "<" | "<=") addition)*;
addition         -> multiplication (("-" | "+") multiplication)*;
multiplication   -> unary (("/" | "*") unary)*;
unary            -> ("!" | "-") unary
				   | primary;
primary          -> NUMBER | STRING | IDENTIFIER | "false" | "true" | "nil"
				   | "("expression")";

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

### Running the code - Haskell

```sh
cd haskell/<chapter_name>
nodemon
```

---

## Happy Writing Interpreters!!
