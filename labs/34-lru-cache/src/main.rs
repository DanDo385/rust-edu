// Lab 34: LRU Cache - Demo
//
// Implements a Least Recently Used (LRU) cache using HashMap + doubly-linked list.
// This demonstrates advanced Rust patterns: interior mutability, reference counting,
// and combining data structures for optimal performance.

use lru_cache::LruCache;

fn main() {
    println!("=== LRU Cache Implementation ===\n");

    // ============================================================================
    // BASIC LRU CACHE USAGE
    // ============================================================================

    println!("Creating LRU cache with capacity 3...");
    let mut cache = LruCache::new(3);
    println!();

    // Insert some values
    println!("=== Inserting Values ===");
    cache.put(1, "one");
    println!("Inserted: 1 -> one");
    cache.put(2, "two");
    println!("Inserted: 2 -> two");
    cache.put(3, "three");
    println!("Inserted: 3 -> three");
    println!("Order (MRU -> LRU): {:?}", cache.keys_mru_order());
    println!("Size: {}/{}", cache.len(), cache.capacity());
    println!();

    // Access a value (moves it to front)
    println!("=== Getting Values ===");
    match cache.get(&2) {
        Some(value) => println!("cache.get(2) = {} (moves 2 to front)", value),
        None => println!("cache.get(2) = None"),
    }
    println!("Order (MRU -> LRU): {:?}", cache.keys_mru_order());
    println!();

    // Insert beyond capacity (should evict LRU)
    println!("=== Testing Eviction ===");
    println!("Cache is at capacity ({}/{}). Inserting 4 -> four...", cache.len(), cache.capacity());
    cache.put(4, "four");
    println!("Least recently used item (1) should be evicted.");
    println!("Order (MRU -> LRU): {:?}", cache.keys_mru_order());
    println!();

    // Verify eviction
    println!("=== Verifying Eviction ===");
    match cache.get(&1) {
        Some(value) => println!("cache.get(1) = {} (shouldn't be here!)", value),
        None => println!("cache.get(1) = None (correctly evicted)"),
    }
    println!();

    // Remove demonstration
    println!("=== Remove ===");
    let removed = cache.remove(&3);
    println!("Removed key 3: {:?}", removed);
    println!("Size: {}/{}", cache.len(), cache.capacity());
    println!("Order (MRU -> LRU): {:?}", cache.keys_mru_order());
    println!();

    // Access pattern demonstration
    println!("=== Access Pattern Test ===");
    let mut cache2: LruCache<i32, &str> = LruCache::new(4);
    cache2.put(1, "A");
    cache2.put(2, "B");
    cache2.put(3, "C");
    cache2.put(4, "D");
    println!("Initial: {:?}", cache2.keys_mru_order());

    cache2.get(&2);
    println!("After get(2): {:?}", cache2.keys_mru_order());

    cache2.get(&4);
    println!("After get(4): {:?}", cache2.keys_mru_order());

    cache2.put(5, "E");
    println!("After put(5, E) (evicts LRU=1): {:?}", cache2.keys_mru_order());

    println!("\n=== LRU Cache Demo Complete ===");
}

// ============================================================================
// WHAT RUST DOES UNDER THE HOOD
// ============================================================================
// 1. REFERENCE COUNTING (Rc)
//    - Each Rc<T> maintains a reference count
//    - When count reaches 0, the data is freed
//    - This is NOT garbage collection - it's deterministic
//    - Small overhead: 2 extra usizes per Rc (strong + weak counts)
//
// 2. INTERIOR MUTABILITY (RefCell)
//    - Moves borrow checking from compile-time to runtime
//    - Maintains a borrow counter at runtime
//    - Panics if you violate borrowing rules (multiple mutable borrows)
//    - Small overhead: 1 usize for borrow state
//
// 3. COMBINED Rc<RefCell<T>>
//    - Allows multiple owners (Rc) of mutable data (RefCell)
//    - Total overhead: ~24 bytes on 64-bit systems
//    - This is the "safe" way to implement linked structures in Rust
//
// 4. HASHMAP PERFORMANCE
//    - Uses SipHash by default (cryptographically secure but slower)
//    - O(1) average case lookup/insert
//    - Grows dynamically, may need to rehash

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. LRU cache = HashMap + doubly-linked list
// 2. HashMap provides O(1) lookup by key
// 3. Linked list maintains access order (MRU at head, LRU at tail)
// 4. All operations (get, put, evict) are O(1)
// 5. Rc<RefCell<T>> enables shared, mutable ownership
// 6. RefCell moves borrow checking to runtime (small cost)
// 7. Interior mutability is necessary for linked structures
// 8. Real-world caches use this pattern everywhere
