//! # A Least Recently Used (LRU) Cache - Complete Solution
//!
//! ## What We're Building
//!
//! An `LruCache` that stores a limited number of key-value pairs. When it's
//! full, it evicts the "least recently used" item to make space.
//!
//! ## The Core Data Structures
//!
//! This implementation uses two standard library collections together:
//!
//! 1.  **`HashMap<K, V>`**: This provides O(1) average-time lookups, insertions,
//!     and deletions by key. This is how we get fast access to the values.
//!
//! 2.  **`LinkedList<K>`**: This tracks the usage order. We keep it sorted from
//!     Most Recently Used (MRU) at the front to Least Recently Used (LRU) at the
//!     back. This structure allows for O(1) additions to the front and O(1)
//!     removals from the back.
//!
//! ## Performance Note
//!
//! The one operation that is *not* O(1) in this implementation is moving an
//! existing element to the front of the `LinkedList` upon access. The standard
//! library's `LinkedList` does not provide a way to remove an arbitrary element
//! in O(1) time. Therefore, our `get` and `put` (on update) operations have a
//! hidden O(n) step.
//!
//! For a production-grade LRU cache, one would typically build a custom doubly-linked
//! list (often with `unsafe` Rust) and store raw pointers to the list nodes inside
//! the `HashMap`, achieving true O(1) performance for all operations. For this
//! educational lab, we stick to safe Rust and accept the trade-off.

use std::collections::{HashMap, LinkedList};
use std::hash::Hash;
use std::fmt;

/// A Least Recently Used (LRU) cache.
pub struct LruCache<K: Eq + Hash, V> {
    capacity: usize,
    /// `map` stores the key and its corresponding value.
    map: HashMap<K, V>,
    /// `list` stores the keys in order of usage, from most recently used (front)
    /// to least recently used (back).
    list: LinkedList<K>,
}

impl<K: Eq + Hash + Clone, V> LruCache<K, V> {
    /// Creates a new `LruCache` with a given capacity.
    ///
    /// The capacity must be greater than 0.
    pub fn new(capacity: usize) -> Self {
        if capacity == 0 {
            panic!("LRU Cache capacity must be greater than 0");
        }
        LruCache {
            capacity,
            map: HashMap::with_capacity(capacity),
            list: LinkedList::new(),
        }
    }

    /// Puts a key-value pair into the cache.
    pub fn put(&mut self, key: K, value: V) {
        if self.map.contains_key(&key) {
            // --- Key already exists ---
            // 1. Update the value in the map.
            self.map.insert(key.clone(), value);
            // 2. Mark the key as most recently used by moving it to the front of the list.
            self.move_to_front(&key);
        } else {
            // --- New key ---
            // 1. Check if the cache is at capacity.
            if self.list.len() == self.capacity {
                // Evict the least recently used item.
                if let Some(lru_key) = self.list.pop_back() {
                    // Remove it from the map as well.
                    self.map.remove(&lru_key);
                }
            }
            // 2. Insert the new key and value.
            self.map.insert(key.clone(), value);
            // 3. Add the new key to the front of the list (most recently used).
            self.list.push_front(key);
        }
    }

    /// Gets a reference to a value for a given key.
    ///
    /// If the key exists, it is marked as most recently used.
    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.map.contains_key(key) {
            // Mark the key as most recently used.
            self.move_to_front(key);
            // Now, return the value from the map.
            self.map.get(key)
        } else {
            None
        }
    }

    /// Helper method to move a key to the front of the usage list.
    ///
    /// This is the O(n) part of the implementation.
    fn move_to_front(&mut self, key: &K) {
        let mut found = false;
        let mut rebuilt = LinkedList::new();

        while let Some(k) = self.list.pop_front() {
            if !found && &k == key {
                found = true;
                continue;
            }
            rebuilt.push_back(k);
        }

        self.list = rebuilt;
        if found {
            self.list.push_front(key.clone());
        }
    }

    /// Returns the number of items in the cache.
    pub fn len(&self) -> usize {
        self.list.len()
    }

    /// Returns `true` if the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    /// Returns the capacity of the cache.
    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

/// Implement `Debug` for easy printing of the cache's state.
impl<K: fmt::Debug + Eq + Hash, V: fmt::Debug> fmt::Debug for LruCache<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LruCache")
            .field("capacity", &self.capacity)
            .field("size", &self.list.len())
            .field("order (MRU->LRU)", &self.list)
            .finish()
    }
}
