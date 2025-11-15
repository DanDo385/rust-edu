# Project 01: Variables and Types

## Overview
This project introduces Rust's fundamental data types, variables, mutability, and basic printing. You'll learn how Rust's type system works, the difference between mutable and immutable variables, and how the compiler enforces safety at compile-time.

## Concepts Taught
- **Variable declaration** with `let`
- **Mutability** with `mut`
- **Type inference** and explicit type annotations
- **Primitive types**: integers (i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize), floats (f32, f64), booleans, characters
- **String types**: `String` vs `&str`
- **Type conversion** and casting
- **Printing** with `println!` macro

## Why Rust Behaves This Way

### Immutability by Default
Rust makes variables immutable by default. This prevents accidental mutations and makes code easier to reason about. When you need mutability, you must explicitly opt-in with `mut`.

**Comparison with other languages:**
- **Python**: Variables are always mutable (you can reassign them freely)
- **Go**: Variables declared with `:=` or `var` are mutable by default
- **TypeScript**: Use `const` for immutable, `let` for mutable

In Rust, the compiler will stop you from accidentally changing a value you didn't intend to change.

### Type Safety
Rust is **statically typed** - all types must be known at compile time. However, Rust has **type inference**, so you often don't need to write types explicitly.

## Beginner Pitfalls & Borrow Checker Notes

### Pitfall 1: Forgetting `mut`
```rust
let x = 5;
x = 6;  // ❌ ERROR: cannot assign twice to immutable variable
```
**Fix**: Add `mut` to make it mutable:
```rust
let mut x = 5;
x = 6;  // ✅ OK
```

### Pitfall 2: String vs &str Confusion
```rust
let s: String = "hello";  // ❌ ERROR: expected String, found &str
```
**Fix**: Use `.to_string()` or `String::from()`:
```rust
let s: String = "hello".to_string();  // ✅ OK
let s: String = String::from("hello");  // ✅ Also OK
```

### Pitfall 3: Integer Overflow in Debug Mode
Rust checks for integer overflow in debug builds and **panics**. In release builds, it wraps around (uses two's complement).

## Code Walkthrough

See `src/main.rs` for a detailed, commented implementation that demonstrates:
1. Variable declaration and mutability
2. All primitive types
3. Type inference vs explicit annotations
4. String types and conversion
5. Type casting
6. Formatted printing

## Performance Considerations

**Zero-cost abstractions**: Rust's type system and immutability guarantees happen at compile-time. There is **NO runtime cost** for these features. The compiled code is just as fast as C.

**Integer types**: Choose the smallest type that fits your needs:
- `i32` is the default and usually the fastest on modern processors
- `u8` saves memory if you're storing many small numbers
- `i64`/`u64` for large numbers

**String allocation**: `String` allocates on the heap (slow), while `&str` is a reference (fast). Use `&str` when you don't need to modify the string.

## Comparison: Rust vs Go vs Python

| Feature | Rust | Go | Python |
|---------|------|----|----|
| Mutability | Immutable by default | Mutable by default | Mutable by default |
| Type system | Static, inferred | Static, explicit | Dynamic |
| String types | `String` (owned), `&str` (borrowed) | `string` (immutable) | `str` (immutable) |
| Integer types | Multiple sizes (i8-i128, u8-u128) | int, int8-int64, uint8-uint64 | int (unlimited precision) |
| Compile-time checks | Very strict | Moderate | None (runtime errors) |

## Additional Challenges

1. **Type conversion**: Write a program that converts between different numeric types (e.g., `f64` to `i32`) and prints the results.

2. **Calculator**: Create a simple calculator that takes two numbers and an operator as input and prints the result.

3. **Temperature converter**: Convert between Celsius and Fahrenheit using user input.

4. **Overflow exploration**: Create a program that demonstrates integer overflow behavior in debug vs release mode.

## Future Directions

- **Next**: Move to control flow and functions (Project 02)
- **Later**: Learn about ownership and borrowing (Project 03)
- **Advanced**: Explore advanced type system features like generics (Project 07)

## Running This Project

```bash
cd 01-variables-types
cargo run
```

## Expected Output

You should see output demonstrating all the different types and their values, formatted nicely with the `println!` macro.
