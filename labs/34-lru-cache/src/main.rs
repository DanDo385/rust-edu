// Project 31: LRU Cache
//
// Implements a Least Recently Used (LRU) cache using HashMap + doubly-linked list.
// This demonstrates advanced Rust patterns: interior mutability, reference counting,
// and combining data structures for optimal performance.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

fn main() {
    println!("=== LRU Cache Implementation ===\n");

    // ============================================================================
    // BASIC LRU CACHE USAGE
    // ============================================================================

    println!("Creating LRU cache with capacity 3...");
    let mut cache = LRUCache::new(3);
    println!();

    // Insert some values
    println!("=== Inserting Values ===");
    cache.put(1, "one");
    println!("Inserted: 1 -> one");
    cache.put(2, "two");
    println!("Inserted: 2 -> two");
    cache.put(3, "three");
    println!("Inserted: 3 -> three");
    cache.print_state();
    println!();

    // Access a value (moves it to front)
    println!("=== Getting Values ===");
    match cache.get(&2) {
        Some(value) => println!("cache.get(2) = {} (moves 2 to front)", value),
        None => println!("cache.get(2) = None"),
    }
    cache.print_state();
    println!();

    // Insert beyond capacity (should evict LRU)
    println!("=== Testing Eviction ===");
    println!("Cache is at capacity (3/3). Inserting 4 -> four...");
    cache.put(4, "four");
    println!("Least recently used item (1) should be evicted.");
    cache.print_state();
    println!();

    // Verify eviction
    println!("=== Verifying Eviction ===");
    match cache.get(&1) {
        Some(value) => println!("cache.get(1) = {} (shouldn't be here!)", value),
        None => println!("cache.get(1) = None (correctly evicted)"),
    }
    println!();

    // Access pattern demonstration
    println!("=== Access Pattern Test ===");
    let mut cache2 = LRUCache::new(4);
    cache2.put(1, "A");
    cache2.put(2, "B");
    cache2.put(3, "C");
    cache2.put(4, "D");
    println!("Initial state:");
    cache2.print_state();

    println!("\nAccessing key 2 (moves to front):");
    cache2.get(&2);
    cache2.print_state();

    println!("\nAccessing key 4 (moves to front):");
    cache2.get(&4);
    cache2.print_state();

    println!("\nInserting key 5 -> E (should evict 1, the LRU):");
    cache2.put(5, "E");
    cache2.print_state();

    println!("\n=== LRU Cache Demo Complete ===");
}

// ============================================================================
// DOUBLY-LINKED LIST NODE
// ============================================================================
// Each node stores a key-value pair and pointers to next/prev nodes.
// We use Rc<RefCell<>> to enable shared ownership AND mutability.

#[derive(Debug)]
struct Node<K, V> {
    key: K,
    value: V,
    prev: Option<Rc<RefCell<Node<K, V>>>>,
    next: Option<Rc<RefCell<Node<K, V>>>>,
}

impl<K, V> Node<K, V> {
    fn new(key: K, value: V) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            key,
            value,
            prev: None,
            next: None,
        }))
    }
}

// ============================================================================
// LRU CACHE STRUCTURE
// ============================================================================
// Combines HashMap (for O(1) lookup) with doubly-linked list (for O(1) reordering).
//
// Structure:
// - HashMap: key -> pointer to node in linked list
// - Linked list: maintains access order (most recent at head)
// - Capacity: maximum number of items to store

struct LRUCache<K, V> {
    capacity: usize,
    map: HashMap<K, Rc<RefCell<Node<K, V>>>>,
    head: Option<Rc<RefCell<Node<K, V>>>>,  // Most recently used
    tail: Option<Rc<RefCell<Node<K, V>>>>,  // Least recently used
}

