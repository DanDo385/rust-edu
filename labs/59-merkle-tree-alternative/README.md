# Project 59 - Merkle Tree with Proofs (Alternative Track)

## What You Will Build

You will build a Merkle tree that supports:

1. Leaf/root computation
2. Node/leaf metadata queries
3. Inclusion proof generation
4. Inclusion proof verification

## Why This Lab Matters (First Principles)

Merkle proofs let you verify membership in large datasets with logarithmic proof size.

- Root commits to all data.
- Proof path contains sibling hashes only.
- Verification recomputes root without full dataset.

## Memory Model and Ownership

### High-level ownership flow

```text
input &[&str]
   |
   | hash leaves -> Vec<String>
   v
build parent levels -> nodes Vec<String>
   |
   | store root + leaves + nodes in MerkleTree
   v
generate_proof borrows tree and returns owned proof vector
```

### Stack vs heap in this lab

- Stack:
  - indices, booleans (`is_left` flags), loop cursors
- Heap:
  - hash strings
  - full node arrays
  - proof vector `Vec<(String, bool)>`

### Concrete memory sketch

```text
Data: [a, b, c, d]
Leaves: [ha, hb, hc, hd]
L1:     [hab, hcd]
Root:   [hr]

Proof for b:
  [(ha, false), (hcd, true)]
(false = sibling on left, true = sibling on right)
```

### Borrow checker behavior

- `new(_data: &[&str])` borrows input strings; tree stores owned hashes.
- `root(&self) -> &str` and `leaves(&self) -> &[String]` return borrowed views.
- Proof generation borrows tree immutably; no in-place mutation required.

## Rust Mental Models in This Lab

- Immutability by default fits append-once tree construction.
- Mutability is explicit while assembling levels/proof vectors.
- Speed: proof verification is O(log n) hash combinations.
- Safety: tree internals are owned; returned borrows cannot outlive tree.

## Symbol Deep Dive

### `&` and `&mut`

- `&str` inputs are read-only source data.
- `&self` query methods expose safe, borrowed views.
- Misconception: returning `&[String]` does not clone all leaves.

### `*`

- `*` may appear as arithmetic in index calculations; not pointer logic.
- Dereference is mostly implicit through references in string operations.

### Additional symbols used here

- `::` for associated methods (`MerkleTree::verify_proof`)
- `->` return contracts including `Option<Vec<...>>`

## Exercises

1. Hash helpers (`hash_bytes`, `hash_string`, `hash_pair`)
- Goal: deterministic and order-sensitive hashing.
- Constraints: consistent encoding/hex format.
- Edge cases: empty input, swapped pair order.
- Success: hash tests pass.

2. Tree construction and metadata
- Goal: build root/leaves/nodes correctly.
- Constraints: handle odd leaf counts.
- Edge cases: empty tree, single leaf.
- Success: node/leaf/root tests pass.

3. Proof generation
- Goal: return sibling path for target leaf.
- Constraints: out-of-bounds index returns `None`.
- Edge cases: two-leaf and odd-leaf trees.
- Success: proof generation tests pass.

4. Proof verification
- Goal: recompute root from leaf + proof.
- Constraints: ordering by sibling side must be correct.
- Edge cases: wrong data or wrong root fails.
- Success: verify tests pass across all leaves.

## What Tests Prove

- Construction tests prove deterministic tree shape and root integrity.
- Integrity tests prove tamper/reorder detection.
- Proof tests prove membership checking without full tree traversal.

Failure interpretation:

- Verification false negatives usually mean sibling-order bug.
- Node-count mismatch suggests incorrect level aggregation.

## Performance Notes

- Tree build is O(n) hashes for fixed fan-out.
- Proof generation and verification are O(log n).
- Storing all nodes trades memory for simpler proof generation.

## How to Run

```bash
cargo run -p merkle-tree-alternative
cargo test -p merkle-tree-alternative
cargo check -p merkle-tree-alternative
```
