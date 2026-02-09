//! # Async Basics Demo

use async_basics::solution;
use futures::executor::block_on;

fn main() {
    println!("=== Async Basics Demo ===\n");
    println!("async_value: {}", block_on(solution::async_value()));
    println!("async_sequence(5): {}", block_on(solution::async_sequence(5)));
    println!("benefit: {}", solution::explain_async_benefit());
}
