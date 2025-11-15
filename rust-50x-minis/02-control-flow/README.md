# Project 02: Control Flow and Functions

## Overview
This project teaches control flow structures (if/else, loops) and function definition in Rust through building a simple number guessing game. You'll learn how to get user input, use conditional logic, and organize code into functions.

## Concepts Taught
- **if/else expressions** and conditional logic
- **loop**, **while**, and **for** loops
- **Function definition** with parameters and return values
- **User input** with `std::io`
- **Error handling basics** with `Result` and `.expect()`
- **Random number generation** (using the `rand` crate)
- **Comparison operators** and **match** expressions

## Why Rust Behaves This Way

### if is an Expression
In Rust, `if` is an **expression**, not a statement. This means it returns a value:
```rust
let number = if condition { 5 } else { 6 };
```

**Comparison with other languages:**
- **Python**: Uses ternary operator `x if condition else y`
- **Go**: Has statements, not expressions for if
- **TypeScript**: Has ternary `condition ? x : y`

Rust's approach is more consistent - almost everything is an expression.

### Explicit Returns
Functions return the last expression implicitly (no `return` keyword needed):
```rust
fn add(a: i32, b: i32) -> i32 {
    a + b  // No semicolon = expression = return value
}
```

## Beginner Pitfalls & Borrow Checker Notes

### Pitfall 1: Semicolons vs Returns
```rust
fn bad() -> i32 {
    42;  // ❌ Semicolon makes it a statement, returns ()
}

fn good() -> i32 {
    42  // ✅ No semicolon = expression = returns 42
}
```

### Pitfall 2: Type Mismatch in if Branches
```rust
let x = if true { 5 } else { "six" };  // ❌ Both branches must return same type
```

### Pitfall 3: Reading Input is Fallible
```rust
let mut guess = String::new();
io::stdin().read_line(&mut guess);  // ❌ read_line returns Result, must handle it
io::stdin().read_line(&mut guess).expect("Failed");  // ✅ Handle error
```

## Code Walkthrough

See `src/main.rs` for:
1. A complete number guessing game
2. All loop types (loop, while, for)
3. Function definitions with various signatures
4. User input handling
5. Match expressions for comparisons

## Performance Considerations

**Loop performance**: All three loop types (`loop`, `while`, `for`) compile to the same efficient assembly. Choose based on clarity, not performance.

**Function calls**: Rust aggressively inlines small functions (zero-cost). The compiler optimizes away most function call overhead.

## Comparison: Rust vs Go vs Python

| Feature | Rust | Go | Python |
|---------|------|----|----|
| if as expression | Yes | No | Ternary only |
| Implicit returns | Yes (last expr) | No (explicit return) | Yes (implicit) |
| Loop types | loop, while, for | for, (while via for) | while, for |
| Pattern matching | match (exhaustive) | switch | match (3.10+) |
| Error handling | Result/Option | Multiple returns | Exceptions |

## Running This Project

```bash
cd 02-control-flow
cargo run
```

Play the guessing game and observe how control flow works!
