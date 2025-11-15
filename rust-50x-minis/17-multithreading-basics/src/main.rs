// Project 17: Multithreading Basics
//
// Rust's approach to concurrency is unique: the type system prevents data races
// at compile time. If your code compiles, it's thread-safe! This is called
// "fearless concurrency" - you can write concurrent code without fear of
// race conditions, deadlocks, or undefined behavior.

use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() {
    println!("=== Rust Multithreading Basics ===\n");

    demonstrate_basic_threads();
    demonstrate_join_handles();
    demonstrate_ownership_and_move();
    demonstrate_arc_mutex();
    demonstrate_parallel_computation();
    demonstrate_send_sync_traits();
}

// ============================================================================
// BASIC THREAD SPAWNING
// ============================================================================
// std::thread::spawn creates a new OS thread and executes a closure.
// Each thread runs independently and concurrently.

fn demonstrate_basic_threads() {
    println!("--- Basic Thread Spawning ---");

    // Spawn a thread
    thread::spawn(|| {
        println!("Hello from spawned thread!");
    });

    // Main thread continues
    println!("Hello from main thread!");

    // ⚠️ Problem: Program might exit before the spawned thread completes!
    // We need to wait for threads to finish (see next section)

    // Give the spawned thread time to run (not a good solution!)
    thread::sleep(Duration::from_millis(100));

    println!();
}

// ============================================================================
// JOIN HANDLES
// ============================================================================
// thread::spawn returns a JoinHandle, which we can use to wait for
// the thread to complete and get its return value.

fn demonstrate_join_handles() {
    println!("--- Join Handles ---");

    // Spawn a thread and save the handle
    let handle = thread::spawn(|| {
        println!("Thread 1: Starting work...");
        thread::sleep(Duration::from_millis(100));
        println!("Thread 1: Work complete!");
        42  // Return value
    });

    // Main thread does other work
    println!("Main: Spawned thread 1, continuing...");

    // Spawn another thread
    let handle2 = thread::spawn(|| {
        println!("Thread 2: Computing...");
        thread::sleep(Duration::from_millis(50));
        println!("Thread 2: Done!");
        "Result from thread 2"
    });

    println!("Main: Spawned thread 2, now waiting...");

    // Wait for threads to complete and get their return values
    let result1 = handle.join().unwrap();
    let result2 = handle2.join().unwrap();

    println!("Thread 1 returned: {}", result1);
    println!("Thread 2 returned: {}", result2);

    // WHY JOIN?
    // 1. Ensures thread completes before program exits
    // 2. Retrieves the thread's return value
    // 3. Propagates panics from the thread (if any)

    println!();
}

// ============================================================================
// OWNERSHIP AND THE MOVE KEYWORD
// ============================================================================
// Threads need to own their data. The `move` keyword transfers ownership
// from the parent thread to the spawned thread.

fn demonstrate_ownership_and_move() {
    println!("--- Ownership and Move ---");

    let data = vec![1, 2, 3, 4, 5];

    // ❌ This won't work without `move`:
    // thread::spawn(|| {
    //     println!("{:?}", data);  // ERROR: closure may outlive the current function
    // });

    // The problem: The thread might outlive the function, so it can't borrow `data`.
    // Solution: Use `move` to transfer ownership to the thread.

    let handle = thread::spawn(move || {
        println!("Thread received data: {:?}", data);

        // Calculate sum in the thread
        let sum: i32 = data.iter().sum();
        println!("Sum calculated by thread: {}", sum);

        sum  // Return the sum
    });

    // ❌ data is no longer accessible here (ownership moved)
    // println!("{:?}", data);  // ERROR: value borrowed here after move

    let result = handle.join().unwrap();
    println!("Main thread received result: {}", result);

    // WHY MOVE?
    // Rust's ownership system prevents use-after-free bugs.
    // If the thread could borrow `data`, what happens when the parent function
    // returns and `data` is dropped? The thread would have a dangling reference!
    // By moving ownership, Rust guarantees the thread owns its data.

    println!();
}

// ============================================================================
// SHARING DATA WITH ARC<MUTEX<T>>
// ============================================================================
// To share mutable data between threads, we need:
// 1. Arc (Atomic Reference Counting) for shared ownership
// 2. Mutex (Mutual Exclusion) for exclusive access
//
// Arc<Mutex<T>> is Rust's equivalent of shared_ptr<mutex<T>> in C++ or
// synchronized objects in Java, but with compile-time safety guarantees!

