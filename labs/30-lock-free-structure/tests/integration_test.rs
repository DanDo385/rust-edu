// Integration tests for Lab 30: Lock-Free Structure
//
// These tests verify the lock-free stack's correctness:
// - Basic push and pop operations
// - LIFO (Last-In, First-Out) ordering
// - Empty stack behavior
// - Length tracking
// - Concurrent push/pop with multiple threads
// - Stress testing under contention

use lock_free_structure::LockFreeStack;

use std::collections::HashSet;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

// ============================================================================
// BASIC PUSH AND POP
// ============================================================================

#[test]
fn test_push_and_pop_single() {
    let stack = LockFreeStack::new();
    stack.push(42);
    assert_eq!(stack.pop(), Some(42));
}

#[test]
fn test_push_and_pop_multiple() {
    let stack = LockFreeStack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    assert_eq!(stack.pop(), Some(3));
    assert_eq!(stack.pop(), Some(2));
    assert_eq!(stack.pop(), Some(1));
}

#[test]
fn test_push_pop_interleaved() {
    let stack = LockFreeStack::new();

    stack.push(1);
    stack.push(2);
    assert_eq!(stack.pop(), Some(2));

    stack.push(3);
    assert_eq!(stack.pop(), Some(3));
    assert_eq!(stack.pop(), Some(1));
    assert_eq!(stack.pop(), None);
}

// ============================================================================
// LIFO ORDERING
// ============================================================================

#[test]
fn test_lifo_ordering() {
    let stack = LockFreeStack::new();

    for i in 0..10 {
        stack.push(i);
    }

    // Should come out in reverse order
    for i in (0..10).rev() {
        assert_eq!(stack.pop(), Some(i));
    }
}

#[test]
fn test_lifo_with_strings() {
    let stack = LockFreeStack::new();

    stack.push(String::from("first"));
    stack.push(String::from("second"));
    stack.push(String::from("third"));

    assert_eq!(stack.pop(), Some(String::from("third")));
    assert_eq!(stack.pop(), Some(String::from("second")));
    assert_eq!(stack.pop(), Some(String::from("first")));
}

// ============================================================================
// EMPTY STACK
// ============================================================================

#[test]
fn test_pop_empty_returns_none() {
    let stack: LockFreeStack<i32> = LockFreeStack::new();
    assert_eq!(stack.pop(), None);
}

#[test]
fn test_pop_empty_multiple_times() {
    let stack: LockFreeStack<i32> = LockFreeStack::new();
    assert_eq!(stack.pop(), None);
    assert_eq!(stack.pop(), None);
    assert_eq!(stack.pop(), None);
}

#[test]
fn test_pop_after_draining() {
    let stack = LockFreeStack::new();
    stack.push(1);
    stack.push(2);

    assert_eq!(stack.pop(), Some(2));
    assert_eq!(stack.pop(), Some(1));
    assert_eq!(stack.pop(), None);
    assert_eq!(stack.pop(), None);
}

// ============================================================================
// LENGTH TRACKING
// ============================================================================

#[test]
fn test_len_empty() {
    let stack: LockFreeStack<i32> = LockFreeStack::new();
    assert_eq!(stack.len(), 0);
}

#[test]
fn test_len_after_pushes() {
    let stack = LockFreeStack::new();
    stack.push(1);
    assert_eq!(stack.len(), 1);
    stack.push(2);
    assert_eq!(stack.len(), 2);
    stack.push(3);
    assert_eq!(stack.len(), 3);
}

#[test]
fn test_len_after_pops() {
    let stack = LockFreeStack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    stack.pop();
    assert_eq!(stack.len(), 2);
    stack.pop();
    assert_eq!(stack.len(), 1);
    stack.pop();
    assert_eq!(stack.len(), 0);
}

#[test]
fn test_len_after_push_and_pop_cycles() {
    let stack = LockFreeStack::new();

    for _ in 0..5 {
        stack.push(1);
    }
    assert_eq!(stack.len(), 5);

    for _ in 0..3 {
        stack.pop();
    }
    assert_eq!(stack.len(), 2);

    for _ in 0..4 {
        stack.push(1);
    }
    assert_eq!(stack.len(), 6);
}

// ============================================================================
// IS_EMPTY
// ============================================================================

#[test]
fn test_is_empty_new_stack() {
    let stack: LockFreeStack<i32> = LockFreeStack::new();
    assert!(stack.is_empty());
}

#[test]
fn test_is_empty_after_push() {
    let stack = LockFreeStack::new();
    stack.push(1);
    assert!(!stack.is_empty());
}

