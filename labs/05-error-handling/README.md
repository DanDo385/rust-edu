# Project 05 - Error Handling

## What You Will Build

You will implement a small validation/parsing layer that returns `Result`/`Option` instead of panicking. This lab teaches explicit failure handling as part of your API contract.

## Why This Lab Matters (First Principles)

Computers fail at I/O, parsing, and invalid input. Rust makes failures part of the type system:

- `Result<T, E>` means success or explicit error.
- `Option<T>` means value may be absent.
- No hidden exceptions crossing function boundaries.

## Memory Model and Ownership

### High-level ownership flow

```text
Borrowed input (&str/path)
   |
   | parse/validate
   v
Result<T, E> returned by value
   |
   | caller matches on Ok/Err
   v
error/value dropped at scope end (RAII)
```

### Stack vs heap in this lab

- Stack:
  - `Result` and `Option` enum tags
  - lightweight error enums
- Heap:
  - `String` in `ParseError::InvalidFormat(String)`
  - file contents when reading lines

### Concrete memory sketch

```text
parse_number("abc")
Stack:
  input: &str -> "abc"
  result: Err(ParseError::InvalidFormat(String))
Heap:
  String buffer: "abc"
```

### Borrow checker behavior

- `parse_number(s: &str)` borrows input; no ownership transfer.
- Returning `Result<i32, ParseError>` transfers owned error/value to caller.
- `read_first_line(path: &str)` isolates I/O lifetime and returns owned `String`.

## Rust Mental Models in This Lab

- Immutability by default limits accidental state changes during validation.
- Mutability is explicit for buffers/readers only.
- Speed: no exception machinery; branching on enum tags is predictable.
- Safety: caller must handle errors before using success value.

## Symbol Deep Dive

### `&` and `&mut`

- `&str` inputs are read-only borrows.
- Use `&mut` only when mutating local buffers (e.g., line reads).
- Misconception: borrowing input does not mean result borrows it too; you can return owned errors safely.

### `*`

- `*` appears only as arithmetic/multiplication if used in helper logic.
- No raw pointer dereference is required for this lab.

### Additional symbols used here

- `::` for enum variants (`ParseError::OutOfRange`)
- `->` for fallible return types
- `?` for concise error propagation in file I/O paths

## Exercises

1. `parse_number`
- Goal: trim and parse integer with custom error mapping.
- Constraints: return semantic errors, no panic.
- Edge cases: whitespace, invalid text, overflow/range checks.
- Success: valid parses and expected `Err` variants.

2. `divide`
- Goal: safe division with `Result`.
- Constraints: reject denominator `0.0`.
- Edge cases: negatives, fractions.
- Success: deterministic `Ok`/`Err` behavior.

3. `read_first_line`
- Goal: read first line from file path.
- Constraints: propagate `std::io::Error`.
- Edge cases: missing file, empty file.
- Success: I/O tests and manual checks pass.

4. `validate_email`
- Goal: basic structural validation.
- Constraints: lightweight checks only.
- Edge cases: missing `@`, missing domain/local part.
- Success: true/false behavior matches tests.

## What Tests Prove

- Parse tests prove mapping from invalid text to explicit errors.
- Divide tests prove no silent division-by-zero.
- Email tests prove baseline input validation rules.

Failure interpretation:

- Unexpected panic means a fallible path is still using `unwrap`.
- False positives in email checks indicate insufficient structural constraints.

## Performance Notes

- Result/Option are zero-cost abstractions in optimized builds.
- Most operations are O(n) in input length.
- Error types keep failure paths explicit without runtime exception overhead.

## How to Run

```bash
cargo run -p error-handling
cargo test -p error-handling
cargo check -p error-handling
```
