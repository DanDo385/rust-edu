# Project 36 - A Tree-Walking Interpreter

## What You Will Build

You will build a miniature language engine for arithmetic expressions such as:

- `2 + 3 * 4`
- `(1 + 2) * 3`
- `10 * (1 / (2 - 2))`

The interpreter pipeline has three stages:

1. Lexer: raw text -> token stream
2. Parser: token stream -> abstract syntax tree (AST)
3. Evaluator: AST -> numeric result (`f64`)

This is the same architecture used in real compilers and interpreters, scaled down to a focused arithmetic grammar.

## Why This Lab Matters (First Principles)

This lab teaches computer science fundamentals directly:

- Parsing is structured control flow over symbols.
- Trees are recursive data structures that encode meaning, not just data.
- Evaluation is recursive problem decomposition.
- Error handling is part of the language design, not an afterthought.

You are not just “calculating math.” You are implementing a deterministic execution model.

## Memory Model and Ownership (Mandatory)

### High-level ownership flow

```text
input &str (borrowed from caller)
        |
        v
tokenize(input) -> Vec<Token> (owned)
        |
        v
parse(tokens) -> Expr AST (owned)
        |
        v
evaluate(&ast) -> f64 (Copy) or EvalError
```

### Stack vs heap in this lab

- Stack:
  - references (`&str`, `&Expr`)
  - scalar values (`f64`, enum tags, indices)
  - smart pointers (`Box<Expr>` handles)
- Heap:
  - `Vec<Token>` token buffer
  - recursive AST nodes behind `Box<Expr>`
  - owned `String` values used by error messages

### Concrete AST memory sketch

Expression: `(2 + 3) * 4`

```text
Stack:
  ast: Expr::Binary { op: Multiply, left: Box<Expr>, right: Box<Expr> }
         |                             |
         v                             v
Heap:
  left_box  -> Expr::Grouping(
                 Box(
                   Expr::Binary { op: Add, left: Box(Literal(2)), right: Box(Literal(3)) }
                 )
               )
  right_box -> Expr::Literal(4)
```

### Borrow checker and evaluation

`evaluate(expr: &Expr)` borrows the AST immutably.

- Multiple read-only borrows are allowed.
- No ownership is transferred during traversal.
- This avoids copying the tree and prevents accidental mutation while evaluating.

## Rust Mental Models in This Lab

- Immutability by default:
  - Bindings are immutable unless marked `mut`.
  - This prevents accidental parser/evaluator state bugs.
- Mutability is explicit:
  - Lexer/parser cursors use `mut` where state movement is required.
- Speed:
  - `Vec<Token>` and `Box<Expr>` are predictable, low-overhead representations.
  - Borrowed references in evaluator avoid extra allocation and cloning.
- Safety:
  - `Result<T, E>` forces explicit handling of malformed input and runtime failure.

## Symbol Deep Dive

### `&` and `&mut`

- `&str` and `&Expr` are borrows, not ownership transfers.
- `&mut` is used where state must advance (for example lexer character cursor or parser position).
- Common misconception: “`&` means pass-by-reference.” In Rust, references are still values with strict aliasing rules.

### `*`

- In evaluator code, `*` can mean:
  - dereference (for example reading from `&f64`)
  - multiplication (for numeric expressions)
- Context determines meaning.

### `Box<T>`

- `Box<Expr>` stores child AST nodes on the heap.
- Required for recursive enums so the outer enum has a known size.

### `Result<T, E>` and `?`

- `Result` encodes success/failure in the type system.
- `?` returns early on error and keeps code linear.

## Grammar and Precedence

The parser implements precedence using layered functions:

```text
expression -> term ((+|-) term)*
term       -> factor ((*|/) factor)*
factor     -> NUMBER | "(" expression ")" | "-" factor
```

Why this matters:
- `2 + 3 * 4` parses as `2 + (3 * 4)`, not `(2 + 3) * 4`
- Parentheses override default precedence.

## How to Run

```bash
cargo run -p interpreter
cargo test -p interpreter
cargo check -p interpreter
```

## Exercises

1. Lexer (`src/lexer.rs`)
- Goal: turn source text into `Vec<Token>`.
- Constraints:
  - ignore whitespace
  - parse integers and decimals
  - fail on unexpected characters
- Edge cases:
  - malformed numbers
  - empty input
- Success looks like: tokens preserve exact operator/number order.

2. Parser (`src/parser.rs`)
- Goal: convert tokens to `Expr` AST with correct precedence and grouping.
- Constraints:
  - obey grammar rules
  - detect unexpected end-of-input and bad parentheses
- Edge cases:
  - unary minus
  - nested parentheses
- Success looks like: AST shape reflects precedence correctly.

3. Evaluator (`src/evaluator.rs`)
- Goal: recursively compute AST value.
- Constraints:
  - no AST ownership transfer
  - return `DivisionByZero` for invalid division
- Edge cases:
  - nested unary/binary expressions
  - division by zero in subexpression
- Success looks like: deterministic numeric result or correct error.

4. Orchestration (`src/lib.rs`)
- Goal: compose `tokenize -> parse -> evaluate`.
- Constraints:
  - map stage-specific errors into top-level `InterpreterError`
- Success looks like: one clear API, `interpret(input: &str) -> Result<f64, InterpreterError>`.

## What Tests Prove

- Basic arithmetic tests prove semantic correctness of operators.
- Precedence tests prove parser structure, not just math correctness.
- Parentheses tests prove grammar grouping behavior.
- Unary tests prove factor-level recursion and operator binding.
- Error tests prove failure contracts for each stage:
  - lexer rejects invalid characters
  - parser rejects malformed syntax
  - evaluator rejects runtime invalid operations

If a precedence test fails, parser structure is likely wrong.
If only division-by-zero tests fail, evaluator guard logic is likely wrong.
If malformed input crashes instead of returning `Err`, error propagation is wrong.

## Common Mistakes

1. Forgetting to consume tokens in parser loops.
2. Mixing precedence levels so `+` and `*` parse at the same depth.
3. Taking ownership of AST nodes in evaluator instead of borrowing (`&Expr`).
4. Treating all failures as one generic error (losing stage context).

## Performance Notes

- Tree-walking is simple and readable but not the fastest strategy.
- This lab optimizes for correctness + clarity first.
- Rust still provides strong baseline speed because:
  - no GC pauses
  - explicit memory layout
  - minimal copying due to borrowing

## Next Steps

After this lab, you can extend the language safely:

- identifiers and variables
- assignment and environments
- function calls
- boolean expressions and conditionals

Each extension reuses the same memory and ownership model you built here.
