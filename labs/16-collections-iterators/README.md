# Project 13: Collections and Iterators

## Overview
This project explores Rust's standard library collections (Vec, HashMap, HashSet, String) and the powerful iterator system. You'll build a word frequency counter that reads a file and demonstrates collection operations, iterator chains, and zero-cost abstractions. This is essential knowledge for any Rust programmer!

## Concepts Taught
- **Vec<T>** - Growable arrays (dynamic arrays)
- **HashMap<K, V>** - Key-value stores (hash tables)
- **HashSet<T>** - Unique value sets
- **String** and **str** operations
- **Iterator trait** and iterator adaptors
- **Iterator methods**: map, filter, fold, collect, etc.
- **Iterator chains** (functional programming style)
- **Zero-cost abstractions** - performance characteristics
- **Entry API** for efficient HashMap operations
- **Ownership** with collections

## Why Rust Behaves This Way

### Collections and Ownership
Collections in Rust own their data. When you add something to a Vec or HashMap:
1. **Ownership transfers** to the collection (for non-Copy types)
2. The collection manages **heap allocation**
3. When the collection is dropped, **all contents are freed**

This is different from languages with garbage collection:
```rust
let v = vec![String::from("hello")];
// The Vec now OWNS the String
// When v is dropped, the String is freed automatically
```

### Zero-Cost Abstractions
Rust's iterators are a **zero-cost abstraction**:
- Iterator chains look like high-level functional code
- But compile down to the **same assembly as hand-written loops**
- No runtime overhead, no virtual dispatch, no allocations

Example:
```rust
// High-level, functional style
data.iter()
    .filter(|x| x % 2 == 0)
    .map(|x| x * 2)
    .sum()

// Compiles to the same code as:
let mut sum = 0;
for x in &data {
    if x % 2 == 0 {
        sum += x * 2;
    }
}
```

## Beginner Pitfalls & Borrow Checker Notes

### Pitfall 1: Moving Values into Collections
```rust
let s = String::from("hello");
let mut v = Vec::new();
v.push(s);  // s moved into v
println!("{}", s);  // ❌ ERROR: s was moved!
```
**Fix**: Clone if you need to keep the original:
```rust
let s = String::from("hello");
let mut v = Vec::new();
v.push(s.clone());  // Clone s
println!("{}", s);  // ✅ OK: s still valid
```

### Pitfall 2: Cannot Modify While Iterating
```rust
let mut v = vec![1, 2, 3];
for x in &v {
    v.push(4);  // ❌ ERROR: cannot borrow v as mutable while iterating
}
```
**Fix**: Collect to a new vector or use indices:
```rust
let v = vec![1, 2, 3];
let v: Vec<_> = v.into_iter().chain(std::iter::once(4)).collect();
```

### Pitfall 3: HashMap Entry API Confusion
```rust
// ❌ INEFFICIENT - multiple lookups
if !map.contains_key(&key) {
    map.insert(key, 0);
}
map.get_mut(&key).unwrap() += 1;
```
**Fix**: Use the Entry API (one lookup):
```rust
// ✅ EFFICIENT
*map.entry(key).or_insert(0) += 1;
```

### Pitfall 4: Collecting Without Type Annotations
```rust
let numbers = vec![1, 2, 3];
let doubled = numbers.iter().map(|x| x * 2);  // ❌ Does nothing!
```
**Fix**: Iterators are lazy - must consume them:
```rust
let doubled: Vec<_> = numbers.iter().map(|x| x * 2).collect();  // ✅
```

### Pitfall 5: String Indexing
```rust
let s = String::from("hello");
let c = s[0];  // ❌ ERROR: cannot index String
```
**Fix**: Strings are UTF-8, not ASCII arrays:
```rust
let c = s.chars().nth(0);  // ✅ Returns Option<char>
// Or use slices (careful with UTF-8!):
let slice = &s[0..1];  // ✅ Returns &str
```

## Code Walkthrough

See `src/main.rs` for a complete word frequency counter that demonstrates:
1. Reading files with error handling
2. String manipulation and splitting
3. HashMap operations with Entry API
4. Iterator chains for data processing
5. Sorting and displaying results
6. Performance characteristics of different approaches

