//! # Parallel Processing with Rayon - Interactive Demo
//! 
//! This binary demonstrates the performance difference between sequential
//! and parallel computation for a CPU-bound task.
//! Run with: cargo run -p parallel-processing --release
//! 
//! NOTE: Always run performance-sensitive code in release mode!

use parallel_processing::solution;
use std::time::Instant;

fn main() {
    println!("=== Parallel Processing Demo ===\n");

    let limit = 200_000;
    println!(
        "Task: Finding all prime numbers up to {} (a CPU-bound task).\n",
        limit
    );

    // ============================================================================
    // DEMO 1: Sequential Execution
    // ============================================================================
    println!("1. Running sequentially (on a single core)...");
    let start_seq = Instant::now();
    let primes_seq = solution::find_primes_sequential(limit);
    let duration_seq = start_seq.elapsed();
    println!("   -> Found {} primes.", primes_seq.len());
    println!("   -> Time taken: {:?}", duration_seq);
    println!();

    // ============================================================================
    // DEMO 2: Parallel Execution
    // ============================================================================
    println!("2. Running in parallel (using all available cores)...");
    let start_par = Instant::now();
    let primes_par = solution::find_primes_parallel(limit);
    let duration_par = start_par.elapsed();
    println!("   -> Found {} primes.", primes_par.len());
    println!("   -> Time taken: {:?}", duration_par);
    println!();

    // ============================================================================
    // Comparison
    // ============================================================================
    assert_eq!(primes_seq, primes_par); // Both should yield the same result!

    println!("3. Comparison");
    println!("   -----------");
    if duration_par < duration_seq {
        let speedup = duration_seq.as_secs_f64() / duration_par.as_secs_f64();
        println!(
            "   ✅ Parallel execution was {:.2}x faster than sequential!",
            speedup
        );
    } else {
        println!("   ⚠️ Parallel execution was not faster. This can happen on very small workloads or single-core machines.");
    }

    let num_cores = num_cpus::get();
    println!("   (Running on a machine with {} logical CPU cores)", num_cores);


    println!("\n=== Demo Complete! ===");
    println!("\nTo see more detailed benchmarks, run:");
    println!("  cargo bench -p parallel-processing");
}
