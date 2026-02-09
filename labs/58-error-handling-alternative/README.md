# Project 58 - Error Handling (Alternative Track)

## What You Will Build

You will implement both `Option` and `Result` workflows, custom error enums, and `?`-based error propagation across numeric and parsing operations.

## Why This Lab Matters (First Principles)

Reliable systems treat failure as data.

- `Option<T>` models absence.
- `Result<T, E>` models recoverable failure with reasons.
- Custom error enums encode domain invariants.

## Memory Model and Ownership

### High-level ownership flow

```text
Borrowed inputs (&str, &[i32])
   |
   | parse/compute/check
   v
Option/Result returned by value
   |
   v
caller handles success/failure explicitly
```

### Stack vs heap in this lab

- Stack:
  - enum discriminants (`Some/None`, `Ok/Err`)
  - primitive numeric intermediates
- Heap:
  - error payloads when variants carry owned strings/data

### Concrete memory sketch

```text
complex_calculation(a,b,c): Result<f64, MathError>
Stack:
  step1 = safe_divide(a,b)?
  step2 = safe_sqrt(c)?
  result = step1 + step2

If step1 is Err, function returns immediately (no step2).
```

### Borrow checker behavior

- Borrowed inputs prevent unnecessary cloning.
- `?` moves `Err` up to caller while preserving type safety.
- No references to temporary error values escape function scope.

## Rust Mental Models in This Lab

- Immutability by default prevents accidental mutation in failure paths.
- Mutability is explicit only for local accumulators if needed.
- Speed: enum-based branching is predictable and low overhead.
- Safety: compiler enforces that error cases are handled by callers.

## Symbol Deep Dive

### `&` and `&mut`

- `&[i32]`/`&str` are shared read-only borrows.
- Most APIs here are pure and need no `&mut`.
- Misconception: borrowing input does not tie output lifetime when output is owned `Result`/`Option`.

### `*`

- `*` appears as multiplication in numeric formulas only.
- Dereference is not central to this lab’s API.

### Additional symbols used here

- `::` for custom error variants
- `->` exact fallible contracts
- `?` for early-return error propagation

## Exercises

1. Option helpers (`divide`, `safe_get`, `first_even`)
- Goal: represent absence safely.
- Constraints: no panics.
- Edge cases: divide by zero, empty slices, no even numbers.
- Success: Option tests pass.

2. Result math helpers (`safe_divide`, `safe_sqrt`, `safe_add`, `safe_multiply`)
- Goal: map invalid states to precise error variants.
- Constraints: checked arithmetic for overflow.
- Edge cases: negative sqrt, overflow boundaries.
- Success: Result tests pass.

3. Composed flow (`complex_calculation`)
- Goal: combine fallible steps with `?`.
- Constraints: first failing step short-circuits.
- Edge cases: multiple potential failure points.
- Success: propagation tests pass.

4. Parsing helpers (`parse_positive_bounded`, `parse_and_double`)
- Goal: convert text to validated numbers.
- Constraints: clear error mapping.
- Edge cases: empty/invalid/negative/out-of-range input.
- Success: parse tests pass.

## What Tests Prove

- Option tests prove safe handling of missing values.
- Result tests prove domain-specific error correctness.
- Propagation tests prove short-circuit behavior with `?`.
- Parsing/combinator tests prove robust input handling without panic.

Failure interpretation:

- Panic in tests usually means `unwrap` leaked into production path.
- Wrong error variant means validation ordering or mapping bug.

## Performance Notes

- Most functions are O(1) or O(n) over input length.
- Error enums avoid exception runtime machinery.
- Checked math adds tiny overhead for strong correctness guarantees.

## How to Run

```bash
cargo run -p error-handling-alternative
cargo test -p error-handling-alternative
cargo check -p error-handling-alternative
```