## Performance Considerations

### Vec Performance
- **Amortized O(1)** push (reallocates when capacity exceeded)
- **O(1)** access by index
- **O(n)** insertion/deletion in middle
- Prefer `Vec::with_capacity(n)` if you know size

### HashMap Performance
- **Average O(1)** insert, get, remove
- **Worst case O(n)** (hash collisions)
- Uses SipHash by default (slower but DoS-resistant)
- Can use `HashMap::with_capacity(n)` for efficiency

### HashSet Performance
- Same as HashMap (it's a HashMap with `()` values)
- **O(1)** average insert, contains, remove

### Iterator Performance
- **Zero cost** - compiled to same code as manual loops
- Lazy evaluation - no work done until consumed
- Can be optimized by LLVM (sometimes faster than manual loops!)

### String Performance
- UTF-8 encoded (1-4 bytes per character)
- No O(1) indexing (must iterate to find character)
- Concatenation with `+` moves left operand
- Use `format!` or `String::push_str` for multiple concatenations

## Comparison: Rust vs Go vs Python

| Feature | Rust | Go | Python |
|---------|------|----|----|
| Dynamic array | `Vec<T>` | `[]T` (slice) | `list` |
| Hash map | `HashMap<K, V>` | `map[K]V` | `dict` |
| Hash set | `HashSet<T>` | `map[T]bool` (idiom) | `set` |
| Iterators | Zero-cost, lazy | Range-based loops | Generators, lazy |
| String indexing | No (UTF-8) | Yes (rune access) | Yes (but slow) |
| Ownership | Moved into collection | Copied/referenced | Always referenced |
| Performance | Zero-cost abstractions | GC overhead | Interpreted overhead |
| Type safety | Fully generic | Generic (1.18+) | Duck typed |

## Iterator Adaptors Reference

**Transformers**:
- `map(f)` - Transform each element
- `filter(p)` - Keep elements matching predicate
- `filter_map(f)` - Map and filter in one step
- `flat_map(f)` - Map and flatten nested iterators
- `flatten()` - Flatten nested iterators

**Consumers**:
- `collect()` - Gather into a collection
- `fold(init, f)` - Reduce to a single value
- `reduce(f)` - Like fold but no initial value
- `sum()` - Sum all elements
- `product()` - Multiply all elements
- `count()` - Count elements
- `any(p)` / `all(p)` - Boolean predicates
- `find(p)` - Find first matching element
- `position(p)` - Find index of element

**Combinators**:
- `chain(other)` - Concatenate iterators
- `zip(other)` - Pair elements from two iterators
- `enumerate()` - Add indices
- `take(n)` / `skip(n)` - Limit iteration
- `take_while(p)` / `skip_while(p)` - Conditional limits

## Additional Challenges

1. **Inverted Index**: Build an inverted index mapping words to document IDs (Vec of file paths)

2. **Top K Elements**: Find the K most frequent words using a binary heap (BinaryHeap)

3. **Deduplication**: Remove duplicate lines from a file while preserving order (Vec + HashSet)

4. **Anagram Groups**: Group words that are anagrams of each other (HashMap with sorted chars as key)

5. **Iterator Implementation**: Implement a custom iterator for Fibonacci numbers

6. **Performance Comparison**: Compare iterator chains vs manual loops vs functional style with benchmarks

## Future Directions

- **Next**: Deep dive into closures and zero-cost abstractions (Project 14)
- **Later**: Build more complex data structures (Project 28 - Key-Value Store)
- **Advanced**: Parallel iterators with Rayon (Project 32)

## Running This Project

```bash
cd 13-collections-iterators
cargo run
```

The program will:
1. Read its own source code as input
2. Count word frequencies
3. Display the top 20 most common words
4. Demonstrate various collection and iterator operations

## Expected Output

You should see:
- Word frequency analysis of the source file
- Top most common words with counts
- Demonstrations of Vec, HashMap, HashSet operations
- Iterator chain examples
- Performance comparisons between different approaches
