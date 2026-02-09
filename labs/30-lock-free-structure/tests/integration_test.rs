//! Integration tests for Lab 30: Lock-Free Structure
//!
//! These tests verify the lock-free stack's correctness under various conditions:
//! - Single-threaded push/pop correctness
//! - LIFO (Last-In, First-Out) ordering
//! - Edge cases (empty stack, etc.)
//! - Memory safety (drop behavior)
//! - Stress testing under contention from multiple threads

use lock_free_structure::solution::LockFreeStack;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::thread;

// ============================================================================
// SINGLE-THREADED TESTS
// ============================================================================

#[test]
fn test_push_and_pop_single() {
    // Test basic push and pop on a single thread
    let stack = LockFreeStack::new();
    stack.push(1);
    assert_eq!(stack.pop(), Some(1));
}

#[test]
fn test_push_and_pop_multiple() {
    // Test a sequence of pushes and pops
    let stack = LockFreeStack::new();
    stack.push(10);
    stack.push(20);
    assert_eq!(stack.pop(), Some(20));
    stack.push(30);
    assert_eq!(stack.pop(), Some(30));
    assert_eq!(stack.pop(), Some(10));
}

#[test]
fn test_lifo_ordering() {
    // Verify that the stack follows Last-In, First-Out order
    let stack = LockFreeStack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    assert_eq!(stack.pop(), Some(3));
    assert_eq!(stack.pop(), Some(2));
    assert_eq!(stack.pop(), Some(1));
}

#[test]
fn test_pop_empty_returns_none() {
    // Popping from an empty stack should return None
    let stack: LockFreeStack<i32> = LockFreeStack::new();
    assert_eq!(stack.pop(), None);
}

#[test]
fn test_pop_empty_multiple_times() {
    // Popping from an empty stack repeatedly should always be None
    let stack: LockFreeStack<i32> = LockFreeStack::new();
    assert_eq!(stack.pop(), None);
    assert_eq!(stack.pop(), None);
    assert_eq!(stack.pop(), None);
}

#[test]
fn test_pop_after_draining() {
    // Popping after all elements have been removed
    let stack = LockFreeStack::new();
    stack.push(1);
    assert_eq!(stack.pop(), Some(1));
    assert_eq!(stack.pop(), None);
}

#[test]
fn test_drop_with_remaining_elements() {
    // Test that Drop deallocates remaining nodes
    // This test doesn't assert, but it will cause memory leaks
    // if Drop is not implemented correctly. Run with `valgrind` or similar
    // tools to verify.
    let stack = LockFreeStack::new();
    stack.push("hello".to_string());
    stack.push("world".to_string());
    // stack goes out of scope here and should be dropped
}

#[test]
fn test_drop_empty_stack() {
    // Dropping an empty stack should be a no-op
    let stack: LockFreeStack<i32> = LockFreeStack::new();
    // stack is dropped here
}

#[test]
fn test_with_strings() {
    // Test with a non-Copy type like String
    let stack = LockFreeStack::new();
    stack.push("hello".to_string());
    stack.push("world".to_string());
    assert_eq!(stack.pop(), Some("world".to_string()));
    assert_eq!(stack.pop(), Some("hello".to_string()));
}

#[test]
fn test_with_boxed_values() {
    // Test with heap-allocated values
    let stack = LockFreeStack::new();
    stack.push(Box::new(10));
    stack.push(Box::new(20));
    assert_eq!(stack.pop(), Some(Box::new(20)));
    assert_eq!(stack.pop(), Some(Box::new(10)));
}

// ============================================================================
// CONCURRENT TESTS
// ============================================================================

#[test]
fn test_concurrent_push() {
    // Multiple threads push concurrently
    let stack = Arc::new(LockFreeStack::new());
    let num_threads = 8;
    let items_per_thread = 100;

    let mut handles = vec![];
    for i in 0..num_threads {
        let stack_clone = Arc::clone(&stack);
        handles.push(thread::spawn(move || {
            for j in 0..items_per_thread {
                stack_clone.push((i, j));
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Verify all items are in the stack
    let mut count = 0;
    while let Some(_) = stack.pop() {
        count += 1;
    }
    assert_eq!(count, num_threads * items_per_thread);
}

#[test]
fn test_concurrent_pop() {
    // Multiple threads pop concurrently from a pre-filled stack
    let stack = Arc::new(LockFreeStack::new());
    let total_items = 10_000;
    for i in 0..total_items {
        stack.push(i);
    }

    let num_threads = 8;
    let popped_count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let mut handles = vec![];

    for _ in 0..num_threads {
        let stack_clone = Arc::clone(&stack);
        let popped_count_clone = Arc::clone(&popped_count);
        handles.push(thread::spawn(move || {
            while let Some(_) = stack_clone.pop() {
                popped_count_clone.fetch_add(1, Ordering::Relaxed);
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    assert_eq!(popped_count.load(Ordering::SeqCst), total_items);
    assert_eq!(stack.pop(), None); // Should be empty now
}

#[test]
fn test_concurrent_push_and_pop() {
    // The ultimate test: multiple threads pushing and popping at the same time
    let stack = Arc::new(LockFreeStack::new());
    let num_threads = 4;
    let items_per_thread = 5_000;

    // Initially populate the stack
    for i in 0..items_per_thread {
        stack.push(i);
    }

    let push_count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let pop_count = Arc::new(std::sync::atomic::AtomicUsize::new(0));

    let mut handles = vec![];
    for i in 0..num_threads {
        let stack_clone = Arc::clone(&stack);
        let push_count_clone = Arc::clone(&push_count);
        let pop_count_clone = Arc::clone(&pop_count);

        handles.push(thread::spawn(move || {
            // Each thread will do a mix of pushes and pops
            for j in 0..items_per_thread {
                if j % 2 == 0 {
                    // Push on even iterations
                    stack_clone.push(items_per_thread * (i + 1) + j);
                    push_count_clone.fetch_add(1, Ordering::Relaxed);
                } else {
                    // Pop on odd iterations
                    if stack_clone.pop().is_some() {
                        pop_count_clone.fetch_add(1, Ordering::Relaxed);
                    }
                }
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_push_count = push_count.load(Ordering::SeqCst);
    let final_pop_count = pop_count.load(Ordering::SeqCst);
    let mut remaining_on_stack = 0;
    while stack.pop().is_some() {
        remaining_on_stack += 1;
    }

    // The total number of items should be conserved
    // Initial + Pushed = Popped + Remaining
    assert_eq!(
        items_per_thread + final_push_count,
        final_pop_count + remaining_on_stack
    );
}

// ============================================================================
// STRESS TEST
// ============================================================================

#[test]
#[ignore] // This test is slow and should be run explicitly
fn test_stress_high_contention() {
    // A very high contention stress test, many threads, many ops
    let stack = Arc::new(LockFreeStack::new());
    let num_threads = 16;
    let ops_per_thread = 100_000;

    let mut handles = vec![];

    for _ in 0..num_threads {
        let stack_clone = Arc::clone(&stack);
        handles.push(thread::spawn(move || {
            for i in 0..ops_per_thread {
                if i % 2 == 0 {
                    stack_clone.push(i);
                } else {
                    stack_clone.pop();
                }
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // We don't know the exact final count, but the test should complete
    // without deadlocking or crashing. We can do a basic sanity check.
    let mut final_count = 0;
    while stack.pop().is_some() {
        final_count += 1;
    }

    println!(
        "Stress test finished with {} items remaining on stack.",
        final_count
    );
    // The number of remaining items must be less than or equal to the total number of pushes
    assert!(final_count <= num_threads * (ops_per_thread / 2));
}
