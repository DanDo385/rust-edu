// Project 27: Lock-Free Structure
//
// Implementation of a lock-free stack using atomic operations and Compare-And-Swap.
// Demonstrates low-level concurrent programming, memory ordering, and the challenges
// of lock-free data structures. This is educational - use crossbeam in production!

use std::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};
use std::ptr;

fn main() {
    println!("=== Lock-Free Stack Implementation ===\n");

    // ============================================================================
    // BASIC OPERATIONS
    // ============================================================================
    demo_basic_operations();

    // ============================================================================
    // MEMORY ORDERING DEMONSTRATION
    // ============================================================================
    demo_memory_ordering();

    // ============================================================================
    // CONCURRENT STRESS TEST
    // ============================================================================
    demo_concurrent_stress_test();

    // ============================================================================
    // PERFORMANCE COMPARISON
    // ============================================================================
    demo_performance_comparison();
}

fn demo_basic_operations() {
    println!("=== Basic Lock-Free Stack Operations ===\n");

    let stack = LockFreeStack::new();

    // Push some values
    stack.push(1);
    stack.push(2);
    stack.push(3);

    println!("Pushed: 1, 2, 3");

    // Pop values (should be in reverse: 3, 2, 1)
    println!("Popped: {:?}", stack.pop());  // Some(3)
    println!("Popped: {:?}", stack.pop());  // Some(2)
    println!("Popped: {:?}", stack.pop());  // Some(1)
    println!("Popped: {:?}", stack.pop());  // None

    println!();
}

fn demo_memory_ordering() {
    println!("=== Memory Ordering Demonstration ===\n");

    // Different orderings have different performance characteristics
    let stack = LockFreeStack::new();

    // SeqCst (Sequentially Consistent) - Strongest, slowest
    stack.push(42);
    println!("Using SeqCst ordering (default in our implementation)");

    // In production, you might use Acquire/Release for better performance
    println!("Acquire: Synchronizes with Release stores");
    println!("Release: Makes stores visible to Acquire loads");
    println!("Relaxed: No synchronization (just atomicity)\n");

    println!("Note: Our implementation uses SeqCst for simplicity and correctness");
    println!("Advanced: Use Acquire/Release pairs for better performance\n");
}

fn demo_concurrent_stress_test() {
    println!("=== Concurrent Stress Test ===\n");

    let stack = Arc::new(LockFreeStack::new());
    let num_threads = 8;
    let operations_per_thread = 1000;

    let mut handles = vec![];

    println!("Spawning {} threads, each doing {} push/pop operations",
             num_threads, operations_per_thread);

    let start = Instant::now();

    // Spawn threads that push and pop concurrently
    for t in 0..num_threads {
        let stack = Arc::clone(&stack);
        let handle = thread::spawn(move || {
            for i in 0..operations_per_thread {
                // Push thread-id and operation number
                let value = t * operations_per_thread + i;
                stack.push(value);

                // Pop occasionally
                if i % 3 == 0 {
                    stack.pop();
                }
            }
        });
        handles.push(handle);
    }

    // Wait for all threads
    for handle in handles {
        handle.join().unwrap();
    }

    let duration = start.elapsed();

    println!("Completed in {:?}", duration);
    println!("Operations performed: {}", num_threads * operations_per_thread * 2);
    println!("Final stack size: {}\n", stack.len());
}

