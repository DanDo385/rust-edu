# Project 12 - Enums and Pattern Matching

## What You Will Build

You will implement message-processing functions over a data-bearing enum. The goal is to model state variants safely and handle each case explicitly.

## Why This Lab Matters (First Principles)

Enums encode "one of many valid states" at the type level. Pattern matching is a control-flow proof that every state is handled.

- `Message` variants carry different payload shapes.
- `match` enforces exhaustiveness at compile time.
- `Option`-style thinking eliminates null-state ambiguity.

## Memory Model and Ownership

### High-level ownership flow

```text
Message value created by caller
   |
   | moved into process_message(msg: Message)
   v
match destructures active variant
   |
   | payload may move out (e.g., String)
   v
function returns owned String description
```

### Stack vs heap in this lab

- Stack:
  - Enum discriminant + inline fields where possible
  - references in borrowed helpers (`&Message`)
- Heap:
  - `String` payload for `Message::Write(String)`

### Concrete memory sketch

```text
Before:
  msg = Message::Write(String("hello"))

Stack: msg(discriminant=Write, ptr/len/cap)
Heap:  ['h','e','l','l','o']

In `process_message(msg)` match arm `Message::Write(text)`:
- ownership of `String` moves into `text`
- no clone required unless you choose to clone
```

### Borrow checker behavior

- `process_message(msg: Message)` consumes ownership intentionally.
- `variant_size(msg: &Message)` and `is_quit(msg: &Message)` borrow read-only.
- You cannot move out of `&Message` without cloning; compiler enforces this.

## Rust Mental Models in This Lab

- Immutability by default makes branch logic predictable.
- Mutability is explicit if you need to edit payload before formatting.
- Speed: `match` compiles to efficient branching over discriminants.
- Safety: impossible to forget a variant when `match` is exhaustive.

## Symbol Deep Dive

### `&` and `&mut`

- `&Message` means inspect without taking ownership.
- Misconception: `&` is a borrow, not "automatic pass-by-reference magic".

### `*`

- `*` may appear as arithmetic multiplication in unrelated helpers.
- In pattern contexts, `*` would mean dereference for referenced patterns, but that is optional here.

### Additional symbols used here

- `::` for variant paths (`Message::Quit`)
- `->` return guarantees
- pattern matching syntax with struct-like and tuple-like variants

## Exercises

1. `process_message`
- Goal: return readable text for every variant.
- Constraints: use exhaustive `match`.
- Edge cases: ensure all payload values appear in output.
- Success: variant-specific tests pass.

2. `variant_size`
- Goal: report field count per variant.
- Constraints: match on borrowed enum.
- Edge cases: none; deterministic mapping.
- Success: exact integer outputs.

3. `is_quit`
- Goal: detect only `Message::Quit`.
- Constraints: no false positives.
- Edge cases: all non-quit variants.
- Success: boolean tests pass.

## What Tests Prove

- Processing tests prove payload extraction and formatting.
- Size tests prove correct structural understanding of variants.
- Quit tests prove precise pattern discrimination.

Failure interpretation:

- Missing match arm causes compile-time exhaustiveness error.
- Wrong payload text usually means destructuring logic is incorrect.

## Performance Notes

- `variant_size` and `is_quit` are O(1).
- `process_message` may allocate output `String`; input `String` can be moved without clone.
- Enum dispatch is branch-efficient and cache-friendly for small variants.

## How to Run

```bash
cargo run -p enums-pattern-matching
cargo test -p enums-pattern-matching
cargo check -p enums-pattern-matching
```