impl<K, V> LRUCache<K, V>
where
    K: std::hash::Hash + Eq + Clone + std::fmt::Display,
    V: Clone + std::fmt::Display,
{
    /// Creates a new LRU cache with the specified capacity
    fn new(capacity: usize) -> Self {
        LRUCache {
            capacity,
            map: HashMap::new(),
            head: None,
            tail: None,
        }
    }

    /// Gets a value from the cache
    /// Returns None if key doesn't exist
    /// Moves accessed item to the front (most recently used)
    fn get(&mut self, key: &K) -> Option<V> {
        // Check if key exists in HashMap
        if let Some(node) = self.map.get(key) {
            let node = Rc::clone(node);

            // Move this node to the front (it's now most recently used)
            self.remove_from_list(Rc::clone(&node));
            self.push_front(Rc::clone(&node));

            // Return the value
            Some(node.borrow().value.clone())
        } else {
            None
        }
    }

    /// Inserts a key-value pair into the cache
    /// If key exists, updates value and moves to front
    /// If cache is at capacity, evicts least recently used item
    fn put(&mut self, key: K, value: V) {
        // If key already exists, update it
        if let Some(node) = self.map.get(&key) {
            let node = Rc::clone(node);

            // Update value
            node.borrow_mut().value = value;

            // Move to front (most recently used)
            self.remove_from_list(Rc::clone(&node));
            self.push_front(Rc::clone(&node));

            return;
        }

        // Check if we need to evict (at capacity and inserting new key)
        if self.map.len() >= self.capacity {
            self.evict_lru();
        }

        // Create new node and add to front
        let new_node = Node::new(key.clone(), value);
        self.push_front(Rc::clone(&new_node));

        // Add to HashMap
        self.map.insert(key, new_node);
    }

    /// Removes the least recently used item (tail of the list)
    fn evict_lru(&mut self) {
        if let Some(tail) = self.tail.take() {
            // Remove from HashMap
            let key = tail.borrow().key.clone();
            self.map.remove(&key);

            // Update tail to previous node
            let new_tail = tail.borrow().prev.clone();

            if let Some(new_tail_node) = new_tail {
                new_tail_node.borrow_mut().next = None;
                self.tail = Some(new_tail_node);
            } else {
                // List is now empty
                self.head = None;
            }
        }
    }

    /// Adds a node to the front of the list (most recently used position)
    fn push_front(&mut self, node: Rc<RefCell<Node<K, V>>>) {
        match self.head.take() {
            Some(old_head) => {
                // Connect new node to old head
                old_head.borrow_mut().prev = Some(Rc::clone(&node));
                node.borrow_mut().next = Some(Rc::clone(&old_head));
                node.borrow_mut().prev = None;

                self.head = Some(node);
            }
            None => {
                // List is empty, this is first node
                node.borrow_mut().prev = None;
                node.borrow_mut().next = None;

                self.tail = Some(Rc::clone(&node));
                self.head = Some(node);
            }
        }
    }

    /// Removes a node from its current position in the list
    /// (Does not remove from HashMap or deallocate)
    fn remove_from_list(&mut self, node: Rc<RefCell<Node<K, V>>>) {
        let prev = node.borrow().prev.clone();
        let next = node.borrow().next.clone();

        // Update previous node's next pointer
        match prev {
            Some(prev_node) => {
                prev_node.borrow_mut().next = next.clone();
            }
            None => {
                // This was the head
                self.head = next.clone();
            }
        }

        // Update next node's prev pointer
        match next {
            Some(next_node) => {
                next_node.borrow_mut().prev = prev;
            }
            None => {
                // This was the tail
                self.tail = prev;
            }
        }
    }

    /// Prints current cache state (for debugging/demonstration)
    fn print_state(&self) {
        print!("Cache state (MRU -> LRU): ");

        let mut current = self.head.clone();
        let mut items = Vec::new();

        while let Some(node) = current {
            let borrowed = node.borrow();
            items.push(format!("{}:{}", borrowed.key, borrowed.value));
            current = borrowed.next.clone();
        }

        if items.is_empty() {
            println!("[empty]");
        } else {
            println!("[{}] ({}/{})", items.join(" -> "), self.map.len(), self.capacity);
        }
    }
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
//
// 5. MEMORY LAYOUT
//    - Each node: ~48-64 bytes (key, value, 2 Rc pointers, RefCell overhead)
//    - HashMap: ~32 bytes per entry + bucket overhead
//    - Total: ~80-100 bytes per cached item
//
// 6. NO GARBAGE COLLECTION
//    - All memory is deterministically freed
//    - When LRU evicts, node is dropped immediately
//    - No GC pauses or stop-the-world events

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
// 9. Production code often uses unsafe for performance
// 10. The `lru` crate provides a well-tested implementation

// ============================================================================
// WHY THIS IS COMPLEX IN RUST
// ============================================================================
// In garbage-collected languages (Python, Go, Java), you can freely create
// cycles and multiple pointers to mutable data. The GC figures it out.
//
// In Rust, we must be explicit:
// - Use Rc for shared ownership (no GC needed)
// - Use RefCell for interior mutability (runtime borrow checking)
// - Be careful with cycles (can cause memory leaks with Rc)
//
// This complexity gives us:
// - No garbage collection pauses
// - Predictable memory usage
// - Better cache locality
// - Deterministic destruction
//
// For production, consider using the `lru` crate which uses unsafe code
// for better performance while maintaining safety through encapsulation.

// ============================================================================
// COMPARISON WITH OTHER APPROACHES
// ============================================================================
//
// OPTION 1: Vec + HashMap (simpler but O(n) eviction)
//   - HashMap: key -> index in Vec
//   - Vec stores (key, value, last_access_time)
//   - Eviction requires finding minimum (O(n))
//   - Good for small caches (<100 items)
//
// OPTION 2: BTreeMap only (O(log n) operations)
//   - Key: timestamp, Value: (original_key, value)
//   - Eviction is O(log n)
//   - Lookup requires reverse mapping (slower)
//
// OPTION 3: Our approach (HashMap + LinkedList)
//   - All operations O(1)
//   - More complex implementation
//   - Best for large caches (1000s+ items)
//
// OPTION 4: Unsafe raw pointers
//   - Eliminates Rc/RefCell overhead
//   - Requires careful manual memory management
//   - Used in production crates like `lru`

// ============================================================================
// REAL-WORLD OPTIMIZATIONS
// ============================================================================
// 1. Use FxHashMap (faster hash function for integer keys)
// 2. Pre-allocate HashMap with capacity
// 3. Use NonNull<Node> with unsafe instead of Rc<RefCell>
// 4. Pool node allocations (reduce malloc calls)
// 5. Add statistics (hit rate, eviction count)
// 6. Add TTL (time-to-live) on top of LRU
// 7. Shard the cache for concurrent access
// 8. Use SIMD for bulk operations

// ============================================================================
// COMMON BEGINNER MISTAKES
// ============================================================================
// ❌ Forgetting to move accessed items to front (not LRU anymore!)
// ❌ Not updating both HashMap AND linked list
// ❌ Creating reference cycles (memory leak with Rc)
// ❌ Holding RefCell borrows across method calls (panic)
// ❌ Not handling empty list cases
// ❌ Off-by-one errors in capacity checks
// ❌ Forgetting to clone Rc before mutating
// ❌ Using .unwrap() instead of proper error handling

// ============================================================================
// THREAD SAFETY NOTE
// ============================================================================
// This implementation is NOT thread-safe because:
// - Rc is not thread-safe (use Arc instead)
// - RefCell is not thread-safe (use Mutex/RwLock instead)
//
// For concurrent LRU cache:
// - Use Arc<Mutex<Node>> instead of Rc<RefCell<Node>>
// - Wrap entire cache in Mutex, or
// - Use lock-free algorithms (much more complex)
// - Consider crates like `lru` with thread-safe variants
