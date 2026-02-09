# Project 08 - Merkle Tree Basics

## What You Will Build

You will construct a Merkle tree from leaf data and compute a deterministic root hash. This models how blockchains summarize many records into one commitment.

## Why This Lab Matters (First Principles)

A Merkle tree is a binary reduction over hashes:

- Leaves represent individual records
- Parents hash child pairs
- Root commits to all leaves

Changing one leaf changes the root, enabling efficient tamper detection.

## Memory Model and Ownership

### High-level ownership flow

```text
Vec<String> data (owned input)
   |
   | hash each leaf
   v
Vec<String> leaves + intermediate levels
   |
   | combine via hash_pair
   v
root String stored in MerkleTree
```

### Stack vs heap in this lab

- Stack:
  - indices, loop variables, references
  - `MerkleTree` handle fields
- Heap:
  - leaf hash strings
  - intermediate level vectors
  - root hash string bytes

### Concrete memory sketch

```text
Leaves: [h(A), h(B), h(C), h(D)]
Level1: [h(h(A)|h(B)), h(h(C)|h(D))]
Root:   [h(level1[0]|level1[1])]

Stack stores Vec handles; hash strings live on heap.
```

### Borrow checker behavior

- `MerkleTree::new(data: Vec<String>)` takes ownership of input vector.
- `root_hash(&self) -> &str` returns borrowed view into owned root string.
- `hash_pair(left: &str, right: &str)` borrows child hashes without cloning by default.

## Rust Mental Models in This Lab

- Immutability by default keeps each level deterministic after construction.
- Mutability is explicit for working vectors while building levels.
- Speed: linear hashing per level; total O(n) hashing work for fixed-size branches.
- Safety: no manual pointer logic in tree construction.

## Symbol Deep Dive

### `&` and `&mut`

- `&self` in `root_hash` provides read-only access to internal state.
- `&str` in `hash_pair` is borrowed text, not ownership transfer.
- Misconception: returning `&str` does not allocate; it borrows existing string bytes.

### `*`

- `*` is arithmetic/multiplication only if used in helper math.
- Merkle pairing logic itself is about concatenation + hashing, not pointer dereference.

### Additional symbols used here

- `::` for associated constructor `MerkleTree::new`
- `->` for explicit ownership/borrowed return contracts

## Exercises

1. `MerkleTree::new`
- Goal: hash input data and build parent levels to a single root.
- Constraints: deterministic ordering.
- Edge cases: one item, odd number of leaves.
- Success: roots are stable and leaf count preserved.

2. `root_hash`
- Goal: return root as borrowed `&str`.
- Constraints: no cloning required.
- Edge cases: single-leaf trees.
- Success: returned root matches struct field.

3. `hash_pair`
- Goal: hash ordered pair `(left, right)`.
- Constraints: order-sensitive.
- Edge cases: identical children, swapped order.
- Success: deterministic and order-aware tests pass.

## What Tests Prove

- Single/two/four-item tests prove tree build correctness across sizes.
- Same-data/different-data tests prove root determinism and tamper sensitivity.
- Pair tests prove order-dependent hashing.

Failure interpretation:

- Root instability usually means inconsistent concatenation or ordering.
- Leaf count mismatch means construction loop incorrectly drops/duplicates nodes.

## Performance Notes

- Building tree is O(n) hash operations for fixed fan-out.
- Memory overhead is additional vectors for each level.
- Borrowed accessors avoid extra root-string allocations.

## How to Run

```bash
cargo run -p merkle-tree
cargo test -p merkle-tree
cargo check -p merkle-tree
```
