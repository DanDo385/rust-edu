# Project 03: Ownership and Borrowing

## Overview
This is THE most important project for understanding Rust. Ownership is Rust's superpower - it enables memory safety without garbage collection. This project teaches the ownership model through practical examples.

## Concepts Taught
- **Ownership rules**: Each value has an owner, only one owner at a time, value is dropped when owner goes out of scope
- **Move semantics**: Assignment and function calls move ownership
- **Borrowing**: References (&T) allow access without taking ownership
- **Mutable borrowing**: Exclusive mutable references (&mut T)
- **The Borrow Checker**: Rust's compile-time guardian
- **Copy vs Move types**
- **Stack vs Heap**

## Why Rust Behaves This Way

### Memory Safety Without GC
Traditional approaches to memory safety:
- **C/C++**: Manual memory management (fast, but unsafe - use-after-free, double-free)
- **Java/Python/Go**: Garbage collection (safe, but pauses and overhead)
- **Rust**: Ownership system (safe AND fast - compiler enforces correctness)

Rust's ownership system is checked at **compile time** - zero runtime cost!

### The Three Rules of Ownership
1. Each value has exactly one owner
2. When the owner goes out of scope, the value is dropped (freed)
3. You can borrow a value (reference) without taking ownership

**Why these rules?**
- Prevents use-after-free bugs
- Prevents double-free bugs
- Prevents data races in concurrent code
- All checked at compile time!

## Beginner Pitfalls & Borrow Checker Notes

### Pitfall 1: Moving a Value
```rust
let s1 = String::from("hello");
let s2 = s1;  // s1 is MOVED to s2
println!("{}", s1);  // ❌ ERROR: value borrowed after move
```
**Why**: String owns heap data. Rust moves ownership to prevent double-free.

### Pitfall 2: Mutable Borrowing Rules
```rust
let mut s = String::from("hello");
let r1 = &mut s;
let r2 = &mut s;  // ❌ ERROR: cannot borrow as mutable more than once
```
**Why**: Multiple mutable references could create data races.

### Pitfall 3: Mixing Immutable and Mutable References
```rust
let mut s = String::from("hello");
let r1 = &s;
let r2 = &s;
let r3 = &mut s;  // ❌ ERROR: cannot borrow as mutable while immutably borrowed
```
**Why**: Readers shouldn't see data change unexpectedly.

## Code Walkthrough

See `src/main.rs` for detailed examples of:
1. Ownership transfer (move semantics)
2. Borrowing (immutable references)
3. Mutable borrowing
4. Copy vs Move types
5. Lifetime of references

## Performance Considerations

**Zero-cost**: Ownership checking happens at compile time. The generated code is as fast as hand-written C with manual memory management.

**No GC pauses**: Unlike Java/Go/Python, Rust has predictable performance. No garbage collector pausing your program.

**Stack vs Heap**: Simple types (i32, bool) are `Copy` and live on the stack (very fast). Complex types (String, Vec) own heap data and are `Move` types.

## Comparison: Rust vs Go vs Python

| Aspect | Rust | Go | Python |
|--------|------|----|----|
| Memory management | Ownership (compile-time) | GC | GC |
| Performance | C-level (no overhead) | Good (GC pauses) | Slow (interpreted) |
| Safety | Compile-time guarantees | Runtime panics | Runtime errors |
| References | Explicit borrowing | Pointers (unsafe) | Everything is a reference |
| Concurrent safety | Enforced by compiler | Convention + runtime checks | GIL prevents parallelism |

## Running This Project

```bash
cd 03-ownership-borrowing
cargo run
```

## Additional Challenges

1. **String manipulation**: Write functions that take `&str` and `&mut String` parameters
2. **Reference counting**: Explore `Rc<T>` for shared ownership
3. **Interior mutability**: Try `RefCell<T>` for runtime borrowing checks

## Future Directions

- **Lifetimes** (Project 12): Explicit lifetime annotations
- **Smart pointers** (Project 16): Box, Rc, Arc, RefCell
- **Concurrency** (Project 17): How ownership prevents data races
