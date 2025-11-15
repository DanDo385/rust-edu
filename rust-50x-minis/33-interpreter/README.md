# Project 33: Expression Interpreter

## Overview
Build a simple interpreter for arithmetic expressions, demonstrating parsing, Abstract Syntax Tree (AST) construction, and recursive evaluation. This project teaches the fundamentals of language implementation, from tokenization to execution.

## Concepts Taught
- **Lexical analysis**: converting text to tokens
- **Parsing**: converting tokens to AST
- **Abstract Syntax Tree (AST)**: representing program structure
- **Recursive evaluation**: traversing and computing AST
- **Enums for AST nodes**: sum types for different expression kinds
- **Pattern matching**: handling different AST node types
- **Operator precedence**: implementing correct evaluation order
- **Error handling**: reporting syntax and runtime errors

## Why Interpreters Work

### The Interpretation Pipeline
1. **Source Code** → "2 + 3 * 4"
2. **Tokenization** → [Number(2), Plus, Number(3), Star, Number(4)]
3. **Parsing** → AST: Add(2, Multiply(3, 4))
4. **Evaluation** → 14

Each stage transforms input into a representation that's easier to work with.

### Abstract Syntax Trees (AST)
An AST represents the syntactic structure of code:
- **Nodes**: operations, literals, variables
- **Tree structure**: reflects operator precedence
- **No syntax details**: parentheses, whitespace are gone

**Example**: `2 + 3 * 4` becomes:
```
    +
   / \
  2   *
     / \
    3   4
```

### Why Recursive Evaluation
Trees are recursive data structures, so we use recursive functions to traverse them:
- Base case: literal values
- Recursive case: evaluate children, then apply operator
- Natural fit for expression evaluation

## Why Rust Behaves This Way

### Enums for AST Nodes
Rust's enums are perfect for ASTs because they're **sum types**:
```rust
enum Expr {
    Number(f64),
    BinOp(Box<Expr>, Op, Box<Expr>),
}
```
- Each variant can hold different data
- Pattern matching ensures you handle all cases
- Compiler prevents missing cases at compile-time

**Comparison with other languages:**
- **Python**: Use classes with inheritance (more verbose, runtime errors)
- **Go**: Use interfaces with type assertions (runtime errors possible)
- **TypeScript**: Discriminated unions (similar to Rust, but less safe)
- **Rust**: Enums + pattern matching (exhaustive, compile-time checked)

### Box for Recursive Types
You can't have infinitely-sized types:
```rust
enum Expr {
    BinOp(Expr, Op, Expr),  // ❌ ERROR: infinite size
}
```
**Fix**: Use `Box<Expr>` (heap allocation, pointer-sized):
```rust
enum Expr {
    BinOp(Box<Expr>, Op, Box<Expr>),  // ✅ OK
}
```

## Beginner Pitfalls & Borrow Checker Notes

### Pitfall 1: Forgetting Box for Recursive Enums
```rust
enum Expr {
    Add(Expr, Expr),  // ❌ ERROR: recursive type has infinite size
}
```
**Fix**: Use `Box<T>` for indirection:
```rust
enum Expr {
    Add(Box<Expr>, Box<Expr>),  // ✅ OK
}
```

### Pitfall 2: Non-Exhaustive Pattern Matching
```rust
match expr {
    Expr::Number(n) => n,
    Expr::Add(l, r) => eval(l) + eval(r),
    // ❌ ERROR: missing Expr::Subtract
}
```
**Fix**: Handle all variants or use wildcard:
```rust
match expr {
    Expr::Number(n) => n,
    Expr::Add(l, r) => eval(l) + eval(r),
    _ => panic!("Unimplemented"),  // ✅ OK
}
```

### Pitfall 3: String Parsing Complexity
Manual string parsing is error-prone. Consider using parser combinator libraries like `nom` or `pest` for production code.

### Pitfall 4: Stack Overflow with Deep Recursion
Very deep expressions can overflow the stack:
```rust
eval(1 + 1 + 1 + ... 10000 times)  // ❌ Stack overflow
```
**Fix**: Use iterative evaluation or increase stack size.

## Code Walkthrough

See `src/main.rs` for a detailed, commented implementation that demonstrates:
1. Token definition and tokenizer (lexer)
2. AST node definitions using enums
3. Recursive descent parser
4. Recursive evaluator
5. Error handling for invalid syntax
6. Support for +, -, *, /, parentheses
7. Proper operator precedence

## Performance Considerations

**Tokenization:**
- O(n) where n = input length
- Each character examined once
- String allocations for numbers

**Parsing:**
- O(n) for our simple recursive descent parser
- Creates AST nodes (heap allocations)
- Could use arena allocator for better performance

**Evaluation:**
- O(nodes) where nodes = AST size
- Each node visited once
- Recursive calls use stack space

**Memory:**
- AST nodes: ~32 bytes each (with Box pointers)
- Stack depth: proportional to expression nesting
- Could optimize with iterative evaluation

**Real-World Optimizations:**
- **JIT compilation**: compile AST to machine code (like LuaJIT, V8)
- **Bytecode**: compile to bytecode, interpret bytecode (like Python)
- **Tree-walking optimizations**: inline caching, quickening
- **Parser generators**: use tools like LALRPOP, pest

## Comparison: Rust vs Go vs Python

| Feature | Rust | Go | Python |
|---------|------|----|----|
| AST representation | Enums with Box | Interfaces | Classes/dataclasses |
| Pattern matching | Exhaustive, compile-time | Type switches, runtime | if/elif, runtime |
| Memory safety | Compile-time | Runtime (nil pointers) | Runtime |
| Performance | Fastest | Fast | Slower |
| Parser tools | nom, pest, LALRPOP | go/parser, antlr | PLY, lark, pyparsing |

## Additional Challenges

1. **Variables**: Add variable assignment and lookup (symbol table).

2. **Functions**: Support user-defined functions with parameters.

3. **Control Flow**: Add if/else statements.

4. **Type System**: Add type checking (int vs float vs bool).

5. **REPL**: Create a read-eval-print loop for interactive use.

6. **Bytecode Compiler**: Compile to bytecode instead of direct interpretation.

7. **Better Errors**: Add line/column numbers and helpful error messages.

8. **Full Language**: Implement a subset of Lua, JavaScript, or Python.

## Real-World Usage

Interpreters are everywhere:
- **Python**: CPython is a bytecode interpreter
- **Ruby**: YARV interpreter
- **JavaScript**: V8, SpiderMonkey (with JIT)
- **Lua**: LuaJIT (fastest interpreter)
- **SQL**: Database query engines
- **RegEx**: Regular expression engines
- **Calculators**: Expression evaluators
- **Configuration Languages**: JSON, TOML, YAML interpreters

## Parser Tools for Rust

- **nom**: Parser combinator library (fast, type-safe)
- **pest**: PEG parser with external grammar files
- **LALRPOP**: LR(1) parser generator
- **combine**: Parser combinator library
- **pom**: Simple parser combinator

## Running This Project

```bash
cd 33-interpreter
cargo run
```

## Expected Output

You should see:
1. Tokenization examples showing token streams
2. AST construction for various expressions
3. Evaluation results with correct operator precedence
4. Error handling for malformed expressions
5. Complex expressions with parentheses
6. Step-by-step evaluation trace
