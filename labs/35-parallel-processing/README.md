# Project 35 - Parallel Processing with Rayon

## What You're Building (Plain English)

You're going to take slow, sequential code and make it run much faster by executing it in parallel across all your computer's CPU cores. You'll be working with a common problem: processing a large collection of data. For example, finding all the prime numbers in a huge range, or applying a filter to every pixel in a large image.

Instead of processing one item at a time, you'll use the `rayon` crate to automatically split the work between multiple threads. If you have an 8-core CPU, you could see a speedup of up to 8x for certain tasks!

## New Rust Concepts in This Project

-   **Data Parallelism**: The concept of taking a single large task and splitting the *data* among multiple threads to be processed concurrently.
-   **The `rayon` Crate**: The de-facto standard for data parallelism in Rust. It provides a simple, safe, and powerful way to convert sequential iterators into parallel iterators.
-   **`.par_iter()`**: The core method from Rayon. You can often just change a `.iter()` to a `.par_iter()` to instantly parallelize your code.
-   **Parallel Iterator Methods**: `map()`, `filter()`, `reduce()`, etc. These are parallel versions of the standard iterator methods you already know.
-   **Benchmarking**: You'll learn how to use the `criterion` crate to properly benchmark your sequential vs. parallel code to measure the speedup.
-   **Amdahl's Law**: The principle that the speedup from parallelization is limited by the sequential part of the task.

## Rust Syntax You'll See

```rust
use rayon::prelude::*;

let numbers: Vec<i32> = (0..1_000_000).collect();

// Sequential sum
let sum_seq = numbers.iter().sum::<i32>();

// Parallel sum - just change .iter() to .par_iter()!
let sum_par = numbers.par_iter().sum::<i32>();


// Sequential map-filter-reduce
let result_seq = numbers.iter()
    .map(|&x| x * x)
    .filter(|&x| x % 2 == 0)
    .sum::<i32>();

// Parallel map-filter-reduce
let result_par = numbers.par_iter()
    .map(|&x| x * x)
    .filter(|&x| x % 2 == 0)
    .sum::<i32>();
```

## How to Run

```bash
# Run the main binary (a demo comparing sequential vs parallel)
cargo run -p parallel-processing

# Run the tests
cargo test -p parallel-processing

# Run the benchmarks to see the performance difference!
cargo bench -p parallel-processing
```

## The Exercises

You will convert several sequential functions into parallel ones using Rayon.

1.  **`sum_of_squares()`**: Given a slice of `i32`, compute the sum of their squares. You'll write a sequential version and a parallel version.

2.  **`find_primes()`**: A function to find all prime numbers up to a certain limit. The provided primality test is slow, making this a great candidate for parallelization. You'll parallelize the process of checking each number.

3.  **`parallel_map()`**: A generic function that takes a slice of data and a function, and applies the function to each element in parallel, returning a new `Vec` with the results.

4.  **`parallel_sum_chunks()`**: A more manual approach. You'll split a large slice into chunks, process each chunk in a separate thread using `rayon::scope`, and then aggregate the results. This demonstrates how Rayon works under the hood.

## Solution Explanation (No Code - Just Ideas)

**How does `.par_iter()` work?**
Rayon's parallel iterators use a technique called "work stealing."
1.  The collection is recursively split into smaller and smaller pieces of work.
2.  A thread pool is created (usually one thread per CPU core).
3.  Each thread grabs a piece of work and starts processing it.
4.  If a thread finishes its work early, it "steals" work from another thread that is still busy.
This ensures that all CPU cores are kept busy, maximizing throughput. Best of all, Rayon guarantees this is done safely, with no data races.

**When is parallelization effective?**
-   When you have a large amount of data.
-   When the work done on each piece of data is independent of the others.
-   When the computation for each item is significant enough to outweigh the overhead of splitting the work and sending it to threads. For very simple operations (like just adding numbers), the overhead might be greater than the benefit.

## Where Rust Shines

-   **Fearless Concurrency**: Rayon is built on Rust's safety guarantees. It's impossible to have data races when using its parallel iterators with safe code. This is a massive advantage over parallel programming in languages like C++.
-   **Zero-Cost Abstractions**: The `.par_iter()` is a high-level abstraction, but it compiles down to highly efficient, low-level code. You get the convenience of functional-style programming with the performance of hand-tuned threading.
-   **Ergonomics**: The `.iter()` -> `.par_iter()` change is famously simple. Rayon's API is designed to be a drop-in replacement for the standard iterator API where possible.

## Common Beginner Mistakes

1.  **Parallelizing small workloads**: Trying to parallelize an operation on a tiny vector. The overhead of setting up threads and the work-stealing deque will be slower than just doing it sequentially.
2.  **Using `par_iter_mut()` without care**: If your parallel operation needs to mutate the data, you must use `par_iter_mut()`. However, you still can't have two threads mutating the *same* element. Rayon's iterators prevent this by giving each thread a unique slice of the data.
3.  **Not realizing some operations are sequential**: Not all iterator methods are parallelizable in the same way. For example, `find_first` will still have to check elements from the beginning, though the check itself might be parallelized.

This lab will give you a taste of the incredible performance gains possible with modern multi-core CPUs and the safety and elegance of Rust's ecosystem.