// Lab 34: LRU Cache - Integration Tests
//
// Tests for the Least Recently Used cache implementation.
// Covers: insert/retrieve, capacity eviction, access recency updates,
// remove, overwrite, empty cache, ordering, and edge cases.

use lru_cache::LruCache;

// ============================================================================
// BASIC INSERT AND RETRIEVE
// ============================================================================

#[test]
fn test_insert_and_get() {
    let mut cache = LruCache::new(3);
    cache.put(1, "one");
    assert_eq!(cache.get(&1), Some("one"));
}

#[test]
fn test_insert_multiple_and_get() {
    let mut cache = LruCache::new(5);
    cache.put(1, "one");
    cache.put(2, "two");
    cache.put(3, "three");

    assert_eq!(cache.get(&1), Some("one"));
    assert_eq!(cache.get(&2), Some("two"));
    assert_eq!(cache.get(&3), Some("three"));
}

#[test]
fn test_get_nonexistent_key() {
    let mut cache: LruCache<i32, &str> = LruCache::new(3);
    cache.put(1, "one");
    assert_eq!(cache.get(&99), None);
}

#[test]
fn test_get_returns_none_for_empty_cache() {
    let mut cache: LruCache<i32, &str> = LruCache::new(3);
    assert_eq!(cache.get(&1), None);
}

// ============================================================================
// CAPACITY AND EVICTION
// ============================================================================

#[test]
fn test_eviction_removes_lru_item() {
    let mut cache = LruCache::new(3);
    cache.put(1, "one");   // LRU order: [1]
    cache.put(2, "two");   // LRU order: [2, 1]
    cache.put(3, "three"); // LRU order: [3, 2, 1]

    // Cache is full. Insert 4 -> evicts 1 (least recently used)
    cache.put(4, "four");  // LRU order: [4, 3, 2]

    assert_eq!(cache.get(&1), None); // evicted
    assert_eq!(cache.get(&2), Some("two"));
    assert_eq!(cache.get(&3), Some("three"));
    assert_eq!(cache.get(&4), Some("four"));
}

#[test]
fn test_eviction_with_capacity_1() {
    let mut cache = LruCache::new(1);
    cache.put(1, "one");
    assert_eq!(cache.get(&1), Some("one"));

    cache.put(2, "two"); // evicts 1
    assert_eq!(cache.get(&1), None);
    assert_eq!(cache.get(&2), Some("two"));
}

#[test]
fn test_multiple_evictions() {
    let mut cache = LruCache::new(2);
    cache.put(1, "one");
    cache.put(2, "two");

    cache.put(3, "three"); // evicts 1
    assert_eq!(cache.get(&1), None);

    cache.put(4, "four"); // evicts 2
    assert_eq!(cache.get(&2), None);

    assert_eq!(cache.get(&3), Some("three"));
    assert_eq!(cache.get(&4), Some("four"));
}

#[test]
fn test_len_and_capacity() {
    let mut cache = LruCache::new(3);
    assert_eq!(cache.len(), 0);
    assert_eq!(cache.capacity(), 3);

    cache.put(1, "one");
    assert_eq!(cache.len(), 1);

    cache.put(2, "two");
    assert_eq!(cache.len(), 2);

    cache.put(3, "three");
    assert_eq!(cache.len(), 3);

    // At capacity, adding a new key evicts one, len stays at capacity
    cache.put(4, "four");
    assert_eq!(cache.len(), 3);
    assert_eq!(cache.capacity(), 3);
}

#[test]
fn test_is_empty() {
    let mut cache: LruCache<i32, i32> = LruCache::new(3);
    assert!(cache.is_empty());

    cache.put(1, 10);
    assert!(!cache.is_empty());
}

// ============================================================================
// ACCESS UPDATES RECENCY
// ============================================================================

#[test]
fn test_get_moves_to_mru() {
    let mut cache = LruCache::new(3);
    cache.put(1, "one");   // [1]
    cache.put(2, "two");   // [2, 1]
    cache.put(3, "three"); // [3, 2, 1]

    // Access key 1 -> moves it to MRU position
    cache.get(&1);         // [1, 3, 2]

    // Now insert 4 -> should evict 2 (the new LRU), not 1
    cache.put(4, "four");  // [4, 1, 3]

    assert_eq!(cache.get(&2), None);     // evicted (was LRU after get(1))
    assert_eq!(cache.get(&1), Some("one")); // kept (was accessed)
    assert_eq!(cache.get(&3), Some("three"));
    assert_eq!(cache.get(&4), Some("four"));
}

#[test]
fn test_put_existing_moves_to_mru() {
    let mut cache = LruCache::new(3);
    cache.put(1, "one");
    cache.put(2, "two");
    cache.put(3, "three"); // [3, 2, 1]

    // Overwrite key 1 (should move to MRU)
    cache.put(1, "ONE");   // [1, 3, 2]

    // Insert 4 -> should evict 2 (LRU)
    cache.put(4, "four");  // [4, 1, 3]

    assert_eq!(cache.get(&2), None);
    assert_eq!(cache.get(&1), Some("ONE")); // updated value, still present
}

