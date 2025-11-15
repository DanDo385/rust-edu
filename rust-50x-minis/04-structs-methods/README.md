# Project 04: Structs and Methods

## Overview
Learn how to organize related data using structs and add behavior with methods. This project shows how Rust implements object-oriented-like patterns while maintaining its ownership guarantees.

## Concepts Taught
- **Struct definition** and instantiation
- **Field access** and tuple structs
- **Methods** with `impl` blocks
- **Associated functions** (static methods)
- **`self`, `&self`, and `&mut self`** patterns
- **Method chaining**
- **Unit-like structs**

## Why Rust Behaves This Way

### No Classes, but Structs + Traits
Rust doesn't have classes. Instead, it separates:
- **Data** (structs)
- **Behavior** (impl blocks and traits)

This is more flexible than traditional OOP and enables zero-cost abstractions.

### Three Forms of self

- `self` - takes ownership (consumes the value)
- `&self` - borrows immutably (reads but doesn't modify)
- `&mut self` - borrows mutably (can modify)

This explicit ownership in methods prevents common OOP bugs.

## Comparison: Rust vs Go vs Python

| Feature | Rust | Go | Python |
|---------|------|----|----|
| Data structures | struct | struct | class |
| Methods | impl blocks | methods on structs | class methods |
| Constructors | Associated functions | Constructor pattern | __init__ |
| Method self | explicit (&self, &mut self) | explicit receiver | implicit self |

## Running This Project

```bash
cd 04-structs-methods
cargo run
```