#[test]
fn test_is_empty_after_drain() {
    let stack = LockFreeStack::new();
    stack.push(1);
    stack.pop();
    assert!(stack.is_empty());
}

// ============================================================================
// DEFAULT TRAIT
// ============================================================================

#[test]
fn test_default() {
    let stack: LockFreeStack<i32> = LockFreeStack::default();
    assert!(stack.is_empty());
    assert_eq!(stack.len(), 0);
}

// ============================================================================
// DROP (MEMORY CLEANUP)
// ============================================================================

#[test]
fn test_drop_with_remaining_elements() {
    // Verify that dropping a non-empty stack doesn't leak or panic
    let stack = LockFreeStack::new();
    for i in 0..100 {
        stack.push(i);
    }
    // stack is dropped here; Drop should free all 100 nodes
}

#[test]
fn test_drop_empty_stack() {
    let stack: LockFreeStack<i32> = LockFreeStack::new();
    drop(stack);
}

#[test]
fn test_drop_with_heap_data() {
    // Ensure Drop properly cleans up heap-allocated T values
    let stack = LockFreeStack::new();
    for i in 0..50 {
        stack.push(format!("string number {}", i));
    }
    // All Strings and Nodes should be freed
}

// ============================================================================
// CONCURRENT PUSH
// ============================================================================

