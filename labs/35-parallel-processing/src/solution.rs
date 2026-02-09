//! # Parallel Processing with Rayon - Complete Solution
//!
//! ## What We're Building
//!
//! This solution demonstrates how to convert sequential, iterator-based
//! computations into parallel ones using the `rayon` crate. Rayon provides
//! "parallel iterators" that can be dropped in to replace standard iterators,
//! often with just a single line of code change, to achieve significant
//! performance improvements on multi-core CPUs.
//!
//! ## Why Rust Is Perfect For This
//!
//! - **Fearless Concurrency**: Rayon is built on Rust's strong safety guarantees.
//!   It ensures that your parallel computations are free from data races. You
//!   can't accidentally share data between threads in an unsafe way.
//! - **High-Level Abstraction, Low-Level Speed**: The `.par_iter()` method is a
//!   zero-cost abstraction. It's as easy to use as a normal iterator, but it
//!   compiles down to a highly efficient work-stealing implementation that keeps
//!   all your CPU cores busy.
//!
//! ## Key Rayon Concepts You'll Learn
//!
//! - **`use rayon::prelude::*`**: This imports the necessary traits to add
//!   parallel methods like `.par_iter()` to standard collection types.
//! - **`.par_iter()`**: The parallel equivalent of `.iter()`. It turns a collection
//!   into a parallel iterator.
//! - **Work Stealing**: The underlying scheduling algorithm Rayon uses to
//!   distribute work efficiently among threads.

use rayon::prelude::*;

/// A deliberately slow primality test function to make the benefits of
/// parallelization more apparent. In a real application, you would use a
/// more efficient algorithm like the Sieve of Eratosthenes.
fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    // Check for divisibility up to the square root of n.
    for i in 2..=(n as f64).sqrt() as u32 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

// --- Sequential Implementations ---

/// Computes the sum of squares for a slice of numbers, sequentially.
pub fn sum_of_squares_sequential(numbers: &[i32]) -> i64 {
    numbers
        .iter() // Get a sequential iterator.
        .map(|&n| n as i64 * n as i64) // Square each number.
        .sum() // Sum the results.
}

/// Finds all prime numbers up to a given limit, sequentially.
pub fn find_primes_sequential(limit: u32) -> Vec<u32> {
    (2..=limit) // Create a range of numbers to check.
        .filter(|&n| is_prime(n)) // Keep only the prime ones.
        .collect() // Collect the results into a Vec.
}


// --- Parallel Implementations ---

/// Computes the sum of squares for a slice of numbers, in parallel.
pub fn sum_of_squares_parallel(numbers: &[i32]) -> i64 {
    numbers
        .par_iter() // The ONLY change: get a parallel iterator.
        .map(|&n| n as i64 * n as i64)
        .sum()
}

/// Finds all prime numbers up to a given limit, in parallel.
pub fn find_primes_parallel(limit: u32) -> Vec<u32> {
    (2..=limit)
        .into_par_iter() // Turn the range into a parallel iterator.
        .filter(|&n| is_prime(n)) // The filtering happens in parallel.
        .collect() // The results are collected back into a Vec in order.
}

/// A generic function to apply a function to each element of a slice in parallel.
pub fn parallel_map<T, R, F>(data: &[T], f: F) -> Vec<R>
where
    T: Sync + Copy, // `T` must be safe to share (`Sync`) and copy.
    R: Send,         // The result `R` must be safe to send between threads.
    F: Fn(T) -> R + Sync + Send, // The closure `f` must be safe to share and send.
{
    data.par_iter() // Create a parallel iterator over the data.
        .map(|&item| f(item)) // Apply the function `f` to each item in parallel.
        .collect() // Collect the results into a new Vec.
}
