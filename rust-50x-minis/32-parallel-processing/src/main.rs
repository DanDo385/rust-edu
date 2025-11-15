// Project 32: Parallel Processing with Rayon
//
// Demonstrates data parallelism using the Rayon crate. Shows how easy it is
// to parallelize computations in Rust with guaranteed safety and impressive
// performance gains on multi-core processors.

use rayon::prelude::*;
use std::time::Instant;

fn main() {
    println!("=== Parallel Processing with Rayon ===\n");

    // ============================================================================
    // BASIC PARALLEL ITERATION
    // ============================================================================
    println!("=== Basic Parallel Iteration ===");

    let numbers: Vec<i32> = (1..=10).collect();

    // Sequential iteration
    println!("Sequential:");
    numbers.iter().for_each(|&n| {
        println!("  Processing {} on thread {:?}", n, std::thread::current().id());
    });

    println!("\nParallel (notice different threads):");
    // Just add par_iter() instead of iter()!
    numbers.par_iter().for_each(|&n| {
        println!("  Processing {} on thread {:?}", n, std::thread::current().id());
    });

    println!();

    // ============================================================================
    // PARALLEL MAP
    // ============================================================================
    println!("=== Parallel Map ===");

    let data: Vec<i32> = (1..=100).collect();

    // Sequential map
    let start = Instant::now();
    let sequential: Vec<i32> = data.iter().map(|&x| expensive_computation(x)).collect();
    let seq_time = start.elapsed();

    // Parallel map - just add par_!
    let start = Instant::now();
    let parallel: Vec<i32> = data.par_iter().map(|&x| expensive_computation(x)).collect();
    let par_time = start.elapsed();

    println!("Sequential time: {:?}", seq_time);
    println!("Parallel time:   {:?}", par_time);
    println!(
        "Speedup: {:.2}x",
        seq_time.as_secs_f64() / par_time.as_secs_f64()
    );

    // Verify results are the same
    assert_eq!(sequential, parallel);
    println!("✓ Results verified identical\n");

    // ============================================================================
    // PRIME NUMBERS (CPU-INTENSIVE WORKLOAD)
    // ============================================================================
    println!("=== Finding Prime Numbers ===");

    let range = 1..100_000;

    // Sequential
    let start = Instant::now();
    let primes_seq: Vec<u32> = range.clone().filter(|&n| is_prime(n)).collect();
    let seq_time = start.elapsed();

    // Parallel
    let start = Instant::now();
    let primes_par: Vec<u32> = range.into_par_iter().filter(|&n| is_prime(n)).collect();
    let par_time = start.elapsed();

    println!("Found {} primes", primes_seq.len());
    println!("Sequential time: {:?}", seq_time);
    println!("Parallel time:   {:?}", par_time);
    println!(
        "Speedup: {:.2}x",
        seq_time.as_secs_f64() / par_time.as_secs_f64()
    );

    assert_eq!(primes_seq, primes_par);
    println!("✓ Results verified identical\n");

    // ============================================================================
    // PARALLEL REDUCE
    // ============================================================================
    println!("=== Parallel Reduce ===");

    let data: Vec<i32> = (1..=1000).collect();

    // Sum using reduce (combines elements pairwise)
    let sum = data.par_iter().map(|&x| x as i64).reduce(|| 0, |a, b| a + b);
    println!("Sum of 1..=1000 = {}", sum);

    // Equivalent using sum() method
    let sum2: i64 = data.par_iter().map(|&x| x as i64).sum();
    println!("Using sum(): {}", sum2);
    assert_eq!(sum, sum2);

    // Find maximum
    let max = data.par_iter().copied().max().unwrap();
    println!("Maximum value: {}", max);

    // Count elements matching condition
    let even_count = data.par_iter().filter(|&&x| x % 2 == 0).count();
    println!("Even numbers: {}", even_count);

    println!();

    // ============================================================================
    // PARALLEL FOLD
    // ============================================================================
    println!("=== Parallel Fold ===");

    // fold() is like reduce() but maintains thread-local accumulators
    // More efficient when accumulator is expensive to merge

    let numbers: Vec<i32> = (1..=100).collect();

    // Compute histogram in parallel
    let histogram = numbers
        .par_iter()
        .fold(
            || vec![0; 10], // Initial accumulator for each thread
            |mut acc, &num| {
                // Each thread maintains its own histogram
                let bucket = (num % 10) as usize;
                acc[bucket] += 1;
                acc
            },
        )
        .reduce(
            || vec![0; 10],
            |mut a, b| {
                // Merge histograms from different threads
                for i in 0..10 {
                    a[i] += b[i];
                }
                a
            },
        );

    println!("Histogram (digit -> count):");
    for (digit, count) in histogram.iter().enumerate() {
        println!("  {} -> {}", digit, count);
    }

    println!();

    // ============================================================================
    // WORK-STEALING DEMONSTRATION
    // ============================================================================
    println!("=== Work-Stealing with Unbalanced Work ===");

    // Create tasks with varying durations (simulating real-world scenarios)
    let tasks: Vec<u64> = vec![1, 5, 2, 8, 1, 1, 10, 1, 1, 1, 7, 1, 1, 6];

    println!("Tasks (milliseconds): {:?}", tasks);

    let start = Instant::now();
    tasks.par_iter().for_each(|&duration| {
        // Simulate work of varying duration
        std::thread::sleep(std::time::Duration::from_millis(duration));
        println!(
            "  Completed {}ms task on thread {:?}",
            duration,
            std::thread::current().id()
        );
    });
    let par_time = start.elapsed();

    let total_work: u64 = tasks.iter().sum();
    println!("\nTotal work: {}ms", total_work);
    println!("Parallel time: {:?}", par_time);
    println!(
        "Efficiency: {:.1}% (ideal would be {}ms on 4 cores)",
        (total_work as f64 / par_time.as_millis() as f64) * 100.0 / 4.0,
        total_work / 4
    );

    println!();

    // ============================================================================
    // PARALLEL CHUNKS
    // ============================================================================
    println!("=== Parallel Chunks ===");

    let data: Vec<i32> = (1..=100).collect();

    // Process in chunks (better for small operations)
    let chunk_sums: Vec<i32> = data
        .par_chunks(10) // Process 10 elements at a time
        .map(|chunk| chunk.iter().sum())
        .collect();

    println!("Chunk sums (10 elements each): {:?}", chunk_sums);
    println!(
        "Total from chunks: {}",
        chunk_sums.iter().sum::<i32>()
    );

    println!();

    // ============================================================================
    // SIMULATED IMAGE PROCESSING
    // ============================================================================
    println!("=== Simulated Image Processing ===");

    // Simulate an image as a 2D array of pixels
    let width = 1920;
    let height = 1080;
    let image: Vec<u8> = (0..(width * height)).map(|i| (i % 256) as u8).collect();

    println!("Image size: {}x{} pixels", width, height);

    // Sequential grayscale conversion
    let start = Instant::now();
    let _gray_seq: Vec<u8> = image.iter().map(|&pixel| apply_filter(pixel)).collect();
    let seq_time = start.elapsed();

    // Parallel grayscale conversion
    let start = Instant::now();
    let _gray_par: Vec<u8> = image.par_iter().map(|&pixel| apply_filter(pixel)).collect();
    let par_time = start.elapsed();

    println!("Sequential processing: {:?}", seq_time);
    println!("Parallel processing:   {:?}", par_time);
    println!(
        "Speedup: {:.2}x",
        seq_time.as_secs_f64() / par_time.as_secs_f64()
    );

    println!();

    // ============================================================================
    // NESTED PARALLELISM
    // ============================================================================
    println!("=== Nested Parallelism (Matrix Operations) ===");

    let matrix: Vec<Vec<i32>> = (0..100)
        .map(|i| (0..100).map(|j| i * 100 + j).collect())
        .collect();

    // Process rows in parallel, and each row's computation uses parallel operations
    let start = Instant::now();
    let row_sums: Vec<i32> = matrix
        .par_iter()
        .map(|row| row.par_iter().sum()) // Nested parallel iteration
        .collect();
    let par_time = start.elapsed();

    println!("Processed 100x100 matrix");
    println!("First 5 row sums: {:?}", &row_sums[..5]);
    println!("Time: {:?}", par_time);

    println!();

    // ============================================================================
    // RAYON SCOPE (ADVANCED)
    // ============================================================================
    println!("=== Rayon Scope (Borrowing with Parallelism) ===");

    let mut data = vec![1, 2, 3, 4, 5];

    // rayon::scope allows parallel closures to borrow from the environment
    rayon::scope(|s| {
        s.spawn(|_| {
            println!("  Task 1 reading data: {:?}", data);
        });

        s.spawn(|_| {
            println!("  Task 2 reading data: {:?}", data);
        });

        // Both tasks can read data simultaneously (immutable borrow)
    });

    // After scope, we can mutate again
    data.push(6);
    println!("After scope, data: {:?}", data);

    println!();

    println!("=== Parallel Processing Demo Complete ===");
    println!("\nKey Takeaway: Adding par_iter() instead of iter() often gives");
    println!("linear speedup on multi-core CPUs with zero race conditions!");
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Simulates an expensive computation (CPU-intensive)
fn expensive_computation(n: i32) -> i32 {
    // Simulate work
    (0..1000).map(|i| i % (n + 1)).sum::<i32>()
}

/// Check if a number is prime (CPU-intensive)
fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    let limit = (n as f64).sqrt() as u32 + 1;
    for i in (3..limit).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

/// Simulates applying a filter to a pixel (e.g., grayscale conversion)
fn apply_filter(pixel: u8) -> u8 {
    // Simulate some computation
    let mut result = pixel;
    for _ in 0..100 {
        result = result.wrapping_mul(17).wrapping_add(31);
    }
    result
}

// ============================================================================
// WHAT RUST DOES UNDER THE HOOD
// ============================================================================
// 1. THREAD POOL
//    - Rayon creates a global thread pool (default: number of CPU cores)
//    - Threads are created once at startup, reused for all parallel operations
//    - Each thread has ~8MB stack (configurable)
//
// 2. WORK-STEALING SCHEDULER
//    - Each thread has a deque (double-ended queue) of tasks
//    - Thread pushes/pops from one end of its own deque (no locks)
//    - Idle threads "steal" from the other end of busy threads' deques
//    - This balances load dynamically without coordination
//
// 3. PARALLEL ITERATOR PROTOCOL
//    - par_iter() splits the data into chunks
//    - Chunks are distributed to worker threads
//    - Each thread processes its chunks independently
//    - Results are combined at the end (if needed)
//
// 4. ZERO-COST ABSTRACTION
//    - Rayon compiles to efficient machine code (no runtime overhead)
//    - Parallel operations are as fast as hand-written thread code
//    - Iterator chains are optimized away by LLVM
//
// 5. MEMORY SAFETY
//    - Send/Sync trait bounds enforced at compile time
//    - Impossible to create data races in safe Rust
//    - Each thread gets immutable references OR exclusive ownership
//
// 6. CACHE EFFICIENCY
//    - Work-stealing keeps threads busy with local data (good cache locality)
//    - Chunking reduces cache invalidation between cores
//    - Much better than naive thread-per-task approach

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. Rayon makes parallelism as easy as adding par_iter()
// 2. Work-stealing automatically balances load across cores
// 3. Compile-time safety prevents data races
// 4. Best for CPU-intensive operations on large datasets
// 5. Performance scales nearly linearly with CPU cores
// 6. No manual thread management needed
// 7. Iterator chains work seamlessly in parallel
// 8. fold() and reduce() handle aggregation safely
// 9. Overhead is minimal for coarse-grained work
// 10. Production-ready (used by ripgrep, fd, polars, etc.)

// ============================================================================
// WHEN TO USE RAYON
// ============================================================================
// ✅ GOOD USE CASES:
// - Image/video processing (filters, encoding, decoding)
// - Scientific computing (simulations, matrix operations)
// - Data analysis (aggregations, transformations)
// - Compilation (parallel module compilation)
// - Search algorithms (parallel tree traversal)
// - Prime factorization, cryptography
// - Log file parsing and analysis
// - Sorting large datasets
//
// ❌ BAD USE CASES:
// - I/O-bound operations (use async instead)
// - Very small datasets (<1000 items)
// - Operations that need strict ordering
// - When work per item is < 1 microsecond
// - When you need fine-grained control over threads

// ============================================================================
// PERFORMANCE TUNING
// ============================================================================
// 1. CHUNK SIZE
//    - Too small: excessive overhead
//    - Too large: poor load balancing
//    - Default is usually good, but can tune with par_chunks()
//
// 2. THREAD POOL SIZE
//    - Defaults to number of CPU cores
//    - Can override with RAYON_NUM_THREADS environment variable
//    - Or programmatically: rayon::ThreadPoolBuilder
//
// 3. MINIMIZE ALLOCATIONS
//    - Use fold() instead of collect() when possible
//    - Reuse buffers across iterations
//    - Consider arena allocators for short-lived data
//
// 4. PROFILE FIRST
//    - Measure sequential performance
//    - Identify hotspots (>1ms per item)
//    - Parallelize only the slow parts
//    - Verify speedup with benchmarks

// ============================================================================
// COMPARISON: RAYON VS MANUAL THREADING
// ============================================================================
// MANUAL THREADING:
//   - More control (custom schedulers, priorities)
//   - More code (spawn, join, channels)
//   - Error-prone (race conditions, deadlocks)
//   - Fixed number of threads
//
// RAYON:
//   - Less control (uses global thread pool)
//   - Minimal code (par_iter() is enough)
//   - Safe by design (compile-time checks)
//   - Automatic load balancing
//
// WHEN TO USE MANUAL THREADS:
//   - Long-running background tasks
//   - Specific thread affinity requirements
//   - Need precise control over scheduling
//   - Interfacing with C libraries
//
// WHEN TO USE RAYON:
//   - Data parallelism (same operation, different data)
//   - CPU-intensive batch processing
//   - Want safety and simplicity
//   - Most common parallel workloads

// ============================================================================
// COMMON BEGINNER MISTAKES
// ============================================================================
// ❌ Parallelizing I/O-bound code (no speedup, might be slower)
// ❌ Using par_iter() on tiny datasets (overhead > benefit)
// ❌ Holding locks across parallel operations (deadlock or serialization)
// ❌ Trying to mutate shared state directly (won't compile)
// ❌ Not measuring actual speedup (assumptions are often wrong)
// ❌ Forgetting that order is not guaranteed
// ❌ Using Debug builds for benchmarking (always use --release)
// ❌ Not considering memory bandwidth limits (may not scale past 4-8 cores)

// ============================================================================
// ADVANCED: UNDER THE HOOD OF WORK-STEALING
// ============================================================================
// Work-stealing uses a clever deque implementation:
//
// Thread A's deque: [Task1, Task2, Task3, Task4]
//                    ^                          ^
//                    |                          |
//                   Steal                     Push/Pop
//                  (other threads)          (owner thread)
//
// - Owner thread pushes/pops from one end (LIFO, good cache locality)
// - Other threads steal from the other end (FIFO, breadth-first)
// - Uses atomic operations (CAS) for lock-free stealing
// - Only contends when a thread is idle AND trying to steal
//
// This gives:
// - O(1) push/pop for owner thread (common case)
// - O(1) steal for other threads (rare case)
// - Minimal synchronization overhead
// - Excellent load balancing