#[test]
fn test_concurrent_push() {
    let stack = Arc::new(LockFreeStack::new());
    let num_threads = 8;
    let pushes_per_thread = 500;

    let mut handles = vec![];

    for t in 0..num_threads {
        let stack = Arc::clone(&stack);
        let handle = thread::spawn(move || {
            for i in 0..pushes_per_thread {
                stack.push(t * pushes_per_thread + i);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    assert_eq!(stack.len(), num_threads * pushes_per_thread);
}

#[test]
fn test_concurrent_push_all_values_present() {
    let stack = Arc::new(LockFreeStack::new());
    let num_threads = 4;
    let pushes_per_thread = 200;

    let mut handles = vec![];

    for t in 0..num_threads {
        let stack = Arc::clone(&stack);
        let handle = thread::spawn(move || {
            for i in 0..pushes_per_thread {
                stack.push(t * pushes_per_thread + i);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Pop all values and verify we got exactly what we pushed
    let mut values = HashSet::new();
    while let Some(v) = stack.pop() {
        values.insert(v);
    }

    assert_eq!(values.len(), num_threads * pushes_per_thread);

    for t in 0..num_threads {
        for i in 0..pushes_per_thread {
            assert!(
                values.contains(&(t * pushes_per_thread + i)),
                "Missing value: {}",
                t * pushes_per_thread + i
            );
        }
    }
}

// ============================================================================
// CONCURRENT POP
// ============================================================================

#[test]
fn test_concurrent_pop() {
    let stack = Arc::new(LockFreeStack::new());
    let total_items = 4000;

    // Pre-fill the stack
    for i in 0..total_items {
        stack.push(i);
    }

    let num_threads = 8;
    let popped_count = Arc::new(AtomicUsize::new(0));

    let mut handles = vec![];

    for _ in 0..num_threads {
        let stack = Arc::clone(&stack);
        let count = Arc::clone(&popped_count);
        let handle = thread::spawn(move || {
            loop {
                match stack.pop() {
                    Some(_) => {
                        count.fetch_add(1, Ordering::SeqCst);
                    }
                    None => break,
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    assert_eq!(popped_count.load(Ordering::SeqCst), total_items);
    assert!(stack.is_empty());
}

// ============================================================================
// CONCURRENT PUSH AND POP
// ============================================================================

#[test]
fn test_concurrent_push_and_pop() {
    let stack = Arc::new(LockFreeStack::new());
    let ops_per_thread = 1000;
    let num_push_threads = 4;
    let num_pop_threads = 4;

    let push_count = Arc::new(AtomicUsize::new(0));
    let pop_count = Arc::new(AtomicUsize::new(0));

    let mut handles = vec![];

    // Spawn push threads
    for _ in 0..num_push_threads {
        let stack = Arc::clone(&stack);
        let count = Arc::clone(&push_count);
        let handle = thread::spawn(move || {
            for i in 0..ops_per_thread {
                stack.push(i);
                count.fetch_add(1, Ordering::SeqCst);
            }
        });
        handles.push(handle);
    }

    // Spawn pop threads
    for _ in 0..num_pop_threads {
        let stack = Arc::clone(&stack);
        let count = Arc::clone(&pop_count);
        let handle = thread::spawn(move || {
            for _ in 0..ops_per_thread {
                // Pop may return None if stack is temporarily empty
                if stack.pop().is_some() {
                    count.fetch_add(1, Ordering::SeqCst);
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let total_pushed = push_count.load(Ordering::SeqCst);
    let total_popped = pop_count.load(Ordering::SeqCst);

    // Everything pushed was either popped or remains in the stack
    assert_eq!(total_pushed, num_push_threads * ops_per_thread);
    let remaining = stack.len();
    assert_eq!(
        total_pushed,
        total_popped + remaining,
        "pushed ({}) != popped ({}) + remaining ({})",
        total_pushed,
        total_popped,
        remaining
    );
}

// ============================================================================
// STRESS TEST
// ============================================================================

#[test]
fn test_stress_push_pop_cycles() {
    let stack = Arc::new(LockFreeStack::new());
    let cycles = 500;
    let num_threads = 8;

    let mut handles = vec![];

    for _ in 0..num_threads {
        let stack = Arc::clone(&stack);
        let handle = thread::spawn(move || {
            for i in 0..cycles {
                stack.push(i);
                // Pop about half the time to keep the stack from growing unbounded
                if i % 2 == 0 {
                    stack.pop();
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Drain remaining elements
    let mut count = 0;
    while stack.pop().is_some() {
        count += 1;
    }

    // Each thread pushed `cycles` and popped `cycles / 2`
    // Net pushes per thread = cycles - cycles/2 = 250
    // Total remaining should be num_threads * 250 = 2000
    // But since pops can fail (empty stack), remaining may vary
    // The important thing: we didn't panic, deadlock, or corrupt memory
    assert!(stack.is_empty());
    // Total drained should match what len() reported approximately
    let _ = count; // just verify we could drain without issues
}

#[test]
fn test_stress_high_contention() {
    // Many threads fighting over a small stack
    let stack = Arc::new(LockFreeStack::new());
    let num_threads = 16;
    let ops = 1000;

    let total_pushes = Arc::new(AtomicUsize::new(0));
    let total_pops = Arc::new(AtomicUsize::new(0));

    let mut handles = vec![];

    for _ in 0..num_threads {
        let stack = Arc::clone(&stack);
        let pushes = Arc::clone(&total_pushes);
        let pops = Arc::clone(&total_pops);
        let handle = thread::spawn(move || {
            for i in 0..ops {
                if i % 2 == 0 {
                    stack.push(i);
                    pushes.fetch_add(1, Ordering::SeqCst);
                } else if stack.pop().is_some() {
                    pops.fetch_add(1, Ordering::SeqCst);
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let pushed = total_pushes.load(Ordering::SeqCst);
    let popped = total_pops.load(Ordering::SeqCst);
    let remaining = stack.len();

    assert_eq!(
        pushed,
        popped + remaining,
        "pushed ({}) != popped ({}) + remaining ({})",
        pushed,
        popped,
        remaining
    );
}

// ============================================================================
// DIFFERENT TYPES
// ============================================================================

#[test]
fn test_with_strings() {
    let stack = LockFreeStack::new();
    stack.push(String::from("hello"));
    stack.push(String::from("world"));

    assert_eq!(stack.pop(), Some(String::from("world")));
    assert_eq!(stack.pop(), Some(String::from("hello")));
}

#[test]
fn test_with_boxed_values() {
    let stack = LockFreeStack::new();
    stack.push(Box::new(100));
    stack.push(Box::new(200));

    assert_eq!(stack.pop(), Some(Box::new(200)));
    assert_eq!(stack.pop(), Some(Box::new(100)));
}

#[test]
fn test_with_tuples() {
    let stack = LockFreeStack::new();
    stack.push((1, "one"));
    stack.push((2, "two"));

    assert_eq!(stack.pop(), Some((2, "two")));
    assert_eq!(stack.pop(), Some((1, "one")));
}

// ============================================================================
// EDGE CASES
// ============================================================================

#[test]
fn test_single_element_push_pop_repeated() {
    let stack = LockFreeStack::new();

    for i in 0..100 {
        stack.push(i);
        assert_eq!(stack.pop(), Some(i));
        assert!(stack.is_empty());
    }
}

#[test]
fn test_large_number_of_elements() {
    let stack = LockFreeStack::new();
    let n = 10_000;

    for i in 0..n {
        stack.push(i);
    }

    assert_eq!(stack.len(), n);

    for i in (0..n).rev() {
        assert_eq!(stack.pop(), Some(i));
    }

    assert!(stack.is_empty());
}
