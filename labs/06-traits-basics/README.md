# Project 06 - Traits Basics

## What You Will Build

You will define a `Shape` trait and implement it for `Circle` and `Rectangle`, then compute aggregate metrics through trait objects.

## Why This Lab Matters (First Principles)

Traits are contracts over behavior. They let different concrete memory layouts share one interface:

- Static dispatch (`T: Shape`) for compile-time specialization.
- Dynamic dispatch (`&dyn Shape`) for heterogeneous collections.
- Safe polymorphism without inheritance hierarchies.

## Memory Model and Ownership

### High-level ownership flow

```text
Concrete shapes owned by caller (Circle, Rectangle)
   |
   | borrow as &dyn Shape
   v
aggregate functions read trait methods
   |
   | return plain values / borrowed reference
   v
caller retains ownership of all shape values
```

### Stack vs heap in this lab

- Stack:
  - concrete structs (`Circle { radius }`, `Rectangle { ... }`)
  - fat pointers for `&dyn Shape` (data ptr + vtable ptr)
- Heap:
  - optional: `Vec<&dyn Shape>` buffer itself

### Concrete memory sketch

```text
Stack:
  circle: Circle { radius: 5.0 }
  rect:   Rectangle { width: 10.0, height: 20.0 }
  shapes: [&dyn Shape, &dyn Shape]
             |             |
             v             v
          data ptr      data ptr
          vtable ptr    vtable ptr
```

### Borrow checker behavior

- `total_area(shapes: &[&dyn Shape])` only reads through shared borrows.
- `largest_shape<'a>(shapes: &[&'a dyn Shape]) -> Option<&'a dyn Shape>` ties output lifetime to input references.
- The compiler prevents returning references to temporaries.

## Rust Mental Models in This Lab

- Immutability by default supports pure geometry operations.
- Mutability is explicit and usually unnecessary here.
- Speed: dynamic dispatch has tiny vtable overhead; static dispatch is monomorphized.
- Safety: trait object usage is memory-safe and lifetime-checked.

## Symbol Deep Dive

### `&` and `&mut`

- `&dyn Shape` is a shared borrowed trait object.
- You pass references, not copies of full objects.
- Misconception: `dyn` does not imply heap allocation by itself.

### `*`

- `*` appears as arithmetic multiplication in formulas.
- It is not dereference in geometry expressions.

### Additional symbols used here

- `::` for `f64::consts::PI`
- `->` return type guarantees
- `&dyn Trait` means dynamic dispatch through borrowed trait object

## Exercises

1. Implement `Shape` for `Circle`
- Goal: area/perimeter formulas.
- Constraints: use `PI` and floating-point math.
- Edge cases: radius zero.
- Success: approximation tests pass.

2. Implement `Shape` for `Rectangle`
- Goal: exact area/perimeter.
- Constraints: no mutation.
- Edge cases: square rectangle.
- Success: numeric equality tests pass.

3. `total_area`
- Goal: sum across heterogeneous shapes.
- Constraints: iterate over trait-object slice.
- Edge cases: empty list.
- Success: aggregate total within tolerance.

4. `largest_shape`
- Goal: return borrowed shape with max area.
- Constraints: preserve lifetimes.
- Edge cases: empty list -> `None`.
- Success: selected shape matches expected max.

## What Tests Prove

- Per-shape tests prove local formula correctness.
- Aggregate tests prove trait polymorphism works across mixed types.
- Largest-shape tests prove reference-return lifetime correctness.

Failure interpretation:

- Lifetime/compiler errors usually indicate returning reference to a local temporary.
- Area mismatch suggests formula or floating precision tolerance issues.

## Performance Notes

- `total_area` is O(n).
- Dynamic dispatch adds one indirection per method call.
- No cloning needed; everything works through borrows.

## How to Run

```bash
cargo run -p traits-basics
cargo test -p traits-basics
cargo check -p traits-basics
```
