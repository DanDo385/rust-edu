# Project 44: Operator Overloading

## Overview
Implement custom numeric types (Rational numbers and Complex numbers) with operator overloading. This project demonstrates trait implementation, operator traits, mathematical operations, and creating expressive, natural-feeling APIs.

## Concepts Taught
- **Operator traits**: `Add`, `Sub`, `Mul`, `Div`, `Neg`
- **Comparison traits**: `PartialEq`, `Eq`, `PartialOrd`, `Ord`
- **Reference operators**: Implementing ops for `&T`
- **Associated types** in operator traits
- **std::ops module** and standard operators
- **Mathematical abstractions** (rational numbers, complex numbers)
- **Trait bounds** for generic numeric code
- **Display formatting** for custom types

## Why Rust Operator Overloading Is Special

### Type Safety
Rust's operator overloading is **type-safe**. You can't accidentally add incompatible types. Operators are implemented via traits, ensuring compile-time checking.

### Explicit, Not Implicit
Unlike C++, Rust doesn't allow implicit conversions. This prevents subtle bugs where operators do unexpected things.

### Zero-Cost Abstractions
Operator overloading has **zero runtime cost**. The compiler inlines operator methods, making custom operators as fast as primitive operations.

**Comparison with other languages:**
- **C++**: Powerful but can be abused (implicit conversions, hard to debug)
- **Python**: `__add__`, `__mul__`, etc. (slower, runtime overhead)
- **Go**: No operator overloading (intentionally simple)
- **Java**: No operator overloading (except String concatenation)

## Beginner Pitfalls & Best Practices

### Pitfall 1: Forgetting to Implement for References
```rust
// ❌ WRONG: Only implements for owned values
impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point { ... }
}

// This won't compile:
let result = &p1 + &p2;  // Error: trait not implemented for &Point
```
**Fix**: Implement for references too:
```rust
// ✅ CORRECT: Implement for both owned and borrowed
impl Add for &Point {
    type Output = Point;
    fn add(self, other: &Point) -> Point { ... }
}
```

### Pitfall 2: Moving Values in Operators
```rust
// ❌ WRONG: Operator consumes values
let a = Complex::new(1.0, 2.0);
let b = Complex::new(3.0, 4.0);
let c = a + b;  // a and b are moved!
let d = a + b;  // Error: a and b already moved
```
**Fix**: Use references or implement Copy:
```rust
// ✅ CORRECT: Use references
let c = &a + &b;
let d = &a + &b;  // OK!
```

### Pitfall 3: Not Implementing All Related Traits
```rust
// ❌ WRONG: Only Add, no AddAssign
impl Add for Point { ... }

// Can't do:
p1 += p2;  // Error: AddAssign not implemented
```
**Fix**: Implement related traits:
```rust
// ✅ CORRECT: Implement both
impl Add for Point { ... }
impl AddAssign for Point { ... }
```

### Pitfall 4: Inconsistent Operators
```rust
// ❌ WRONG: Operators don't follow mathematical rules
// a + b != b + a (violates commutativity)
```
**Fix**: Ensure operators follow mathematical laws:
```rust
// ✅ CORRECT: Respect mathematical properties
// - Commutativity: a + b == b + a
// - Associativity: (a + b) + c == a + (b + c)
// - Identity: a + 0 == a
```

## Code Walkthrough

See `src/main.rs` for a complete implementation that demonstrates:
1. Rational number type with arithmetic operators
2. Complex number type with full operator support
3. Operator implementation for owned and borrowed types
4. Comparison operators (`==`, `<`, `>`, etc.)
5. Display formatting for readable output
6. Generic functions using operator traits
7. Advanced: Matrix operations
8. Testing operator implementations

## Operator Traits Reference

### Arithmetic Operators
| Operator | Trait | Method | Example |
|----------|-------|--------|---------|
| `+` | `Add` | `add` | `a + b` |
| `-` | `Sub` | `sub` | `a - b` |
| `*` | `Mul` | `mul` | `a * b` |
| `/` | `Div` | `div` | `a / b` |
| `%` | `Rem` | `rem` | `a % b` |
| `-` (unary) | `Neg` | `neg` | `-a` |

### Compound Assignment Operators
| Operator | Trait | Method | Example |
|----------|-------|--------|---------|
| `+=` | `AddAssign` | `add_assign` | `a += b` |
| `-=` | `SubAssign` | `sub_assign` | `a -= b` |
| `*=` | `MulAssign` | `mul_assign` | `a *= b` |
| `/=` | `DivAssign` | `div_assign` | `a /= b` |
| `%=` | `RemAssign` | `rem_assign` | `a %= b` |

### Comparison Operators
| Operator | Trait | Method | Example |
|----------|-------|--------|---------|
| `==`, `!=` | `PartialEq` | `eq`, `ne` | `a == b` |
| `<`, `<=`, `>`, `>=` | `PartialOrd` | `partial_cmp` | `a < b` |

