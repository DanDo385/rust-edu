//! # A Least Recently Used (LRU) Cache - Your Implementation
//!
//! This project is about building a cache with a "Least Recently Used"
//! eviction policy.
//!
//! ## Your Task
//!
//! Implement the `LruCache` and its methods.
//!
//! 1.  **`LruCache` Struct**: Define the struct to hold the cache's capacity,
//!     a `HashMap` for fast key lookups, and a `LinkedList` to track the
//!     usage order of the keys.
//!
//! 2.  **`new()`**: A constructor that takes a capacity and creates an empty cache.
//!
//! 3.  **`put()`**: Inserts a key-value pair. If the key already exists, it updates
//!     the value and marks it as recently used. If the cache is full, it evicts
//!     the least recently used item before inserting the new one.
//!
//! 4.  **`get()`**: Retrieves a value for a key. If the key exists, it marks it
//!     as recently used and returns a reference to the value. Otherwise, it
//!     returns `None`.
//!
//! ## A Note on `LinkedList`
//!
//! The `std::collections::LinkedList` in Rust's standard library is not ideal
//! for a high-performance LRU cache because moving an element from the middle
//! to the front is an O(n) operation. A more optimal implementation would
//! use a custom linked list (likely with `unsafe` code) where the `HashMap`
//! could store raw pointers to the list nodes, allowing O(1) moves.
//!
//! **For this educational exercise, an O(n) move is acceptable.** It allows you
//! to focus on the logic of the cache without diving into `unsafe` Rust.
//!
//! ## Running Your Code
//!
//! ```bash
//! cargo test -p lru-cache
//! cargo run -p lru-cache
//! ```
//!
//! ## Stuck?
//!
//! Check out `src/solution.rs` for a complete, heavily-commented solution.

use std::collections::{HashMap, LinkedList};
use std::hash::Hash;

// TODO: Define the LruCache struct.
// It should be generic over a key `K` and a value `V`.
// K needs to have the `Eq` and `Hash` traits.
// It should contain:
// - capacity: usize
// - map: a HashMap to store keys and values
// - list: a LinkedList to track the order of usage (MRU to LRU)
//
// pub struct LruCache<K: Eq + Hash, V> { ... }
pub struct LruCache<K: Eq + Hash, V> {
    _capacity: usize,
    _map: HashMap<K, V>,
    _list: LinkedList<K>,
}

impl<K: Eq + Hash + Clone, V> LruCache<K, V> {
    /// Creates a new `LruCache` with a given capacity.
    pub fn new(capacity: usize) -> Self {
        // TODO: Initialize the cache.
        // The capacity cannot be zero. You might want to panic if it is.
        todo!("Initialize LruCache with capacity, an empty map, and an empty list");
    }

    /// Puts a key-value pair into the cache.
    pub fn put(&mut self, key: K, value: V) {
        // TODO: Implement the put logic.
        // 1. Check if the key already exists in the `map`.
        //    - If yes: update the value, and move the key to the front of the `list`.
        //    - If no:
        //        a. Check if the cache is at full capacity.
        //           - If yes: evict the least recently used item. This means
        //             removing the key from the back of the `list` and also
        //             removing the entry from the `map`.
        //        b. Insert the new key-value pair into the `map`.
        //        c. Push the new key to the front of the `list`.
        todo!("Implement the put method");
    }

    /// Gets a reference to a value for a given key.
    pub fn get(&mut self, key: &K) -> Option<&V> {
        // TODO: Implement the get logic.
        // 1. Check if the key exists in the `map`.
        // 2. If yes:
        //    a. Move the corresponding key in the `list` to the front.
        //       (This is the tricky O(n) part with `std::collections::LinkedList`).
        //    b. Return `Some(&value)`.
        // 3. If no, return `None`.
        //
        // Note: You'll need to get a mutable reference to the map to return a
        // non-mutable reference to the value. `map.get_mut(key)` will be useful
        // if you are also modifying the list. Or, you can perform the list
        // modification first, then do a separate `map.get(key)`.
        todo!("Implement the get method");
    }

    /// Returns the number of items in the cache.
    pub fn len(&self) -> usize {
        // TODO: Return the number of items currently in the cache.
        todo!("Return the length of the map or list");
    }

    /// Returns `true` if the cache is empty.
    pub fn is_empty(&self) -> bool {
        // TODO: Check if the cache is empty.
        todo!("Return true if the cache has no items");
    }

    /// Returns the capacity of the cache.
    pub fn capacity(&self) -> usize {
        // TODO: Return the configured capacity.
        todo!("Return the capacity");
    }
}


// Re-export the solution module so people can compare
#[doc(hidden)]
pub mod solution;
