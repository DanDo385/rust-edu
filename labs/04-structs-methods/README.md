# Project 04 - Structs and Methods

## What You Will Build

You will model rectangles as a Rust `struct` and implement methods for geometry and containment. This lab teaches how data and behavior compose without classes.

## Why This Lab Matters (First Principles)

A `struct` is a concrete memory layout plus methods that operate on that layout. This is about data ownership and aliasing discipline:

- `Rectangle` fields are plain values (`u32`) stored inline.
- `&self` methods read without moving ownership.
- Returning a new `Rectangle` (`scale`) demonstrates value semantics.

## Memory Model and Ownership

### High-level ownership flow

```text
Rectangle value owned by caller
   |
   | borrow via &self
   v
read-only method computes result
   |
   | returns value
   v
caller still owns original Rectangle
```

### Stack vs heap in this lab

- Stack:
  - `Rectangle { width, height }`
  - Method locals and return values
- Heap:
  - None required by core model (no `String`/`Vec` fields)

### Concrete memory sketch

```text
Stack:
  rect -> Rectangle { width: 10, height: 20 }
  borrow -> &rect (shared reference)

No heap allocation required for Rectangle itself.
```

### Borrow checker behavior

- `area(&self)` and `perimeter(&self)` take shared borrows.
- `can_fit(&self, other: &Rectangle)` creates two shared borrows, safe for concurrent reads.
- `scale(&self, factor)` should not mutate original unless API says `&mut self`.

## Rust Mental Models in This Lab

- Immutability by default keeps geometry computations side-effect free.
- Mutability is explicit; your current API returns a scaled copy.
- Speed: POD-like struct with inline `u32`s is cache-friendly.
- Safety: impossible to read uninitialized fields with proper constructors.

## Symbol Deep Dive

### `&` and `&mut`

- `&self` means: borrow the existing rectangle, do not consume it.
- `&Rectangle` in `can_fit` means both rectangles stay owned by caller.
- Misconception: `&self` does not clone the object.

### `*`

- `*` here should mean multiplication (`width * height`) in area math.
- It is arithmetic, not pointer dereference in this context.

### Additional symbols used here

- `::` for associated constructor `Rectangle::new`
- `->` for exact return type contract (`Option<Rectangle>`, `u32`, `bool`)

## Exercises

1. `Rectangle::new`
- Goal: validate dimensions and construct rectangle.
- Constraints: reject zero width/height via `None`.
- Edge cases: `(0, n)`, `(n, 0)`, `(0, 0)`.
- Success: constructor tests pass.

2. `area` and `perimeter`
- Goal: compute standard formulas.
- Constraints: no mutation.
- Edge cases: squares, thin rectangles.
- Success: exact numeric outputs.

3. `can_fit`
- Goal: geometric containment check.
- Constraints: compare both dimensions.
- Edge cases: equal sizes, only one dimension larger.
- Success: boolean tests pass.

4. `scale`
- Goal: return a scaled copy.
- Constraints: original rectangle remains unchanged.
- Edge cases: factor 1, larger factors.
- Success: scaled dimensions correct and original unchanged.

## What Tests Prove

- Constructor tests prove invariants: dimensions must be positive.
- Geometry tests prove method correctness and pure behavior.
- Fit tests prove containment logic.
- Scale tests prove value semantics (new value returned).

Failure interpretation:

- Constructor failures indicate invariant checks are incomplete.
- Scale regression usually means accidental in-place mutation.

## Performance Notes

- All operations are O(1).
- No heap allocation in core methods.
- Returning small structs by value is inexpensive.

## How to Run

```bash
cargo run -p structs-methods
cargo test -p structs-methods
cargo check -p structs-methods
```
