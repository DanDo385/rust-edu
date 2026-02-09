//! # A Least Recently Used (LRU) Cache - Interactive Demo
//! 
//! This binary demonstrates the `LruCache` from our library.
//! Run with: cargo run -p lru-cache

use lru_cache::solution::LruCache;

fn main() {
    println!("=== LRU Cache Demo ===\n");

    // ============================================================================
    // DEMO 1: Basic Put, Get, and Eviction
    // ============================================================================
    println!("1. Basic Operations:");
    println!("   ----------------");

    // Create a cache with a capacity of 3
    let mut cache = LruCache::new(3);
    println!("   Created a new cache with capacity 3.");

    println!("\n   Putting ('a', 1), ('b', 2), ('c', 3)...");
    cache.put('a', 1);
    cache.put('b', 2);
    cache.put('c', 3);
    println!("   Cache contents: {:?}", cache);

    println!("\n   Getting 'b'...");
    let val = cache.get(&'b');
    println!("   -> Got: {:?}", val);
    println!("   'b' is now the most recently used.");
    println!("   Cache contents: {:?}", cache);

    println!("\n   Putting ('d', 4)... This should evict 'a'.");
    cache.put('d', 4);
    println!("   Cache contents: {:?}", cache);
    println!("   Getting 'a' (should be None): {:?}", cache.get(&'a'));

    println!("\n   Putting ('c', 33) to update existing key...");
    cache.put('c', 33);
    println!("   'c' is now the most recently used.");
    println!("   Cache contents: {:?}", cache);
    assert_eq!(cache.get(&'c'), Some(&33));
    println!();

    // ============================================================================
    // DEMO 2: Access Pattern Test
    // ============================================================================
    println!("2. Access Pattern Test:");
    println!("   -------------------");
    let mut cache = LruCache::new(4);
    cache.put(1, 10);
    cache.put(2, 20);
    cache.put(3, 30);
    cache.put(4, 40);
    println!("   Initial cache: {:?}", cache);

    // Access 1, making it the MRU
    cache.get(&1);
    println!("   Accessed 1. Cache: {:?}", cache);

    // Add 5, which should evict 2 (the LRU item)
    cache.put(5, 50);
    println!("   Put 5. Evicted 2. Cache: {:?}", cache);
    assert!(cache.get(&2).is_none());
    assert!(cache.get(&1).is_some());
    println!();


    println!("=== Demo Complete! ===");
    println!("\nNow try:");
    println!("  1. Look at src/solution.rs for detailed explanations");
    println!("  2. Implement your own version in src/lib.rs");
    println!("  3. Run 'cargo test -p lru-cache' to check your work");
}