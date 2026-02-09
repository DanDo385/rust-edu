# Project 07 - Hashing Fundamentals (Educational SHA-256 Style)

## What You Will Build

You will build deterministic hashing helpers, nonce-based hashing, and a small proof-of-work style search (`find_hash_with_prefix`).

Pipeline:

1. Encode input bytes
2. Compute deterministic hash digest
3. Convert digest to hex
4. Search nonce space for prefix target

## Why This Lab Matters (First Principles)

Hashing is a one-way compression from arbitrary input to fixed-size output.

- Same input -> same output (determinism)
- Tiny input change -> very different output (avalanche behavior)
- Prefix search models proof-of-work cost

This project uses an educational hash helper, not production cryptography.

## Memory Model and Ownership

### High-level ownership flow

```text
&str input borrowed
   |
   | UTF-8 bytes read
   v
hash bytes computed into Vec<u8>
   |
   | hex conversion allocates String
   v
owned hash String returned
```

### Stack vs heap in this lab

- Stack:
  - input refs (`&str`), nonce counters (`u64`)
  - loop state for prefix search
- Heap:
  - hash byte buffer (`Vec<u8>`)
  - output hex string (`String`)

### Concrete memory sketch

```text
find_hash_with_prefix("test", "00")
Stack:
  nonce = 0..N
  prefix -> "00"
Heap per iteration:
  hash bytes (32 bytes)
  hex string (64 chars)
```

### Borrow checker behavior

- Input strings are borrowed; no caller ownership transfer.
- Returned hash strings are owned, lifetime-independent outputs.
- Prefix search mutates only local loop variables.

## Rust Mental Models in This Lab

- Immutability by default keeps hash verification deterministic.
- Mutability is explicit for nonce incrementation and buffers.
- Speed: byte-level operations and deterministic loops are CPU-friendly.
- Safety: no manual memory management while iterating millions of hashes.

## Symbol Deep Dive

### `&` and `&mut`

- `&str` parameters borrow text read-only.
- `&hash` in verification passes a shared borrow to avoid cloning.
- Misconception: borrowing hash text does not make it mutable.

### `*`

- `*` is arithmetic in this lab (e.g., derived mixing / multiplication).
- It is not pointer dereference in public API logic.

### Additional symbols used here

- `::` for constructors and associated functions
- `->` return type contracts (`String`, `(u64, String)`, `bool`)

## Exercises

1. `hash_string`
- Goal: hash input text and return hex digest.
- Constraints: deterministic output.
- Edge cases: empty string.
- Success: same input gives identical 64-char hash.

2. `hash_with_nonce`
- Goal: include nonce in hashing input.
- Constraints: different nonce should alter output.
- Edge cases: nonce `0`, large nonce.
- Success: tests detect nonce sensitivity.

3. `find_hash_with_prefix`
- Goal: brute-force nonce until hash starts with prefix.
- Constraints: must eventually terminate for simple prefixes.
- Edge cases: prefix length 1 or 2 in tests.
- Success: returned hash satisfies prefix and recomputes with nonce.

4. `verify_hash`
- Goal: compare expected hash with recomputed hash.
- Constraints: exact match required.
- Edge cases: wrong input, wrong hash.
- Success: valid/invalid verification tests pass.

## What Tests Prove

- Determinism tests prove pure hashing behavior.
- Nonce tests prove input-space expansion.
- Prefix tests prove proof-of-work style search correctness.
- Verify tests prove reproducible hash checking.

Failure interpretation:

- Non-deterministic output usually means hidden mutable state in hashing path.
- Prefix failures usually indicate nonce loop or string-prefix bug.

## Performance Notes

- Prefix search is unbounded brute force: expected cost grows exponentially with prefix length.
- Keep allocations predictable by building hashes directly from byte buffers.
- This educational hash is for learning only; real systems use secure primitives.

## How to Run

```bash
cargo run -p sha256-hashing
cargo test -p sha256-hashing
cargo check -p sha256-hashing
```
