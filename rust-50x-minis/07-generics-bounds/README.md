# Project 07: Generics and Trait Bounds

## Overview
Generics enable code reuse across different types. Combined with trait bounds, they provide type-safe, zero-cost abstractions. This is the foundation of Rust's powerful type system.

## Concepts Taught
- **Generic functions** with `<T>`
- **Generic structs and enums**
- **Trait bounds** for constraining generics
- **Multiple trait bounds**
- **where clauses** for complex bounds
- **Monomorphization** (how generics compile)

## Why Rust Behaves This Way

### Zero-Cost Abstractions
Generics are **compile-time** feature. The compiler generates specialized code for each concrete type (monomorphization). No runtime overhead!

**Example**: `Vec<i32>` and `Vec<String>` compile to completely separate code.

### Type Safety
Trait bounds ensure generic code only works with types that support the required operations. Errors caught at compile time, not runtime.

## Running This Project

```bash
cd 07-generics-bounds
cargo run
```
