# Project 56 - Structs and Methods (Alternative Track)

## What You Will Build

You will implement three data models with method-heavy APIs:

1. `Rectangle` geometry and transformations
2. `Point` coordinate math
3. `Counter` stateful fluent API (`&mut Self` chaining)

## Why This Lab Matters (First Principles)

This lab demonstrates how Rust models state without classes:

- Structs define memory layout.
- `impl` blocks define behavior.
- Receiver type (`self`, `&self`, `&mut self`) defines ownership and aliasing rules.

## Memory Model and Ownership

### High-level ownership flow

```text
Owned struct values (Rectangle/Point/Counter)
   |
   | methods borrow or consume based on receiver
   v
read/modify/return values safely
```

### Stack vs heap in this lab

- Stack:
  - `Rectangle` and `Point` numeric fields
  - `Counter { count: i32 }`
- Heap:
  - none required for core fields (all primitive scalars)

### Concrete memory sketch

```text
Counter chaining:
  let mut c = Counter::new();
  c.increment().increment().decrement();

Stack:
  c: Counter { count: 1 }
Borrowing:
  increment(&mut self) returns same mutable reference
```

### Borrow checker behavior

- `&self` enables many simultaneous readers.
- `&mut self` for `scale`, `translate`, `increment` requires exclusive access.
- `into_tuple(self)` consumes `Rectangle`; caller cannot use old value after move.

## Rust Mental Models in This Lab

- Immutability by default protects values from accidental mutation.
- Mutability is explicit for in-place updates.
- Speed: operations are constant-time scalar arithmetic.
- Safety: aliasing rules prevent two mutable writers.

## Symbol Deep Dive

### `&` and `&mut`

- `&self`: shared read access.
- `&mut self`: exclusive write access.
- Misconception: `&mut` does not mean globally mutable forever; it is scoped borrow.

### `*`

- `*` is multiplication in area/perimeter and scaling math.
- It is arithmetic, not pointer dereference.

### Additional symbols used here

- `::` for constructors (`Rectangle::new`, `Point::origin`)
- `->` for return contracts (`Rectangle`, `bool`, `&mut Self`, tuples)

## Exercises

1. Rectangle API
- Goal: implement constructors, geometry, containment, and mutation.
- Constraints: preserve strict semantics for `can_hold` and in-place scaling.
- Edge cases: zero sizes, equal sizes, factor 0.
- Success: rectangle test group passes.

2. Point API
- Goal: implement distance and translation.
- Constraints: floating-point calculations with tolerance-aware tests.
- Edge cases: origin, same-point distance.
- Success: point test group passes.

3. Counter API
- Goal: build mutable chainable methods and `Default`.
- Constraints: methods returning `&mut Self` support fluent style.
- Edge cases: negative counts, reset behavior.
- Success: counter test group passes.

## What Tests Prove

- Rectangle tests prove value semantics, copy/equality behavior, and geometric invariants.
- Point tests prove coordinate math and mutable updates.
- Counter tests prove borrow-checker-compatible fluent mutation.

Failure interpretation:

- Chaining errors usually mean wrong receiver/return type for mutable methods.
- Move errors around `into_tuple` indicate ownership-consumption confusion.

## Performance Notes

- All methods are O(1).
- No heap allocations in core struct operations.
- `Copy` types enable cheap by-value passing for primitives and small structs.

## How to Run

```bash
cargo run -p structs-methods-alternative
cargo test -p structs-methods-alternative
cargo check -p structs-methods-alternative
```
