# Project 13 - Generics and Trait Bounds

## What You Will Build

You will implement generic algorithms and data types (`largest`, `Point<T>`, `Pair<T, U>`) with trait bounds that encode what operations are legal.

## Why This Lab Matters (First Principles)

Generics separate algorithm shape from concrete type while preserving performance.

- Trait bounds (`PartialOrd`, `Copy`, `Display`) define required capabilities.
- Monomorphization generates concrete code per used type.
- Lifetimes on borrowed returns prevent dangling references.

## Memory Model and Ownership

### High-level ownership flow

```text
Generic function receives borrowed or owned values
   |
   | trait-bound operations compile per concrete T
   v
returns value/reference based on API contract
   |
   v
ownership and drops remain fully explicit
```

### Stack vs heap in this lab

- Stack:
  - `Point<T>` / `Pair<T,U>` struct handles and fields (if inline types)
  - references returned by `x()` and `y()`
- Heap:
  - only if `T`/`U` themselves own heap memory (e.g., `String`)

### Concrete memory sketch

```text
Point<String> p
Stack:
  p.x -> String(ptr,len,cap)
  p.y -> String(ptr,len,cap)
Heap:
  bytes for each string

p.x() returns &String (borrow), not a clone.
```

### Borrow checker behavior

- `x(&self) -> &T` ties output lifetime to `self`.
- `swap(self) -> Pair<U, T>` consumes `self` and moves fields.
- You cannot use `self` after `swap` because ownership moved.

## Rust Mental Models in This Lab

- Immutability by default keeps generic APIs predictable.
- Mutability is explicit if you later add setters.
- Speed: generics are usually zero-cost after monomorphization.
- Safety: trait bounds prevent invalid operations at compile time.

## Symbol Deep Dive

### `&` and `&mut`

- `&[T]` and `&self` are non-owning borrows.
- `&Point<T>` in comparisons avoids moving points.
- Misconception: borrowed return `&T` does not extend object lifetime; it is constrained by borrow scope.

### `*`

- In tests, `*p.x()` dereferences `&T` when `T: Copy` (e.g., `i32`).
- Dereference does not always copy; copy behavior depends on `T` implementing `Copy`.

### Additional symbols used here

- `::` for constructors (`Point::new`, `Pair::new`)
- `->` for type-level contracts
- `<T: Bound>` for compile-time capability constraints

## Exercises

1. `largest<T: PartialOrd + Copy>`
- Goal: return max item from non-empty slice.
- Constraints: avoid allocation; single scan.
- Edge cases: duplicate maxima.
- Success: integer/float/char tests pass.

2. `Point<T>` methods
- Goal: construct and borrow fields safely.
- Constraints: `x()`/`y()` return references.
- Edge cases: heap-owning `T` like `String`.
- Success: reference-based tests pass.

3. `compare_distance`
- Goal: produce teaching-friendly comparison output.
- Constraints: requires `Display` + `PartialOrd` bounds.
- Edge cases: equal coordinates.
- Success: formatted string includes expected values.

4. `Pair<T,U>` and `swap`
- Goal: model ownership moves between generic fields.
- Constraints: `swap` consumes `self`.
- Edge cases: mixed owned/borrowed-like types.
- Success: swapped pair type/value tests pass.

## What Tests Prove

- `largest` tests prove trait-bound generic correctness across multiple types.
- `Point` tests prove borrowing semantics and clone/equality behavior.
- `Pair` tests prove ownership transfer in consuming methods.

Failure interpretation:

- Borrow/lifetime compiler errors usually mean returning refs to temporaries.
- Type mismatch often indicates insufficient or incorrect trait bounds.

## Performance Notes

- `largest` is O(n) and allocation-free.
- Generic specialization avoids dynamic dispatch overhead in this lab.
- Moves in `swap` are usually cheap field moves, not deep copies.

## How to Run

```bash
cargo run -p generics-bounds
cargo test -p generics-bounds
cargo check -p generics-bounds
```