fn demonstrate_arc_mutex() {
    println!("--- Sharing Data with Arc<Mutex<T>> ---");

    // Create a shared counter
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    // Spawn 10 threads, each incrementing the counter
    for i in 0..10 {
        // Clone the Arc (increments reference count, not the data!)
        let counter_clone = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            // Lock the mutex to get exclusive access
            let mut num = counter_clone.lock().unwrap();

            // Critical section: only one thread can be here at a time
            *num += 1;
            println!("Thread {} incremented counter to {}", i, *num);

            // Lock is automatically released when `num` goes out of scope
        });

        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Print final value
    println!("Final counter value: {}", *counter.lock().unwrap());

    // HOW THIS WORKS:
    // 1. Arc provides shared ownership (multiple threads can own the data)
    // 2. Mutex provides mutual exclusion (only one thread accesses at a time)
    // 3. lock() blocks until the lock is available
    // 4. The lock is automatically released when the MutexGuard is dropped
    //
    // Why Arc instead of Rc?
    // - Rc is NOT Send (can't be transferred between threads)
    // - Arc uses atomic operations for thread-safe reference counting
    //
    // Why Mutex instead of RefCell?
    // - RefCell is NOT Sync (runtime borrowing isn't thread-safe)
    // - Mutex uses OS primitives for thread-safe exclusive access

    println!();
}

// ============================================================================
// PARALLEL COMPUTATION
// ============================================================================
// Let's use multiple threads to speed up a CPU-intensive task.

fn demonstrate_parallel_computation() {
    println!("--- Parallel Computation ---");

    // Task: Calculate sum of numbers 1 to 1,000,000
    let total_numbers = 1_000_000;
    let num_threads = 4;
    let chunk_size = total_numbers / num_threads;

    println!("Calculating sum of 1 to {} using {} threads", total_numbers, num_threads);

    let shared_sum = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    let start = std::time::Instant::now();

    // Spawn threads, each calculating a portion
    for i in 0..num_threads {
        let sum_clone = Arc::clone(&shared_sum);
        let start_num = i * chunk_size + 1;
        let end_num = if i == num_threads - 1 {
            total_numbers  // Last thread gets any remainder
        } else {
            (i + 1) * chunk_size
        };

        let handle = thread::spawn(move || {
            // Calculate partial sum (no lock needed here - local computation)
            let partial_sum: i64 = (start_num..=end_num).sum();

            println!("Thread {} computed sum of {}..{} = {}", i, start_num, end_num, partial_sum);

            // Lock only to update shared sum (minimize lock time!)
            let mut total = sum_clone.lock().unwrap();
            *total += partial_sum;
        });

        handles.push(handle);
    }

    // Wait for all threads
    for handle in handles {
        handle.join().unwrap();
    }

    let duration = start.elapsed();
    let result = *shared_sum.lock().unwrap();

    println!("Parallel sum: {}", result);
    println!("Time taken: {:?}", duration);

    // Verify with sequential calculation
    let sequential_sum: i64 = (1..=total_numbers).sum();
    println!("Sequential sum (for verification): {}", sequential_sum);
    assert_eq!(result, sequential_sum);

    // PERFORMANCE NOTES:
    // - For this simple task, sequential might actually be faster due to overhead
    // - Thread creation, Arc, and Mutex all have costs
    // - Parallelism shines with more expensive computations
    // - Consider using rayon for easier data parallelism

    println!();
}

// ============================================================================
// SEND AND SYNC TRAITS
// ============================================================================
// Send: Type can be transferred between threads (ownership transfer)
// Sync: Type can be referenced from multiple threads (&T is Send)
//
// Most types are Send and Sync. Some exceptions:
// - Rc<T>: NOT Send or Sync (use Arc instead)
// - RefCell<T>: Send but NOT Sync (use Mutex instead)
// - Raw pointers: NOT Send or Sync (unsafe)