#[test]
fn test_keys_mru_order_reflects_access() {
    let mut cache = LruCache::new(4);
    cache.put(1, "A");
    cache.put(2, "B");
    cache.put(3, "C");
    cache.put(4, "D"); // MRU order: [4, 3, 2, 1]

    assert_eq!(cache.keys_mru_order(), vec![4, 3, 2, 1]);

    cache.get(&2); // [2, 4, 3, 1]
    assert_eq!(cache.keys_mru_order(), vec![2, 4, 3, 1]);

    cache.get(&1); // [1, 2, 4, 3]
    assert_eq!(cache.keys_mru_order(), vec![1, 2, 4, 3]);
}

#[test]
fn test_get_same_key_repeatedly() {
    let mut cache = LruCache::new(3);
    cache.put(1, "one");
    cache.put(2, "two");
    cache.put(3, "three");

    // Getting key 1 multiple times keeps it at MRU
    for _ in 0..5 {
        assert_eq!(cache.get(&1), Some("one"));
    }

    // Key 1 should be MRU
    assert_eq!(cache.keys_mru_order()[0], 1);
}

// ============================================================================
// REMOVE
// ============================================================================

#[test]
fn test_remove_existing_key() {
    let mut cache = LruCache::new(3);
    cache.put(1, "one");
    cache.put(2, "two");

    let removed = cache.remove(&1);
    assert_eq!(removed, Some("one"));
    assert_eq!(cache.len(), 1);
    assert_eq!(cache.get(&1), None);
}

#[test]
fn test_remove_nonexistent_key() {
    let mut cache: LruCache<i32, &str> = LruCache::new(3);
    cache.put(1, "one");

    let removed = cache.remove(&99);
    assert_eq!(removed, None);
    assert_eq!(cache.len(), 1);
}

#[test]
fn test_remove_head() {
    let mut cache = LruCache::new(3);
    cache.put(1, "one");
    cache.put(2, "two");
    cache.put(3, "three"); // MRU order: [3, 2, 1]

    // Remove the head (MRU = 3)
    cache.remove(&3);
    assert_eq!(cache.keys_mru_order(), vec![2, 1]);
}

#[test]
fn test_remove_tail() {
    let mut cache = LruCache::new(3);
    cache.put(1, "one");
    cache.put(2, "two");
    cache.put(3, "three"); // MRU order: [3, 2, 1]

    // Remove the tail (LRU = 1)
    cache.remove(&1);
    assert_eq!(cache.keys_mru_order(), vec![3, 2]);
}

#[test]
fn test_remove_middle() {
    let mut cache = LruCache::new(3);
    cache.put(1, "one");
    cache.put(2, "two");
    cache.put(3, "three"); // MRU order: [3, 2, 1]

    // Remove middle node (2)
    cache.remove(&2);
    assert_eq!(cache.keys_mru_order(), vec![3, 1]);
}

#[test]
fn test_remove_all_items() {
    let mut cache = LruCache::new(3);
    cache.put(1, "one");
    cache.put(2, "two");

    cache.remove(&1);
    cache.remove(&2);

    assert!(cache.is_empty());
    assert_eq!(cache.len(), 0);
    assert_eq!(cache.keys_mru_order(), Vec::<i32>::new());
}

#[test]
fn test_insert_after_remove() {
    let mut cache = LruCache::new(2);
    cache.put(1, "one");
    cache.put(2, "two");

    cache.remove(&1);
    assert_eq!(cache.len(), 1);

    // Insert a new key into the freed slot
    cache.put(3, "three");
    assert_eq!(cache.len(), 2);

    assert_eq!(cache.get(&1), None);
    assert_eq!(cache.get(&2), Some("two"));
    assert_eq!(cache.get(&3), Some("three"));
}

// ============================================================================
// OVERWRITE EXISTING KEY
// ============================================================================

#[test]
fn test_overwrite_updates_value() {
    let mut cache = LruCache::new(3);
    cache.put(1, "one");
    cache.put(1, "ONE");

    assert_eq!(cache.get(&1), Some("ONE"));
    assert_eq!(cache.len(), 1); // no duplicate entries
}

#[test]
fn test_overwrite_does_not_evict() {
    let mut cache = LruCache::new(3);
    cache.put(1, "one");
    cache.put(2, "two");
    cache.put(3, "three");

    // Overwriting an existing key should NOT trigger eviction
    cache.put(2, "TWO");

    assert_eq!(cache.len(), 3);
    assert_eq!(cache.get(&1), Some("one"));
    assert_eq!(cache.get(&2), Some("TWO"));
    assert_eq!(cache.get(&3), Some("three"));
}

#[test]
fn test_overwrite_multiple_times() {
    let mut cache = LruCache::new(2);
    cache.put(1, "v1");
    cache.put(1, "v2");
    cache.put(1, "v3");

    assert_eq!(cache.get(&1), Some("v3"));
    assert_eq!(cache.len(), 1);
}

// ============================================================================
// CONTAINS_KEY
// ============================================================================

