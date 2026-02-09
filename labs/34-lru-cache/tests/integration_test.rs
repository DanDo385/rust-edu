//! Integration tests for Lab 34: LRU Cache
//!
//! These tests verify the `LruCache`'s functionality:
//! - `put` and `get` operations
//! - Eviction policy (Least Recently Used)
//! - Updating existing values
//! - Correctly tracking usage order
//! - Edge cases like zero capacity (panic) and capacity 1.

use lru_cache::solution::LruCache;

#[test]
fn test_new_cache_is_empty() {
    let cache: LruCache<i32, &str> = LruCache::new(10);
    assert_eq!(cache.len(), 0);
    assert!(cache.is_empty());
    assert_eq!(cache.capacity(), 10);
}

#[test]
#[should_panic]
fn test_new_with_zero_capacity_panics() {
    LruCache::<i32, i32>::new(0);
}

#[test]
fn test_put_and_get() {
    let mut cache = LruCache::new(2);
    cache.put("a", 1);
    cache.put("b", 2);

    assert_eq!(cache.get(&"a"), Some(&1));
    assert_eq!(cache.get(&"b"), Some(&2));
    assert_eq!(cache.len(), 2);
}

#[test]
fn test_get_nonexistent() {
    let mut cache: LruCache<&str, i32> = LruCache::new(2);
    assert_eq!(cache.get(&"a"), None);
}

#[test]
fn test_put_updates_existing_value() {
    let mut cache = LruCache::new(2);
    cache.put("a", 1);
    assert_eq!(cache.get(&"a"), Some(&1));

    cache.put("a", 11);
    assert_eq!(cache.get(&"a"), Some(&11));
    assert_eq!(cache.len(), 1);
}

#[test]
fn test_eviction_policy_lru() {
    let mut cache = LruCache::new(3);
    cache.put("a", 1); // LRU
    cache.put("b", 2);
    cache.put("c", 3); // MRU

    // 'a' is now the least recently used.
    // Adding 'd' should evict 'a'.
    cache.put("d", 4);

    assert_eq!(cache.get(&"a"), None); // 'a' should be gone
    assert_eq!(cache.get(&"b"), Some(&2));
    assert_eq!(cache.get(&"c"), Some(&3));
    assert_eq!(cache.get(&"d"), Some(&4));
    assert_eq!(cache.len(), 3);
}

#[test]
fn test_get_updates_recency() {
    let mut cache = LruCache::new(3);
    cache.put("a", 1); // Will become LRU
    cache.put("b", 2);
    cache.put("c", 3); // MRU

    // Access 'a', making it the new MRU.
    cache.get(&"a");

    // Now, 'b' should be the LRU.
    // Adding 'd' should evict 'b'.
    cache.put("d", 4);

    assert_eq!(cache.get(&"b"), None); // 'b' should be gone
    assert_eq!(cache.get(&"a"), Some(&1));
    assert_eq!(cache.get(&"c"), Some(&3));
    assert_eq!(cache.get(&"d"), Some(&4));
}

#[test]
fn test_put_updates_recency() {
    let mut cache = LruCache::new(3);
    cache.put("a", 1); // Will become LRU
    cache.put("b", 2);
    cache.put("c", 3); // MRU

    // Update 'a', which should make it the new MRU.
    cache.put("a", 11);

    // Now, 'b' should be the LRU.
    // Adding 'd' should evict 'b'.
    cache.put("d", 4);

    assert_eq!(cache.get(&"b"), None); // 'b' should be gone
    assert_eq!(cache.get(&"a"), Some(&11));
    assert_eq!(cache.get(&"c"), Some(&3));
    assert_eq!(cache.get(&"d"), Some(&4));
}

#[test]
fn test_capacity_1() {
    let mut cache = LruCache::new(1);

    cache.put("a", 1);
    assert_eq!(cache.get(&"a"), Some(&1));

    cache.put("b", 2); // Evicts 'a'
    assert_eq!(cache.get(&"a"), None);
    assert_eq!(cache.get(&"b"), Some(&2));
    assert_eq!(cache.len(), 1);

    cache.get(&"b"); // Should not change anything
    assert_eq!(cache.get(&"b"), Some(&2));

    cache.put("b", 22);
    assert_eq!(cache.get(&"b"), Some(&22));
    assert_eq!(cache.len(), 1);
}

#[test]
fn test_complex_access_pattern() {
    let mut cache = LruCache::new(4);
    cache.put(1, 10); // 1
    cache.put(2, 20); // 2, 1
    cache.put(3, 30); // 3, 2, 1
    cache.put(4, 40); // 4, 3, 2, 1 (1 is LRU)

    assert_eq!(cache.len(), 4);

    cache.get(&1);    // 1, 4, 3, 2 (2 is LRU)
    cache.get(&2);    // 2, 1, 4, 3 (3 is LRU)

    cache.put(5, 50); // 5, 2, 1, 4 (evicts 3)
    assert!(cache.get(&3).is_none());
    assert_eq!(cache.len(), 4);

    cache.put(6, 60); // 6, 5, 2, 1 (evicts 4)
    assert!(cache.get(&4).is_none());
    assert_eq!(cache.len(), 4);

    assert_eq!(cache.get(&1), Some(&10)); // 1, 6, 5, 2
    assert_eq!(cache.get(&2), Some(&20)); // 2, 1, 6, 5
}