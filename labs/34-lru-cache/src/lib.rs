// Lab 34: LRU Cache - Library
//
// Implements a Least Recently Used (LRU) cache using HashMap + doubly-linked list.
// This demonstrates advanced Rust patterns: interior mutability, reference counting,
// and combining data structures for optimal performance.
//
// ============================================================================
// OWNERSHIP & MEMORY MODEL
// ============================================================================
// The LRU cache combines two data structures for O(1) operations:
//
// 1. HashMap<K, Rc<RefCell<Node>>>: O(1) lookup by key
//    - Keys are cloned into both the HashMap and the Node
//    - Values are stored inside Nodes on the heap
//
// 2. Doubly-linked list (via head/tail pointers): O(1) reordering
//    - Nodes use Rc<RefCell<T>> for shared ownership + interior mutability
//    - Rc: multiple owners (HashMap entry + prev/next pointers all reference same node)
//    - RefCell: runtime borrow checking (we need to mutate node fields through shared refs)
//
// This Rc<RefCell<Node>> pattern is the idiomatic safe Rust way to build
// graph-like structures. The alternative is unsafe raw pointers (used by
// production crates like `lru` for better performance).
//
// Memory per entry: ~80-100 bytes (Node + HashMap bucket + Rc/RefCell overhead)

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

// ============================================================================
// DOUBLY-LINKED LIST NODE
// ============================================================================
// Each node stores a key-value pair and pointers to adjacent nodes.
// Using Rc<RefCell<>> enables shared ownership AND mutability.

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
// Combines HashMap (O(1) lookup) with doubly-linked list (O(1) reordering).
//
// Invariants:
// - head points to the Most Recently Used (MRU) item
// - tail points to the Least Recently Used (LRU) item
// - map.len() <= capacity
// - Every node in the list has a corresponding entry in the map, and vice versa

/// A Least Recently Used (LRU) cache with a fixed capacity.
///
/// When the cache is full and a new item is inserted, the least recently
/// used item is evicted. All operations (get, put, remove) are O(1).
///
/// # Type Parameters
/// - `K`: Key type, must be hashable, comparable, and cloneable
/// - `V`: Value type, must be cloneable
///
/// # Note
/// This implementation is NOT thread-safe (uses Rc/RefCell).
/// For concurrent use, replace Rc with Arc and RefCell with Mutex.
pub struct LruCache<K, V> {
    capacity: usize,
    map: HashMap<K, Rc<RefCell<Node<K, V>>>>,
    head: Option<Rc<RefCell<Node<K, V>>>>,  // Most recently used
    tail: Option<Rc<RefCell<Node<K, V>>>>,  // Least recently used
}

impl<K, V> LruCache<K, V>
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    /// Creates a new LRU cache with the specified maximum capacity.
    ///
    /// # Panics
    /// Panics if capacity is 0.
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "LRU cache capacity must be greater than 0");
        LruCache {
            capacity,
            map: HashMap::new(),
            head: None,
            tail: None,
        }
    }

    /// Gets a value from the cache by key.
    ///
    /// Returns `Some(value)` if the key exists, or `None` if it does not.
    /// Accessing a key moves it to the most-recently-used position.
    pub fn get(&mut self, key: &K) -> Option<V> {
        if let Some(node) = self.map.get(key) {
            let node = Rc::clone(node);

            // Move this node to the front (most recently used)
            self.remove_from_list(Rc::clone(&node));
            self.push_front(Rc::clone(&node));

            Some(node.borrow().value.clone())
        } else {
            None
        }
    }

    /// Inserts a key-value pair into the cache.
    ///
    /// If the key already exists, its value is updated and it is moved to
    /// the most-recently-used position. If the cache is at capacity and the
    /// key is new, the least recently used item is evicted first.
    pub fn put(&mut self, key: K, value: V) {
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

    /// Removes a key from the cache, returning its value if it existed.
    ///
    /// If the key is not present, returns `None`.
    pub fn remove(&mut self, key: &K) -> Option<V> {
        if let Some(node) = self.map.remove(key) {
            self.remove_from_list(Rc::clone(&node));
            Some(node.borrow().value.clone())
        } else {
            None
        }
    }

    /// Returns the number of items currently in the cache.
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Returns true if the cache contains no items.
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Returns the maximum capacity of the cache.
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Returns true if the cache contains the given key.
    ///
    /// Note: this does NOT update the recency of the key (it is a read-only check).
    pub fn contains_key(&self, key: &K) -> bool {
        self.map.contains_key(key)
    }

    /// Returns the keys in order from most recently used to least recently used.
    ///
    /// Useful for debugging and testing the internal ordering.
    pub fn keys_mru_order(&self) -> Vec<K> {
        let mut keys = Vec::new();
        let mut current = self.head.clone();

        while let Some(node) = current {
            let borrowed = node.borrow();
            keys.push(borrowed.key.clone());
            current = borrowed.next.clone();
        }

        keys
    }

    // ========================================================================
    // INTERNAL METHODS
    // ========================================================================

    /// Removes the least recently used item (tail of the list).
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

    /// Adds a node to the front of the list (most recently used position).
    fn push_front(&mut self, node: Rc<RefCell<Node<K, V>>>) {
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(Rc::clone(&node));
                node.borrow_mut().next = Some(Rc::clone(&old_head));
                node.borrow_mut().prev = None;

                self.head = Some(node);
            }
            None => {
                // List is empty, this is the first node
                node.borrow_mut().prev = None;
                node.borrow_mut().next = None;

                self.tail = Some(Rc::clone(&node));
                self.head = Some(node);
            }
        }
    }

    /// Removes a node from its current position in the list.
    /// Does not remove from HashMap or deallocate.
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
}
