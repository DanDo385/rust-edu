//! # Thread Pool Demo

use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
use std::thread;
use std::time::Duration;
use thread_pool::solution::ThreadPool;

fn main() {
    println!("=== Thread Pool Demo ===\n");

    let pool = ThreadPool::new(4);
    let counter = Arc::new(AtomicUsize::new(0));

    for _ in 0..8 {
        let c = Arc::clone(&counter);
        pool.execute(move || {
            c.fetch_add(1, Ordering::SeqCst);
        });
    }

    thread::sleep(Duration::from_millis(100));
    println!("workers: {}", pool.worker_count());
    println!("jobs completed: {}", counter.load(Ordering::SeqCst));
}
