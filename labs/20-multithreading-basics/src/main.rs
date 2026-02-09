//! # Multithreading Basics Demo

use multithreading_basics::solution::{self, SharedCounter};

fn main() {
    println!("=== Multithreading Basics Demo ===\n");

    let workers = solution::spawn_workers(5);
    println!("workers: {:?}", workers);

    let threaded_sum = solution::process_data_in_thread(vec![1, 2, 3, 4, 5]);
    println!("process_data_in_thread sum: {}", threaded_sum);

    let counter = SharedCounter::new(0);
    counter.increment();
    counter.add(4);
    println!("shared counter: {}", counter.get());

    println!("thread_worker: {}", solution::thread_worker(2, 4));
    println!("sum_parallel: {}", solution::sum_parallel(vec![10, 20, 30, 40]));
    println!("parallel_map x2: {:?}", solution::parallel_map(vec![1, 2, 3], |x| x * 2));
    println!("counting_threads: {}", solution::counting_threads(3, 5));
}