fn demonstrate_send_sync_traits() {
    println!("--- Send and Sync Traits ---");

    // SEND EXAMPLE: Types that can be transferred to another thread
    let string = String::from("Hello");
    let numbers = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        // Both String and Vec are Send - we can move them into the thread
        println!("Thread received string: {}", string);
        println!("Thread received numbers: {:?}", numbers);
    });

    handle.join().unwrap();

    // SYNC EXAMPLE: Types that can be shared between threads
    let shared_string = Arc::new(String::from("Shared"));

    let handles: Vec<_> = (0..3)
        .map(|i| {
            let s = Arc::clone(&shared_string);
            thread::spawn(move || {
                // String is Sync, so &String can be shared via Arc
                println!("Thread {}: {}", i, s);
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    // COMPILER ENFORCEMENT EXAMPLES (commented out):

    // ❌ Rc is NOT Send:
    // let rc_data = Rc::new(vec![1, 2, 3]);
    // thread::spawn(move || {
    //     println!("{:?}", rc_data);  // ERROR: Rc cannot be sent between threads
    // });

    // ❌ RefCell is NOT Sync:
    // let ref_cell_data = Arc::new(RefCell::new(0));
    // let clone = Arc::clone(&ref_cell_data);
    // thread::spawn(move || {
    //     *clone.borrow_mut() += 1;  // ERROR: RefCell is not Sync
    // });

    // ✅ Must use Mutex instead:
    let mutex_data = Arc::new(Mutex::new(0));
    let clone = Arc::clone(&mutex_data);
    let h = thread::spawn(move || {
        *clone.lock().unwrap() += 1;  // OK: Mutex is Sync
    });
    h.join().unwrap();

    println!("Mutex data after thread: {}", *mutex_data.lock().unwrap());

    // THE MAGIC OF RUST:
    // The compiler AUTOMATICALLY checks Send and Sync for you!
    // If your code compiles, it's thread-safe. No data races possible!
    // This is what makes Rust's concurrency "fearless".

    println!();
}

// ============================================================================
// ADDITIONAL PATTERNS
// ============================================================================

// Pattern 1: Scoped Threads (using crossbeam crate)
// Allows borrowing instead of moving, but requires external crate
// crossbeam::scope(|s| {
//     s.spawn(|_| { ... });
// });

// Pattern 2: Channels for message passing (see Project 19)
// use std::sync::mpsc;
// let (tx, rx) = mpsc::channel();

// Pattern 3: Thread pools (see Project 26)
// Reuse threads instead of spawning new ones

// Pattern 4: Rayon for data parallelism
// Easiest way to parallelize iterators
// use rayon::prelude::*;
// let sum: i32 = (1..1000).into_par_iter().sum();

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. thread::spawn creates OS threads, returns JoinHandle<T>
// 2. Call join() to wait for thread completion and get return value
// 3. Use `move` to transfer ownership to thread closure
// 4. Arc<T> provides shared ownership across threads (atomic ref counting)
// 5. Mutex<T> provides exclusive access to shared mutable data
// 6. Combine Arc<Mutex<T>> for shared mutable state
// 7. Send trait means "can transfer between threads"
// 8. Sync trait means "can share references between threads"
// 9. Compiler enforces thread safety at compile time (no data races!)
// 10. Lock for minimal time to avoid contention and deadlocks

// ============================================================================
// COMMON MISTAKES
// ============================================================================
// ❌ Forgetting to join threads (work might not complete)
// ❌ Using Rc instead of Arc in multithreaded code
// ❌ Using RefCell instead of Mutex for shared mutable state
// ❌ Holding locks too long (causes contention)
// ❌ Creating too many threads (expensive, use thread pools)
// ❌ Deadlocks from acquiring multiple locks in different orders
// ❌ Not handling thread panics (unwrap join results)
// ❌ Sharing data without Arc (ownership errors)
// ❌ Using threads for I/O-bound work (async is better)
// ❌ Cloning Arc unnecessarily (just borrow when possible)

// ============================================================================
// PERFORMANCE TIPS
// ============================================================================
// 1. Thread creation is expensive (~100μs, ~2MB stack)
// 2. Use thread pools for many short tasks
// 3. Minimize time holding locks (lock contention kills performance)
// 4. Consider lock-free data structures for high contention
// 5. Number of threads ≈ number of CPU cores for CPU-bound work
// 6. Use async/await for I/O-bound work (cheaper than threads)
// 7. Arc/Mutex have overhead - use only when needed
// 8. Beware of false sharing (cache line contention)
// 9. Profile before optimizing - concurrency bugs are hard!
// 10. Consider rayon for easy data parallelism
