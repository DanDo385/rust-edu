# Project 09 - Simple Blockchain

## What You Will Build

You will implement a minimal blockchain with blocks, hash linking, mining difficulty, append operations, and chain validation.

Pipeline:

1. Create genesis block
2. Append block with previous hash link
3. Mine nonce until difficulty prefix is satisfied
4. Validate chain integrity end-to-end

## Why This Lab Matters (First Principles)

A blockchain is an append-only linked log secured by hashes.

- Each block commits to prior block hash.
- Tampering one block invalidates downstream links.
- Proof-of-work increases rewrite cost.

## Memory Model and Ownership

### High-level ownership flow

```text
Blockchain owns Vec<Block>
   |
   | add_block(data: String) takes ownership of data
   v
new Block stores owned fields + mined hash
   |
   | pushed into chain vector
   v
chain validation borrows blocks immutably
```

### Stack vs heap in this lab

- Stack:
  - block metadata (`index`, `timestamp`, `nonce`)
  - local validation variables
- Heap:
  - `Vec<Block>` backing buffer
  - `String` block data and hashes

### Concrete memory sketch

```text
chain[0] (genesis): hash = H0
chain[1]: previous_hash = H0, hash = H1
chain[2]: previous_hash = H1, hash = H2

If chain[1].data changes, recalculated hash != stored hash,
then chain[2].previous_hash no longer matches.
```

### Borrow checker behavior

- `add_block(&mut self, data: String)` requires exclusive mutable borrow of chain.
- `is_valid(&self)` uses shared borrow for read-only verification.
- `mine(&mut self, difficulty)` mutates nonce/hash in place on a single block.

## Rust Mental Models in This Lab

- Immutability by default helps keep historic blocks conceptually stable.
- Mutability is explicit where mining/appending state changes occur.
- Speed: contiguous `Vec<Block>` traversal is cache-friendly for validation.
- Safety: no dangling links because block storage is owned by chain vector.

## Symbol Deep Dive

### `&` and `&mut`

- `&mut self` in mining/addition grants exclusive write access.
- `&self` validation proves read-only traversal.
- Misconception: `&mut self` does not allow multiple mutable aliases; borrow checker prevents that.

### `*`

- `*` is arithmetic (difficulty/math helpers) when present.
- No pointer dereference needed in core chain logic.

### Additional symbols used here

- `::` for constructors (`Block::new`, `Blockchain::new`)
- `->` return contracts

## Exercises

1. `Block::new` and `calculate_hash`
- Goal: initialize block and compute deterministic hash.
- Constraints: include all relevant fields.
- Edge cases: empty data.
- Success: hash changes when fields change.

2. `Block::mine`
- Goal: increment nonce until hash matches difficulty prefix.
- Constraints: deterministic prefix check.
- Edge cases: difficulty 0/1/2.
- Success: mined hash starts with required zeros.

3. `Blockchain::new` and `add_block`
- Goal: create genesis then append mined blocks with correct linking.
- Constraints: `previous_hash` must match prior block hash.
- Edge cases: first append after genesis.
- Success: chain length/index progression correct.

4. `Blockchain::is_valid`
- Goal: verify hashes, links, and (optionally) difficulty rule.
- Constraints: full-chain scan.
- Edge cases: tampered data or broken links.
- Success: detects corruption.

## What Tests Prove

- Creation tests prove genesis initialization.
- Append tests prove ownership flow and indexing.
- Valid/tampered tests prove integrity checks.
- Mining tests prove proof-of-work constraints are enforced.

Failure interpretation:

- False-valid chain usually means missing hash recomputation checks.
- Mining failure usually means nonce loop or prefix logic bug.

## Performance Notes

- Append is amortized O(1) plus mining cost.
- Validation is O(n) over chain length.
- Mining dominates runtime as difficulty increases.

## How to Run

```bash
cargo run -p simple-blockchain
cargo test -p simple-blockchain
cargo check -p simple-blockchain
```
