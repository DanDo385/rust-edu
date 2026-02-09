# Project 03 - Collections Basics

## What You Will Build

You will build a small text-and-number processing toolkit using `Vec<i32>` and `HashMap<String, usize>`. The code path mirrors a real data pipeline:

1. Read borrowed input (`&[i32]`, `&str`)
2. Transform/filter/count
3. Return owned output (`Vec<i32>`, `HashMap<String, usize>`, `Option<String>`)

## Why This Lab Matters (First Principles)

Collections are about memory layout and access patterns, not just syntax.

- `Vec<T>` gives contiguous heap storage with cache-friendly iteration.
- `HashMap<K, V>` trades ordering for average O(1) lookup.
- Borrowed input avoids unnecessary heap clones.
- Owned output defines clear lifetime boundaries.

## Memory Model and Ownership

### High-level ownership flow

```text
Caller owns data
   |
   | borrow (&[i32], &str)
   v
Functions read/process without taking ownership
   |
   | allocate new output when needed
   v
Return owned Vec / HashMap / Option<String>
```

### Stack vs heap in this lab

- Stack:
  - Slice references (`&[i32]`), lengths, loop counters
  - `Option<String>` tag (`Some` or `None`)
- Heap:
  - `Vec<i32>` elements
  - `HashMap` buckets
  - `String` keys/values

### Concrete memory sketch

```text
Stack frame: most_common_word(text: &str)
+------------------------------+
| text ptr -> "the cat the"    |
| text len = 11                |
| freq: HashMap handle         |
+------------------------------+
              |
              v
Heap:
  buckets -> ["the" => 2, "cat" => 1]
```

### Borrow checker behavior

- `&str` inputs are shared borrows: many readers, no mutation.
- You can create owned keys with `to_lowercase()`/`to_string()` when inserting into `HashMap`.
- Returning owned data prevents dangling references.

## Rust Mental Models in This Lab

- Immutability by default: iteration variables are immutable unless `mut` is explicitly needed.
- Mutability is explicit: `let mut map = HashMap::new()` means this binding may change.
- Speed: contiguous `Vec` iteration and single-pass counting are efficient.
- Safety: no nulls, no out-of-bounds writes, no use-after-free.

## Symbol Deep Dive

### `&` and `&mut`

- `&[i32]` and `&str` are shared borrows of caller-owned data.
- You pass an address + metadata, not a copied collection.
- Misconception: `&` does not mean "copy everything by reference forever". It creates a scoped borrow.

### `*`

- `*` is not needed in this lab's public API for dereference.
- If you see `*` in numeric code, verify if it means multiplication, not pointer dereference.

### Additional symbols used here

- `::` for paths like `HashMap::new`
- `->` for return contracts
- `|x|` closure syntax in iterator chains

## Exercises

1. `sum_of_evens` (`src/lib.rs`)
- Goal: sum only even values.
- Constraints: single pass preferred.
- Edge cases: empty input, all odd, negatives.
- Success: integration tests return exact sums.

2. `word_frequency` (`src/lib.rs`)
- Goal: count case-insensitive word occurrences.
- Constraints: normalize consistently.
- Edge cases: empty string, multiple spaces.
- Success: correct counts and key set.

3. `filter_and_sort` (`src/lib.rs`)
- Goal: keep values in `[min, max]`, return sorted ascending.
- Constraints: do not mutate caller slice.
- Edge cases: no matches, already sorted input.
- Success: deterministic sorted output.

4. `most_common_word` (`src/lib.rs`)
- Goal: return the highest-frequency word.
- Constraints: return `None` for empty input.
- Edge cases: ties (tests allow either max word), case normalization.
- Success: `Some(word)` or `None` exactly as expected.

## What Tests Prove

- Even-sum tests prove filtering logic and no off-by-one errors.
- Frequency tests prove normalization and hash-map counting invariants.
- Sorting tests prove range filtering and output ordering.
- Most-common tests prove optional return behavior and tie handling.

Failure interpretation:

- Frequency mismatch usually means normalization/splitting bug.
- Wrong ordering means sort step is missing or unstable.

## Performance Notes

- `Vec` scan is O(n); sorting filtered output is O(k log k).
- `HashMap` counting is average O(n) over words.
- Borrowed inputs avoid unnecessary cloning of entire datasets.

## How to Run

```bash
cargo run -p collections-basics
cargo test -p collections-basics
cargo check -p collections-basics
```
