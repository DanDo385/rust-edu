# Project 08: Error Handling

## Overview
Learn Rust's robust error handling with Result and Option. Build a file-reading CLI tool that gracefully handles errors without panicking.

## Concepts Taught
- **Result<T, E>** for recoverable errors
- **Option<T>** for optional values
- **?** operator for error propagation
- **unwrap()** and **expect()** for prototyping
- **unwrap_or()** and **unwrap_or_else()** for defaults
- **Custom error types**
- **panic!** for unrecoverable errors

## Why Rust Behaves This Way

### No Exceptions
Unlike Java/Python, Rust doesn't have exceptions. Instead:
- **Result<T, E>**: For operations that can fail
- **panic!**: For unrecoverable errors (program crash)

This makes error handling **explicit** and **type-safe**. You can't forget to handle errors!

### The ? Operator
The `?` operator propagates errors automatically:
```rust
let contents = fs::read_to_string(path)?;  // Returns early if error
```

This is more ergonomic than manual error checking in C or Go.

## Comparison: Rust vs Go vs Python

| Feature | Rust | Go | Python |
|---------|------|----|----|
| Error handling | Result<T, E> | Multiple returns (val, err) | Exceptions (try/except) |
| Forced handling | Yes (compiler enforced) | No (convention) | No (can ignore) |
| Error propagation | ? operator | if err != nil { return err } | raise/reraise |
| Unrecoverable | panic! | panic() | SystemExit |

## Running This Project

```bash
cd 08-error-handling
cargo run
```
