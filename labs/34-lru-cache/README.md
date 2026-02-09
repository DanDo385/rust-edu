# Project 34 - A Least Recently Used (LRU) Cache

## What You're Building (Plain English)

You're building a "smart" storage box with a limited capacity, called an LRU Cache. When you `put` an item in, it's stored. When you `get` an item, you retrieve it. The catch is, when the box is full and you want to add a new item, you have to make room by throwing something out.

An LRU (Least Recently Used) cache has a simple rule for what to throw out: the item that you haven't looked at for the longest time. If you just looked at an item, it's "recently used" and safe. An item you haven't touched in a while is "least recently used" and is the first to go. This is a very common and effective caching strategy used everywhere from web browsers to operating systems.

## New Rust Concepts in This Project

-   **`std::collections::LinkedList`**: A doubly-linked list. We'll use this to keep track of the "used" order of our items. It allows for O(1) insertion and deletion at the front or back, which is perfect for our needs.
-   **`std::collections::HashMap`**: We'll use this for O(1) average-time lookups of our items by key.
-   **Combining Data Structures**: The core of this project is learning how to combine a `HashMap` and a `LinkedList` to create a new, more powerful data structure. The `HashMap` will store keys and pointers to the nodes in the `LinkedList`.
-   **Generics (`<K, V>`)**: Your `LruCache` will be generic over the key and value types, so it can store any kind of data, as long as the key is hashable and equatable.
-   **Manual `LinkedList` manipulation**: You will learn how to add, move, and remove nodes from a `LinkedList` efficiently.

## Rust Syntax You'll See

```rust
use std::collections::{HashMap, LinkedList};
use std::hash::Hash;

// A generic struct for our cache
pub struct LruCache<K: Eq + Hash, V> {
    capacity: usize,
    map: HashMap<K, V>, // For fast lookups
    list: LinkedList<K>, // For tracking usage order
}

// Pushing to the front of a list
// list.push_front(key);

// Moving an item to the front
// if let Some(index) = list.iter().position(|k| *k == key) {
//     if let Some(key) = list.remove(index) {
//         list.push_front(key);
//     }
// }

// Popping from the back of a list
// let lru_key = list.pop_back();
```

## How to Run

```bash
# Run the main binary (a demo of the LRU cache)
cargo run -p lru-cache

# Run the tests
cargo test -p lru-cache

# Check if code compiles
cargo check -p lru-cache
```

## The Exercises

You will implement `LruCache<K, V>`.

1.  **`LruCache` Struct**: Define the struct to hold the `capacity`, the `HashMap` for storage, and the `LinkedList` for ordering. The `HashMap` will store `V` (the values), but what will it use for its keys and what will the `LinkedList` store? *Hint: The `LinkedList` should store keys to avoid duplicating data.*

2.  **`new()`**: A constructor that takes a `capacity` and creates a new, empty cache.

3.  **`put()`**:
    -   If the key already exists, update its value and move it to the "front" of the usage list (most recently used).
    -   If it's a new key:
        -   If the cache is full (at capacity), evict the *least* recently used item. This means removing it from the `LinkedList` (from the back) and also from the `HashMap`.
        -   Insert the new key-value pair into the `HashMap`.
        -   Add the key to the front of the `LinkedList`.

4.  **`get()`**:
    -   Look up the key in the `HashMap`.
    -   If it exists, you've just "used" it, so you must move its corresponding entry in the `LinkedList` to the front.
    -   Return a reference to the value.
    -   If it doesn't exist, return `None`.

5.  **Helper methods**: `len()`, `is_empty()`, `capacity()`.

## Solution Explanation (No Code - Just Ideas)

**The Core Trick**: We use two data structures for their respective strengths.
-   `HashMap`: Gives us O(1) average-time `get` operations. We can instantly find a value if we know its key.
-   `LinkedList`: Gives us O(1) time for moving items to the front (when used) and removing items from the back (when evicting). This is our usage-order tracker.

When we `put` or `get` an item, we do two things:
1.  The `HashMap` operation (insert or lookup).
2.  The `LinkedList` operation (move key to front).

When we evict, we also do two things:
1.  The `LinkedList` operation (pop key from back).
2.  The `HashMap` operation (remove the entry for that key).

This combination allows all our main operations (`put` and `get`) to be O(1) on average.

## Where Rust Shines

-   **Rich `std::collections`**: Rust provides powerful, well-tested data structures like `HashMap` and `LinkedList` right in its standard library.
-   **Generics**: Rust's generic system allows us to write the `LruCache` once and have it work for any key/value types that meet the trait bounds (`Eq`, `Hash` for keys), promoting code reuse.
-   **Ownership**: The cache clearly owns its keys and values, and Rust's ownership rules ensure that memory is managed correctly without a garbage collector.

## Common Beginner Mistakes

1.  **Inefficiently finding an item in the `LinkedList`**: Iterating through the `LinkedList` to find a key to move it is an O(n) operation, which defeats the purpose of the cache.
    -   **Fix**: The standard library's `LinkedList` isn't ideal for this. A real-world implementation might use `unsafe` code to build a linked list where the `HashMap` can store a raw pointer directly to the node, allowing O(1) deletion. For this lab, we might accept the O(n) search during moves as a simplification, or explore more advanced solutions.

2.  **Forgetting to update both data structures**: A common bug is to remove an item from the `HashMap` but forget to remove it from the `LinkedList`, or vice-versa. This leads to an inconsistent state.
    -   **Fix**: Be disciplined. Every operation that modifies the cache must atomically update both the `HashMap` and the `LinkedList`.

3.  **Key Duplication**: Storing both the key and value in the `HashMap` and also storing the key in the `LinkedList` seems redundant.
    -   **Fix**: This is often the cleanest way. The `HashMap` owns the values, and the `LinkedList` can own the keys. The memory overhead is usually acceptable for the cleaner logic.

This is a classic data structures problem that appears frequently in technical interviews and is fundamental to high-performance computing. Enjoy! 🦀