# Project 31: LRU Cache

## Overview
Build a Least Recently Used (LRU) cache that evicts the least recently accessed item when capacity is reached. This project combines HashMap for O(1) lookups with a custom doubly-linked list for O(1) insertions/deletions, demonstrating advanced data structure design and interior mutability patterns in Rust.

## Concepts Taught
- **LRU caching strategy**: evicting least recently used items
- **HashMap + LinkedList combination**: achieving O(1) operations
- **Interior mutability**: using `RefCell` for mutability in shared contexts
- **Smart pointers**: `Rc<RefCell<T>>` for shared ownership
- **Option handling**: working with `Option<Rc<RefCell<Node>>>`
- **Custom data structures**: building a doubly-linked list in safe Rust
- **Generic types**: making the cache work with any key-value types

## Why LRU Cache Works

### The LRU Eviction Strategy
When memory is limited, we need to decide which items to keep:
- **LRU**: Remove the item accessed longest ago
- **Why?** Temporal locality - recently accessed items are likely to be accessed again
- **Real-world uses**: CPU caches, database buffers, web browser caches, CDNs

### The Data Structure Challenge
We need:
1. **O(1) lookup**: find value by key → HashMap
2. **O(1) insertion**: add new items → any data structure
3. **O(1) deletion**: remove least recently used → need to track order
4. **O(1) reordering**: move accessed items to front → doubly-linked list

**Solution**: HashMap + Doubly-Linked List
- HashMap: key → pointer to list node
- Doubly-Linked List: maintains access order (most recent at head)

## Why Rust Behaves This Way

### Interior Mutability with RefCell
In Rust, you normally can't mutate through a shared reference. But in a linked list, nodes need to be shared (multiple pointers) AND mutable (update next/prev).

**Solution**: `RefCell<T>` provides interior mutability
- Moves borrow checking from compile-time to runtime
- Allows mutation through shared references
- Panics if borrowing rules are violated at runtime

### Shared Ownership with Rc
Multiple nodes need to point to the same node (prev/next pointers), so we use `Rc<T>`:
- Reference counting: automatically frees when count reaches 0
- Enables shared ownership without garbage collection
- Combined with RefCell: `Rc<RefCell<Node>>` = shared, mutable

**Comparison with other languages:**
- **Python**: Garbage collector handles everything automatically
- **Go**: Garbage collector, simpler but with runtime overhead
- **C++**: `std::shared_ptr` + manual management, easy to leak memory
- **Rust**: Explicit `Rc<RefCell<T>>`, no GC, compile-time guarantees where possible

## Beginner Pitfalls & Borrow Checker Notes

### Pitfall 1: Forgetting to Upgrade Weak Pointers
```rust
let node = weak_ptr.upgrade();  // Returns Option<Rc<T>>
// Must handle None case!
```
**Why?** Weak pointers don't keep data alive. Use `Weak<T>` to break reference cycles.

### Pitfall 2: RefCell Runtime Panics
```rust
let borrow1 = node.borrow_mut();
let borrow2 = node.borrow_mut();  // ❌ PANIC: already borrowed
```
**Fix**: Don't hold borrows across operations. Scope them tightly.

### Pitfall 3: Reference Cycles
```rust
// head.next → node → node.prev → head
// Creates a cycle! Memory leaked if using only Rc
```
**Fix**: Use `Weak<T>` for backward pointers (prev) to break cycles.

### Pitfall 4: HashMap and Borrow Checker
```rust
let value = map.get(&key).unwrap();
map.insert(new_key, new_value);  // ❌ ERROR: can't mutate while borrowed
```
**Fix**: Clone or finish using `value` before modifying map.

## Code Walkthrough

See `src/main.rs` for a detailed, commented implementation that demonstrates:
1. Custom doubly-linked list node structure
2. LRU cache with get/put operations
3. HashMap + linked list integration
4. Interior mutability with `Rc<RefCell<T>>`
5. Handling evictions when at capacity
6. Moving accessed items to the front

## Performance Considerations

**Time Complexity:**
- `get(key)`: O(1) - HashMap lookup + list reordering
- `put(key, value)`: O(1) - HashMap insert + list operations
- Space: O(capacity)

**Why It's Fast:**
- HashMap gives O(1) average-case lookup
- Doubly-linked list gives O(1) insertion/deletion at any position
- No need to shift elements (unlike Vec)

**Memory Overhead:**
- Each node: 2 pointers (next/prev) + key + value + Rc overhead
- HashMap: keys + pointers to nodes + bucket overhead
- Typical overhead: ~48-64 bytes per entry (64-bit system)

**Real-World Optimizations:**
- Use `FxHashMap` instead of `HashMap` for faster hashing
- Pool allocations to reduce malloc overhead
- Use unsafe code with raw pointers (eliminates RefCell overhead)
- Consider `lru` crate for production use

## Comparison: Rust vs Go vs Python

| Feature | Rust | Go | Python |
|---------|------|----|----|
| LRU Implementation | Manual with Rc<RefCell<T>> | Manual or use container/list | `functools.lru_cache` decorator |
| Memory Safety | Compile-time (mostly) + RefCell runtime | Garbage collected | Garbage collected |
| Performance | Fastest, zero-cost abstractions | Fast, minor GC pauses | Slower, GC overhead |
| Ease of Implementation | Complex, requires understanding ownership | Moderate | Very easy |
| Memory Overhead | Low (no GC) | Medium (GC metadata) | Higher (object overhead + GC) |

## Additional Challenges

1. **Generic Implementation**: Make the cache work with any key/value types that implement appropriate traits.

2. **TTL Cache**: Add time-to-live expiration on top of LRU eviction.

3. **Thread-Safe LRU**: Use `Arc<Mutex<T>>` instead of `Rc<RefCell<T>>` for concurrent access.

4. **LFU Cache**: Implement Least Frequently Used instead of LRU.

5. **Benchmarking**: Compare your implementation against the `lru` crate.

6. **2Q Cache**: Implement a two-queue cache (combination of LRU and FIFO).

## Real-World Usage

LRU caches are everywhere:
- **Redis**: In-memory cache with LRU eviction
- **Memcached**: Distributed caching system
- **CPU caches**: L1/L2/L3 caches use LRU-like policies
- **Database buffers**: PostgreSQL, MySQL buffer pools
- **CDNs**: CloudFlare, Akamai cache content
- **Web browsers**: Cache images, scripts, stylesheets

## Running This Project

```bash
cd 31-lru-cache
cargo run
```

## Expected Output

You should see:
1. Cache initialization
2. Inserting items into the cache
3. Retrieving items (hits)
4. Evicting least recently used items when at capacity
5. Cache state at various points
6. Demonstration of access pattern affecting eviction order
