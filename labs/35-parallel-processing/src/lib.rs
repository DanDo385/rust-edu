//! # Parallel Processing with Rayon - Your Implementation
//!
//! This project is about converting sequential data processing operations
//! into parallel ones using the `rayon` crate to gain a performance speedup.
//!
//! ## Your Task
//!
//! Implement the parallel versions of the functions below. The sequential
//! versions are already provided for you to adapt.
//!
//! 1.  **`sum_of_squares_parallel()`**: A parallel version of `sum_of_squares_sequential`.
//!     This should be a simple one-line change from `.iter()` to `.par_iter()`.
//!
//! 2.  **`find_primes_parallel()`**: A parallel version of `find_primes_sequential`.
//!     This is where you should see a significant speedup, as the primality test
//!     is computationally expensive.
//!
//! 3.  **`parallel_map()`**: A generic function that applies a function to all
//!     elements of a slice in parallel. The function `F` must be `Sync` and `Send`
//!     so it can be safely sent between threads.
//!
//! ## Running Your Code
//!
//! ```bash
//! # Run tests to check your implementation for correctness
//! cargo test -p parallel-processing
//!
//! # Run the main binary to see a performance comparison
//! cargo run -p parallel-processing --release
//!
//! # Run the official benchmarks for a more rigorous measurement
//! cargo bench -p parallel-processing
//! ```
//!
//! ## Stuck?
//!
//! Check out `src/solution.rs` for a complete, heavily-commented solution.

use rayon::prelude::*;

// A deliberately slow primality test.
// This is to make the performance difference more obvious.
fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=(n as f64).sqrt() as u32 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

// --- Sequential Functions (Provided) ---

/// Computes the sum of squares for a slice of numbers, sequentially.
pub fn sum_of_squares_sequential(numbers: &[i32]) -> i64 {
    numbers.iter().map(|&n| n as i64 * n as i64).sum()
}

/// Finds all prime numbers up to a given limit, sequentially.
pub fn find_primes_sequential(limit: u32) -> Vec<u32> {
    (2..=limit).filter(|&n| is_prime(n)).collect()
}

// --- Your Tasks (Parallel Implementations) ---

/// Computes the sum of squares for a slice of numbers, in parallel.
pub fn sum_of_squares_parallel(numbers: &[i32]) -> i64 {
    // TODO: Parallelize this function using Rayon.
    // Hint: Change `.iter()` to `.par_iter()`. That's it!
    todo!("Implement parallel sum of squares");
}

/// Finds all prime numbers up to a given limit, in parallel.
pub fn find_primes_parallel(limit: u32) -> Vec<u32> {
    // TODO: Parallelize this function.
    // Hint: The range `(2..=limit)` can be turned into a parallel iterator.
    todo!("Implement parallel prime finding");
}

/// A generic function to apply a function to each element of a slice in parallel.
///
/// `F` must be a closure that is `Sync` and `Send`.
/// `T` and `R` are the input and output types.
pub fn parallel_map<T, R, F>(data: &[T], f: F) -> Vec<R>
where
    T: Sync,
    R: Send,
    F: Fn(T) -> R + Sync + Send,
    T: Copy
{
    // TODO: Implement a generic parallel map function.
    // Hint: Use `.par_iter()`, `.map()`, and `.collect()`.
    // The type constraints on `T`, `R`, and `F` are necessary for thread safety.
    todo!("Implement generic parallel map");
}


// Re-export the solution module so people can compare
#[doc(hidden)]
pub mod solution;