### Bitwise Operators
| Operator | Trait | Method | Example |
|----------|-------|--------|---------|
| `&` | `BitAnd` | `bitand` | `a & b` |
| `\|` | `BitOr` | `bitor` | `a \| b` |
| `^` | `BitXor` | `bitxor` | `a ^ b` |
| `!` | `Not` | `not` | `!a` |
| `<<` | `Shl` | `shl` | `a << b` |
| `>>` | `Shr` | `shr` | `a >> b` |

### Indexing Operators
| Operator | Trait | Method | Example |
|----------|-------|--------|---------|
| `[]` | `Index` | `index` | `a[i]` |
| `[]` (mut) | `IndexMut` | `index_mut` | `a[i] = x` |

## Performance Considerations

**Inlining**: Simple operator methods are inlined by the compiler. No function call overhead!

**Copying vs Moving**: For small types (like `f64`, `i32`), implement `Copy` to avoid moves.

**Reference operators**: For large types, implement operators for `&T` to avoid unnecessary clones.

**Benchmarks** (approximate):
- Custom `+` operator: Same speed as `f64 + f64` (~1 nanosecond)
- Complex number multiply: ~2-3 nanoseconds
- With proper inlining: Zero overhead compared to manual code

## Mathematical Properties

When implementing operators, respect these properties:

### Commutativity (for appropriate operations)
- `a + b == b + a` (addition)
- `a * b == b * a` (multiplication)
- But: `a - b != b - a` (subtraction is not commutative)

### Associativity
- `(a + b) + c == a + (b + c)`
- `(a * b) * c == a * (b * c)`

### Identity Elements
- `a + 0 == a` (additive identity)
- `a * 1 == a` (multiplicative identity)

### Inverse Elements
- `a + (-a) == 0` (additive inverse)
- `a * (1/a) == 1` (multiplicative inverse)

## Additional Challenges

1. **Vector type**: Implement 2D/3D vector with dot product, cross product, magnitude.

2. **Matrix type**: Build a matrix type with addition, multiplication, determinant.

3. **Fraction simplification**: Simplify rational numbers (reduce to lowest terms).

4. **Polynomial type**: Implement polynomials with addition and multiplication.

5. **BigInt**: Create arbitrary-precision integer type.

6. **Quaternions**: Extend complex numbers to 4D quaternions (used in 3D graphics).

7. **Modular arithmetic**: Implement numbers with modular operations (useful in cryptography).

8. **Units**: Create typed units (meters, seconds) with dimensional analysis.

## Future Directions

- **Linear algebra**: Full matrix library with BLAS integration
- **Computer graphics**: Implement transformation matrices
- **Game engines**: Physics vectors and quaternions
- **Cryptography**: Modular arithmetic and finite fields

## Running This Project

```bash
cd 44-operator-overloading
cargo run
```

**Note**: No external dependencies needed! This uses only standard library traits.

## Expected Output

The program will:
1. Demonstrate rational number arithmetic (fractions)
2. Show complex number operations
3. Compare custom types with comparison operators
4. Display formatted output
5. Test mathematical properties (commutativity, etc.)
6. Demonstrate generic functions with operator bounds
7. Show operator implementations for references

## Rational Numbers

**What they are**: Fractions represented as `numerator/denominator`.

**Example**: `1/2 + 1/3 = 5/6`

**Benefits**:
- Exact arithmetic (no floating-point errors)
- Perfect for financial calculations
- Used in computer algebra systems

## Complex Numbers

**What they are**: Numbers with real and imaginary parts: `a + bi`

**Example**: `(1 + 2i) * (3 + 4i) = -5 + 10i`

**Uses**:
- Signal processing (Fourier transforms)
- Quantum mechanics
- Electrical engineering (AC circuits)
- Computer graphics

## Comparison: Custom Numeric Types

| Type | Use Case | Precision | Speed |
|------|----------|-----------|-------|
| `f64` | General math | ~15 digits | Fastest |
| Rational | Exact fractions | Infinite (limited by int size) | Fast |
| Complex | 2D numbers | Depends on inner type | Fast |
| BigInt | Arbitrary precision | Unlimited | Slow |

## Trait Hierarchy

```
PartialEq (==, !=)
  └─ Eq (reflexive, symmetric, transitive)

PartialOrd (<, <=, >, >=)
  └─ Ord (total ordering)

Add, Sub, Mul, Div (arithmetic)
  └─ AddAssign, SubAssign, etc. (compound assignment)
```

## Advanced: Generic Numeric Functions

```rust
fn sum<T>(a: T, b: T) -> T
where
    T: Add<Output = T>,
{
    a + b
}
```

This function works with any type that implements `Add`!

## Real-World Examples

Popular Rust crates with operator overloading:
- **num-complex**: Complex number library
- **nalgebra**: Linear algebra (vectors, matrices)
- **num-bigint**: Arbitrary-precision integers
- **num-rational**: Rational number library
- **cgmath**: Computer graphics math
- **ndarray**: N-dimensional arrays (like NumPy)
