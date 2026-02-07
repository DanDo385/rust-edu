// Lab 35: Parallel Processing with Rayon
//
// This module provides pure computation functions that demonstrate
// data parallelism concepts. These functions are designed to be used
// with both sequential and parallel iterators, allowing learners to
// compare performance and verify correctness.
//
// Key Concepts:
// - CPU-intensive pure functions suitable for parallelization
// - Deterministic computations that produce identical results
//   regardless of execution order (sequential vs parallel)
// - Filter/map/reduce patterns common in data-parallel workloads

use rayon::prelude::*;

// ============================================================================
// CORE COMPUTATION FUNCTIONS
// ============================================================================

/// Check if a number is prime using trial division.
///
/// This is a CPU-intensive function that benefits from parallelization
/// when applied across large ranges of numbers.
///
/// # Memory Model
/// Pure function - no heap allocation, no shared state.
/// All computation happens on the stack with simple integer arithmetic.
pub fn is_prime(n: u32) -> bool {
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

/// Simulate an expensive computation on an integer.
///
/// Returns a deterministic result for any given input, making it
/// safe to parallelize and easy to verify (sequential == parallel).
///
/// # Memory Model
/// Pure function with no side effects. The iterator chain is
/// evaluated eagerly via .sum(), producing a single i32 on the stack.
pub fn expensive_computation(n: i32) -> i32 {
    (0..1000).map(|i| i % (n + 1)).sum::<i32>()
}

/// Apply a simulated image filter to a single pixel value.
///
/// Uses wrapping arithmetic to simulate a transformation pipeline.
/// Deterministic: same input always produces same output.
pub fn apply_filter(pixel: u8) -> u8 {
    let mut result = pixel;
    for _ in 0..100 {
        result = result.wrapping_mul(17).wrapping_add(31);
    }
    result
}

// ============================================================================
// PARALLEL OPERATION FUNCTIONS
// ============================================================================

/// Find all prime numbers in the range [start, end) using parallel iteration.
///
/// Returns a sorted Vec of primes. Uses rayon's `into_par_iter` to
/// distribute the primality checks across multiple threads.
pub fn find_primes_parallel(start: u32, end: u32) -> Vec<u32> {
    (start..end).into_par_iter().filter(|&n| is_prime(n)).collect()
}

/// Find all prime numbers in the range [start, end) using sequential iteration.
///
/// Returns a sorted Vec of primes. Useful for verifying that
/// parallel results match sequential results.
pub fn find_primes_sequential(start: u32, end: u32) -> Vec<u32> {
    (start..end).filter(|&n| is_prime(n)).collect()
}

/// Apply expensive_computation to each element in parallel, returning results.
///
/// The output Vec preserves the same ordering as the input thanks to
/// rayon's `par_iter().map().collect()` which maintains index correspondence.
pub fn parallel_map(data: &[i32]) -> Vec<i32> {
    data.par_iter().map(|&x| expensive_computation(x)).collect()
}

/// Apply expensive_computation to each element sequentially, returning results.
pub fn sequential_map(data: &[i32]) -> Vec<i32> {
    data.iter().map(|&x| expensive_computation(x)).collect()
}

/// Compute the parallel sum of a slice cast to i64.
///
/// Uses rayon's parallel reduce to sum elements across threads,
/// then combines thread-local results.
pub fn parallel_sum(data: &[i32]) -> i64 {
    data.par_iter().map(|&x| x as i64).sum()
}

/// Count elements in a slice that satisfy the given predicate, in parallel.
pub fn parallel_count<F>(data: &[i32], predicate: F) -> usize
where
    F: Fn(&i32) -> bool + Sync,
{
    data.par_iter().filter(|x| predicate(x)).count()
}

/// Apply the pixel filter to an image buffer in parallel.
///
/// Each pixel is processed independently, making this trivially
/// parallelizable with no synchronization needed.
pub fn parallel_filter_image(image: &[u8]) -> Vec<u8> {
    image.par_iter().map(|&pixel| apply_filter(pixel)).collect()
}

/// Apply the pixel filter to an image buffer sequentially.
pub fn sequential_filter_image(image: &[u8]) -> Vec<u8> {
    image.iter().map(|&pixel| apply_filter(pixel)).collect()
}

/// Compute chunk sums: divide data into chunks of `chunk_size` and sum each chunk in parallel.
///
/// Returns a Vec where each element is the sum of one chunk.
pub fn parallel_chunk_sums(data: &[i32], chunk_size: usize) -> Vec<i32> {
    data.par_chunks(chunk_size)
        .map(|chunk| chunk.iter().sum())
        .collect()
}

/// Build a histogram of the last digit (mod 10) of each number, computed in parallel.
///
/// Uses rayon's fold/reduce pattern:
/// - fold: each thread builds a local histogram
/// - reduce: merge histograms from all threads
///
/// Returns a Vec of 10 elements where index i holds the count of numbers
/// whose value mod 10 equals i.
pub fn parallel_digit_histogram(numbers: &[i32]) -> Vec<usize> {
    numbers
        .par_iter()
        .fold(
            || vec![0usize; 10],
            |mut acc, &num| {
                let bucket = (num.unsigned_abs() % 10) as usize;
                acc[bucket] += 1;
                acc
            },
        )
        .reduce(
            || vec![0usize; 10],
            |mut a, b| {
                for i in 0..10 {
                    a[i] += b[i];
                }
                a
            },
        )
}

// ============================================================================
// WHAT RUST DOES UNDER THE HOOD
// ============================================================================
// 1. THREAD POOL
//    Rayon creates a global thread pool (default: number of CPU cores).
//    Threads are created once at startup and reused for all parallel operations.
//
// 2. WORK-STEALING SCHEDULER
//    Each thread has a deque of tasks. Idle threads steal from busy threads.
//    This balances load dynamically without explicit coordination.
//
// 3. PARALLEL ITERATOR PROTOCOL
//    par_iter() splits the data into chunks distributed to worker threads.
//    Each thread processes its chunks independently.
//    Results are combined at the end (if needed).
//
// 4. SEND + SYNC SAFETY
//    The compiler enforces Send/Sync bounds at compile time.
//    Closures passed to par_iter must be Sync (safe to share between threads).
//    Data must be Send (safe to transfer between threads).
//    This makes data races impossible in safe Rust.
//
// 5. CACHE EFFICIENCY
//    Work-stealing keeps threads busy with local data (good cache locality).
//    Chunking reduces cache invalidation between cores.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime_basic() {
        assert!(!is_prime(0));
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(!is_prime(4));
        assert!(is_prime(5));
    }

    #[test]
    fn test_expensive_computation_deterministic() {
        let a = expensive_computation(5);
        let b = expensive_computation(5);
        assert_eq!(a, b);
    }

    #[test]
    fn test_apply_filter_deterministic() {
        let a = apply_filter(42);
        let b = apply_filter(42);
        assert_eq!(a, b);
    }
}
