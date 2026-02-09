# Project 60 - Simple Blockchain (Alternative Track)

## What You Will Build

You will implement a compact blockchain engine with:

1. Block creation (`new`, `genesis`)
2. Hash calculation
3. Proof-of-work mining
4. Chain append and validation

## Why This Lab Matters (First Principles)

This lab combines multiple CS ideas into one system:

- Hash-linked data structures
- Costly mutation via proof-of-work
- Invariant checking over append-only history

It is a systems-model exercise in data integrity and deterministic computation.

## Memory Model and Ownership

### High-level ownership flow

```text
Blockchain owns Vec<Block>
   |
   | add_block(data: String) moves owned data into new block
   v
block mined via &mut self methods
   |
   | appended to chain
   v
is_valid(&self) scans immutable borrows of blocks
```

### Stack vs heap in this lab

- Stack:
  - numeric metadata and loop state
  - references during validation
- Heap:
  - chain vector storage
  - block data/hash strings

### Concrete memory sketch

```text
Block i:
  previous_hash = hash(Block i-1)

Validation checks:
  1) block.hash == block.calculate_hash()
  2) block.previous_hash == previous block hash
  3) hash satisfies difficulty prefix
```

### Borrow checker behavior

- `mine(&mut self, difficulty)` mutates one block's nonce/hash.
- `latest_block(&self) -> &Block` returns borrowed reference tied to chain lifetime.
- `add_block(&mut self, data: String)` requires exclusive mutable chain access.

## Rust Mental Models in This Lab

- Immutability by default supports mental model of historical ledger entries.
- Mutability is explicit during block creation/mining only.
- Speed: validation is linear and data-local in `Vec<Block>`.
- Safety: ownership guarantees prevent dangling references to blocks.

## Symbol Deep Dive

### `&` and `&mut`

- `&mut self` methods perform controlled mutation.
- `&self` methods expose read-only views and validation.
- Misconception: returning `&Block` from `latest_block` does not copy block data.

### `*`

- `*` appears as arithmetic in helper math when present.
- Core blockchain logic is hash/link validation, not pointer dereference.

### Additional symbols used here

- `::` for constructors (`Block::genesis`, `Blockchain::new`)
- `->` explicit return types for ownership and borrowing semantics

## Exercises

1. Block constructors + hashing
- Goal: initialize valid block and compute deterministic hash.
- Constraints: include key fields in hash material.
- Edge cases: genesis block special case.
- Success: genesis/hash tests pass.

2. Mining
- Goal: adjust nonce until difficulty target met.
- Constraints: prefix checking consistency.
- Edge cases: low difficulty, repeated mining attempts.
- Success: mined block hashes satisfy target.

3. Chain growth
- Goal: append properly linked blocks.
- Constraints: correct index and previous-hash linkage.
- Edge cases: first block after genesis.
- Success: add-block tests pass.

4. Validation
- Goal: detect tampering and broken links.
- Constraints: full scan of chain invariants.
- Edge cases: mutated block data post-mining.
- Success: tamper-detection tests pass.

## What Tests Prove

- Genesis tests prove deterministic chain start.
- Append tests prove ownership + linkage correctness.
- Validity tests prove invariant preservation and tamper detection.

Failure interpretation:

- False-valid tampered chain indicates missing recalculation/link checks.
- Add-block mismatch indicates index or previous-hash assignment bug.

## Performance Notes

- Append is amortized O(1) plus mining work.
- Validation is O(n) over blocks.
- Mining cost grows sharply with difficulty and dominates runtime.

## How to Run

```bash
cargo run -p simple-blockchain-alternative
cargo test -p simple-blockchain-alternative
cargo check -p simple-blockchain-alternative
```