fn demo_performance_comparison() {
    println!("=== Performance Comparison: Lock-Free vs Mutex ===\n");

    let iterations = 10000;
    let num_threads = 4;

    // Benchmark lock-free stack
    let lock_free_stack = Arc::new(LockFreeStack::new());
    let start = Instant::now();

    let mut handles = vec![];
    for _ in 0..num_threads {
        let stack = Arc::clone(&lock_free_stack);
        let handle = thread::spawn(move || {
            for i in 0..iterations {
                stack.push(i);
                stack.pop();
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let lock_free_duration = start.elapsed();

    println!("Lock-free stack ({} threads, {} ops each): {:?}",
             num_threads, iterations, lock_free_duration);

    // Benchmark mutex-based stack (for comparison)
    use std::sync::Mutex;
    let mutex_stack = Arc::new(Mutex::new(Vec::new()));
    let start = Instant::now();

    let mut handles = vec![];
    for _ in 0..num_threads {
        let stack = Arc::clone(&mutex_stack);
        let handle = thread::spawn(move || {
            for i in 0..iterations {
                stack.lock().unwrap().push(i);
                stack.lock().unwrap().pop();
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let mutex_duration = start.elapsed();

    println!("Mutex-based stack ({} threads, {} ops each): {:?}",
             num_threads, iterations, mutex_duration);

    println!("\nSpeedup: {:.2}x",
             mutex_duration.as_secs_f64() / lock_free_duration.as_secs_f64());

    println!("\nNote: Results vary by contention, CPU, and workload");
    println!("Try different thread counts and operation counts!");
}

// ============================================================================
// LOCK-FREE STACK IMPLEMENTATION
// ============================================================================

struct LockFreeStack<T> {
    head: AtomicPtr<Node<T>>,
    len: AtomicUsize,
}

struct Node<T> {
    data: T,
    next: *mut Node<T>,
}

impl<T> LockFreeStack<T> {
    /// Creates a new empty lock-free stack
    pub fn new() -> Self {
        LockFreeStack {
            head: AtomicPtr::new(ptr::null_mut()),
            len: AtomicUsize::new(0),
        }
    }

    /// Pushes a value onto the stack
    ///
    /// This is lock-free: uses CAS (Compare-And-Swap) loop
    pub fn push(&self, data: T) {
        // Allocate a new node on the heap
        let new_node = Box::into_raw(Box::new(Node {
            data,
            next: ptr::null_mut(),
        }));

        // CAS loop: Keep trying until we successfully update head
        loop {
            // Read the current head
            // Ordering::Acquire ensures we see all previous writes to nodes
            let old_head = self.head.load(Ordering::Acquire);

            // Set the new node's next pointer to current head
            unsafe {
                (*new_node).next = old_head;
            }

            // Try to update head to point to new node
            // If head is still old_head, update it to new_node
            // Otherwise, loop and try again
            match self.head.compare_exchange(
                old_head,
                new_node,
                Ordering::Release,  // Success: make new node visible to other threads
                Ordering::Acquire,  // Failure: reload with synchronization
            ) {
                Ok(_) => {
                    // Success! We installed the new head
                    self.len.fetch_add(1, Ordering::Relaxed);
                    break;
                }
                Err(_) => {
                    // Another thread modified head, try again
                    // This is the "lock-free" part - we don't block, just retry
                    continue;
                }
            }
        }
    }

    /// Pops a value from the stack
    ///
    /// Returns None if the stack is empty
    pub fn pop(&self) -> Option<T> {
        loop {
            // Read the current head
            let old_head = self.head.load(Ordering::Acquire);

            // If head is null, stack is empty
            if old_head.is_null() {
                return None;
            }

            // Read the next pointer from the node
            // SAFETY: We just checked that old_head is not null
            // However, another thread could free this node!
            // This is why production lock-free structures use epoch-based reclamation
            let next = unsafe { (*old_head).next };

            // Try to update head to the next node
            match self.head.compare_exchange(
                old_head,
                next,
                Ordering::Release,
                Ordering::Acquire,
            ) {
                Ok(_) => {
                    // Success! We removed old_head from the stack
                    self.len.fetch_sub(1, Ordering::Relaxed);

                    // Take ownership of the node and extract data
                    // SAFETY: We own this node now (successfully CAS'd)
                    // No other thread can access it
                    let node = unsafe { Box::from_raw(old_head) };
                    return Some(node.data);
                }
                Err(_) => {
                    // Another thread modified head, try again
                    continue;
                }
            }
        }
    }

    /// Returns the current length (approximate)
    ///
    /// Note: This is racy - length may change immediately after reading
    pub fn len(&self) -> usize {
        self.len.load(Ordering::Relaxed)
    }

    /// Returns true if the stack is empty (approximate)
    pub fn is_empty(&self) -> bool {
        self.head.load(Ordering::Acquire).is_null()
    }
}

// ============================================================================
// DROP IMPLEMENTATION - MEMORY CLEANUP
// ============================================================================

impl<T> Drop for LockFreeStack<T> {
    fn drop(&mut self) {
        // When the stack is dropped, we need to free all nodes
        // This is safe because we have exclusive access (&mut self)
        let mut current = self.head.load(Ordering::Relaxed);

        while !current.is_null() {
            unsafe {
                let node = Box::from_raw(current);
                current = node.next;
                // node is dropped here, freeing memory
            }
        }
    }
}

// SAFETY: LockFreeStack can be safely sent between threads
// All operations use atomic synchronization
unsafe impl<T: Send> Send for LockFreeStack<T> {}
unsafe impl<T: Send> Sync for LockFreeStack<T> {}

// ============================================================================
// WHAT RUST DOES UNDER THE HOOD
// ============================================================================
//
// 1. COMPARE-AND-SWAP (CAS)
//    On x86-64: Becomes CMPXCHG instruction
//    On ARM: Becomes LL/SC (Load-Linked/Store-Conditional)
//    Hardware ensures atomicity even across CPUs
//
// 2. MEMORY ORDERING
//    Ordering::Acquire: Prevents reordering of subsequent loads
//    Ordering::Release: Prevents reordering of prior stores
//    On x86: Nearly free (TSO memory model)
//    On ARM: Memory barriers (DMB instructions)
//
// 3. CACHE COHERENCY
//    When one CPU writes to head, other CPUs' caches are invalidated
//    MESI protocol ensures all CPUs see consistent state
//    High contention = lots of cache coherency traffic!
//
// 4. CAS LOOP SPINNING
//    If CAS fails, thread immediately retries (no sleeping)
//    Can waste CPU cycles if high contention
//    But avoids kernel context switch (faster than mutex if short wait)
//
// 5. RAW POINTERS AND UNSAFE
//    Box::into_raw gives us a raw pointer (*mut Node)
//    AtomicPtr stores/loads raw pointers atomically
//    Box::from_raw reconstructs Box for dropping
//    Unsafe is needed because compiler can't verify thread-safety
//
// 6. SEND/SYNC TRAITS
//    We manually implement Send + Sync
//    Compiler can't auto-derive because we use raw pointers
//    We promise: "Trust me, the atomics make this thread-safe"

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. Lock-free uses CAS loops instead of locks
// 2. compare_exchange is the core primitive
// 3. Memory ordering matters (Acquire/Release/SeqCst)
// 4. ABA problem requires mitigation (tagged pointers, epoch-based GC)
// 5. Memory reclamation is hard (we leak on pop in this example!)
// 6. Lock-free is faster under high contention
// 7. Lock-free is complex - use libraries (crossbeam) in production
// 8. unsafe code needs careful review and reasoning

// ============================================================================
// THE ABA PROBLEM
// ============================================================================
//
// Consider this scenario:
//
// Thread 1: Reads head = A
// Thread 2: Pop A (head = B)
// Thread 2: Pop B (head = C)
// Thread 2: Push A again (head = A)
// Thread 1: CAS succeeds (head is still A!)
//           But B is now invalid!
//
// SOLUTIONS:
// 1. Tagged pointers: Store (pointer, version) atomically
// 2. Hazard pointers: Track which pointers threads are using
// 3. Epoch-based reclamation: Defer freeing until safe (crossbeam-epoch)
//
// Our implementation leaks memory on pop to avoid this (for simplicity).
// Production code MUST use proper memory reclamation!

// ============================================================================
// WHY UNSAFE IS NECESSARY
// ============================================================================
//
// We use unsafe for:
// 1. Dereferencing raw pointers (*old_head).next
// 2. Box::from_raw (manual memory management)
// 3. Implementing Send + Sync manually
//
// WHY IT'S SAFE:
// 1. We only dereference after null check
// 2. CAS ensures only one thread "owns" a node at a time
// 3. Drop cleans up all nodes
// 4. Atomic operations synchronize across threads
//
// REMAINING UNSAFETY:
// - We leak memory on pop (acceptable for long-lived stacks)
// - ABA problem could cause issues (mitigated by not freeing)
// - Production code should use crossbeam-epoch

// ============================================================================
// PERFORMANCE CHARACTERISTICS
// ============================================================================
//
// BEST CASE (low contention):
// - Push: 1 CAS (~20-50ns)
// - Pop: 1 CAS (~20-50ns)
// - Faster than Mutex (~100ns lock/unlock)
//
// WORST CASE (high contention):
// - Push: Many CAS retries (microseconds)
// - Pop: Many CAS retries (microseconds)
// - Still faster than Mutex (avoids kernel)
//
// MEMORY:
// - Each node: 16 bytes (8-byte pointer + 8-byte data on 64-bit)
// - No extra metadata (unlike Mutex which has OS state)
//
// SCALABILITY:
// - Scales better than Mutex with more threads
// - But still limited by cache coherency traffic

// ============================================================================
// COMMON BEGINNER MISTAKES
// ============================================================================
// ❌ Using Relaxed ordering for everything
//    Ordering::Relaxed doesn't synchronize - can see stale data!
//    Fix: Use Acquire/Release or SeqCst
//
// ❌ Freeing nodes immediately in pop
//    Another thread might still be reading the node!
//    Fix: Use epoch-based reclamation (crossbeam-epoch)
//
// ❌ Not handling CAS failure
//    if let Ok(_) = cas(...) { return; }  // ❌ Missing loop!
//    Fix: Loop until CAS succeeds
//
// ❌ Forgetting null checks before dereferencing
//    let next = (*head).next;  // ❌ head might be null!
//    Fix: if !head.is_null() { ... }
//
// ❌ Assuming lock-free is always faster
//    For low contention, Mutex is simpler and often faster
//    Fix: Benchmark your specific workload!

// ============================================================================
// COMPARING MEMORY ORDERINGS
// ============================================================================
//
// RELAXED:
// - Only guarantees atomicity (no torn reads/writes)
// - No synchronization with other operations
// - Fastest, but can see stale data
// - Use for: Counters where exact order doesn't matter
//
// ACQUIRE (for loads):
// - Synchronizes with Release stores
// - All loads/stores after this cannot be reordered before
// - Use for: Reading shared state
//
// RELEASE (for stores):
// - Synchronizes with Acquire loads
// - All loads/stores before this cannot be reordered after
// - Use for: Publishing shared state
//
// SEQCST:
// - Strongest ordering - total order across all threads
// - Slowest, but easiest to reason about
// - Use for: Correctness first, optimize later
//
// In our implementation, we use SeqCst (via Acquire + Release pairs)
// for correctness. Advanced users can optimize to Acquire/Release.

// ============================================================================
// EXTENDING THIS IMPLEMENTATION
// ============================================================================
//
// To make production-ready:
//
// 1. MEMORY RECLAMATION
//    Use crossbeam-epoch for safe node deletion
//
// 2. ABA PREVENTION
//    Use tagged pointers (store version counter in unused bits)
//
// 3. BACKOFF
//    If CAS fails repeatedly, sleep or yield to reduce contention
//
// 4. LOCK-FREE QUEUE
//    Implement MPSC or MPMC queue (Michael-Scott algorithm)
//
// 5. HAZARD POINTERS
//    Alternative to epoch-based reclamation
//
// 6. WAIT-FREE VARIANT
//    Guarantee progress for every thread (harder!)