#[test]
fn test_contains_key() {
    let mut cache = LruCache::new(3);
    cache.put(1, "one");

    assert!(cache.contains_key(&1));
    assert!(!cache.contains_key(&2));
}

#[test]
fn test_contains_key_after_eviction() {
    let mut cache = LruCache::new(2);
    cache.put(1, "one");
    cache.put(2, "two");
    cache.put(3, "three"); // evicts 1

    assert!(!cache.contains_key(&1));
    assert!(cache.contains_key(&2));
    assert!(cache.contains_key(&3));
}

// ============================================================================
// ORDERING (keys_mru_order)
// ============================================================================

#[test]
fn test_insertion_order() {
    let mut cache = LruCache::new(5);
    cache.put(1, "A");
    cache.put(2, "B");
    cache.put(3, "C");

    // Most recently inserted is at front
    assert_eq!(cache.keys_mru_order(), vec![3, 2, 1]);
}

#[test]
fn test_ordering_after_eviction() {
    let mut cache = LruCache::new(3);
    cache.put(1, "A");
    cache.put(2, "B");
    cache.put(3, "C"); // [3, 2, 1]

    cache.put(4, "D"); // evicts 1. [4, 3, 2]

    assert_eq!(cache.keys_mru_order(), vec![4, 3, 2]);
}

#[test]
fn test_ordering_after_overwrite() {
    let mut cache = LruCache::new(3);
    cache.put(1, "A");
    cache.put(2, "B");
    cache.put(3, "C"); // [3, 2, 1]

    cache.put(1, "A2"); // overwrite moves to MRU. [1, 3, 2]

    assert_eq!(cache.keys_mru_order(), vec![1, 3, 2]);
}

// ============================================================================
// DIFFERENT KEY/VALUE TYPES
// ============================================================================

#[test]
fn test_string_keys() {
    let mut cache = LruCache::new(3);
    cache.put("hello".to_string(), 1);
    cache.put("world".to_string(), 2);

    assert_eq!(cache.get(&"hello".to_string()), Some(1));
    assert_eq!(cache.get(&"world".to_string()), Some(2));
}

#[test]
fn test_integer_values() {
    let mut cache = LruCache::new(3);
    cache.put(1, 100);
    cache.put(2, 200);
    cache.put(3, 300);

    assert_eq!(cache.get(&1), Some(100));
    assert_eq!(cache.get(&2), Some(200));
    assert_eq!(cache.get(&3), Some(300));
}

// ============================================================================
// EDGE CASES
// ============================================================================

#[test]
fn test_capacity_1_eviction_chain() {
    let mut cache = LruCache::new(1);

    for i in 0..10 {
        cache.put(i, i * 10);
        assert_eq!(cache.len(), 1);
        assert_eq!(cache.get(&i), Some(i * 10));
    }

    // Only the last key should remain
    assert_eq!(cache.get(&9), Some(90));
    for i in 0..9 {
        assert_eq!(cache.get(&i), None);
    }
}

#[test]
fn test_large_cache() {
    let mut cache = LruCache::new(1000);

    for i in 0..1000 {
        cache.put(i, i);
    }
    assert_eq!(cache.len(), 1000);

    // All keys should be accessible
    for i in 0..1000 {
        assert_eq!(cache.get(&i), Some(i));
    }
}

#[test]
fn test_eviction_after_access_pattern() {
    // Demonstrate that access pattern affects eviction
    let mut cache = LruCache::new(3);
    cache.put(1, "A");
    cache.put(2, "B");
    cache.put(3, "C"); // [3, 2, 1]

    // Access 1 repeatedly, making it MRU
    cache.get(&1); // [1, 3, 2]

    // Insert 4 -> evicts 2 (the LRU)
    cache.put(4, "D"); // [4, 1, 3]

    assert!(cache.contains_key(&1));  // kept (recently accessed)
    assert!(!cache.contains_key(&2)); // evicted (LRU)
    assert!(cache.contains_key(&3));
    assert!(cache.contains_key(&4));
}

#[test]
#[should_panic(expected = "capacity must be greater than 0")]
fn test_zero_capacity_panics() {
    let _cache: LruCache<i32, i32> = LruCache::new(0);
}

#[test]
fn test_remove_then_reinsert() {
    let mut cache = LruCache::new(3);
    cache.put(1, "v1");
    cache.remove(&1);

    cache.put(1, "v2");
    assert_eq!(cache.get(&1), Some("v2"));
}

#[test]
fn test_interleaved_operations() {
    let mut cache = LruCache::new(3);

    cache.put(1, 10);
    cache.put(2, 20);
    assert_eq!(cache.get(&1), Some(10));

    cache.put(3, 30);
    cache.put(4, 40); // evicts 2 (LRU after get(1) moved 1 to front)

    assert_eq!(cache.get(&2), None);
    assert_eq!(cache.get(&1), Some(10));

    cache.remove(&3);
    cache.put(5, 50);

    assert_eq!(cache.len(), 3); // keys: 1, 4, 5
    assert!(cache.contains_key(&1));
    assert!(cache.contains_key(&4));
    assert!(cache.contains_key(&5));
    assert!(!cache.contains_key(&3));
}
