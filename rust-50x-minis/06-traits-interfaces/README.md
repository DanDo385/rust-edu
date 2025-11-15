# Project 06: Traits and Polymorphism

## Overview
Traits define shared behavior across types. They're similar to interfaces in other languages but more powerful. Traits enable Rust's zero-cost polymorphism.

## Concepts Taught
- **Trait definition** and implementation
- **Default implementations**
- **Trait bounds** on functions
- **Returning traits** with impl Trait
- **Trait objects** with dyn Trait
- **Derive macros** for common traits
- **Operator overloading** via traits

## Why Rust Behaves This Way

### Traits vs Interfaces
Traits are more powerful than traditional interfaces:
- Can provide default implementations
- Can be implemented for external types
- Enable zero-cost abstractions through monomorphization
- Support operator overloading

### Static vs Dynamic Dispatch
- **Static** (impl Trait, generics): Monomorphization at compile-time, zero cost
- **Dynamic** (dyn Trait): Runtime vtable lookup, small overhead

## Running This Project

```bash
cd 06-traits-interfaces
cargo run
```
