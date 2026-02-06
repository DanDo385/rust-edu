# Project 05: Enums and Pattern Matching

## Overview
Enums let you define a type with multiple variants. Combined with pattern matching, they create a powerful and safe way to handle different cases. This is one of Rust's most expressive features.

## Concepts Taught
- **Enum definition** with variants
- **Enums with data** (variants can hold values)
- **Pattern matching** with `match`
- **if let** for simple matches
- **Option<T>** - Rust's solution to null
- **Result<T, E>** - error handling
- **Exhaustive matching**

## Why Rust Behaves This Way

### No Null Pointers!
Rust doesn't have null. Instead, it has `Option<T>`:
- `Some(value)` - contains a value
- `None` - no value

This forces you to handle the "no value" case explicitly, preventing null pointer errors.

**Tony Hoare** (inventor of null) called it his "billion-dollar mistake". Rust fixes this!

### Exhaustive Pattern Matching
The compiler ensures you handle ALL possible cases. This prevents bugs at compile time.

## Comparison: Rust vs Go vs Python

| Feature | Rust | Go | Python |
|---------|------|----|----|
| Null handling | Option<T> | nil + checks | None |
| Error handling | Result<T, E> | multiple returns | Exceptions |
| Pattern matching | match (exhaustive) | switch | match (3.10+) |
| Enums | Algebraic data types | const/iota | Enum class |

## Running This Project

```bash
cd 05-enums-pattern-matching
cargo run
```
