//! # Lock-Free Stack - Interactive Demo
//! 
//! This binary demonstrates the `LockFreeStack` from our library.
//! Run with: cargo run -p lock-free-structure

use lock_free_structure::solution::LockFreeStack;
use std::sync::Arc;
use std::thread;

fn main() {
    println!("=== Lock-Free Stack Demo ===\n");

    // ============================================================================
    // DEMO 1: Basic Push and Pop
    // ============================================================================
    println!("1. Basic Push and Pop (Single Thread):\n");
    println!("   ----------------------------------\n");

    let stack = LockFreeStack::new();
    stack.push(10);
    stack.push(20);
    stack.push(30);

    println!("   Pushed 10, 20, 30\n");
    println!("   Popped: {:?}\n", stack.pop()); // Should be Some(30)
    println!("   Popped: {:?}\n", stack.pop()); // Should be Some(20)
    println!("   Popped: {:?}\n", stack.pop()); // Should be Some(10)
    println!("   Popped: {:?}\n", stack.pop()); // Should be None
    println!();

    // ============================================================================
    // DEMO 2: Concurrent Stress Test
    // ============================================================================
    println!("2. Concurrent Stress Test:\n");
    println!("   ----------------------\n");
    demo_concurrent_stress_test();
    println!();

    println!("=== Demo Complete! ===\n");
    println!("\nNow try:\n");
    println!("  1. Look at src/solution.rs for detailed explanations\n");
    println!("  2. Implement your own version in src/lib.rs\n");
    println!("  3. Run 'cargo test -p lock-free-structure' to check your work\n");
}

fn demo_concurrent_stress_test() {
    let stack = Arc::new(LockFreeStack::new());
    let num_threads = 8;
    let items_per_thread = 1000;

    println!("   Spawning {} threads, each pushing {} items...\n", num_threads, items_per_thread);

    let mut handles = vec![];
    for i in 0..num_threads {
        let stack_clone = Arc::clone(&stack);
        handles.push(thread::spawn(move || {
            for j in 0..items_per_thread {
                stack_clone.push(i * items_per_thread + j);
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("   All threads finished pushing.\n");
    println!("   Total items pushed: {}\n", num_threads * items_per_thread);

    let mut count = 0;
    let mut numbers = vec![false; num_threads * items_per_thread];
    while let Some(value) = stack.pop() {
        if !numbers[value] {
            numbers[value] = true;
            count += 1;
        }
    }

    println!("   Popped {} unique items from the stack.\n", count);

    if count == num_threads * items_per_thread {
        println!("   ✅ Success: All items were pushed and popped correctly!\n");
    } else {
        println!("   ❌ Error: Lost {} items!\n", num_threads * items_per_thread - count);
    }
}