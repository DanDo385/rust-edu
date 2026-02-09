//! # Lab 20: Multithreading Basics
//!
//! Student-facing API for spawning threads, message passing, and shared state.

use std::sync::{Arc, Mutex};

pub fn spawn_workers(count: usize) -> Vec<usize> {
    // TODO: Spawn `count` worker threads and collect their outputs.
    let _ = count;
    todo!("Spawn worker threads")
}

pub fn process_data_in_thread(numbers: Vec<i32>) -> i32 {
    // TODO: Move numbers into a thread and return their sum.
    let _ = numbers;
    todo!("Process data in spawned thread")
}

#[derive(Clone)]
pub struct SharedCounter {
    count: Arc<Mutex<i32>>,
}

impl SharedCounter {
    pub fn new(initial: i32) -> Self {
        // TODO: Build SharedCounter from Arc<Mutex<i32>>.
        let _ = initial;
        todo!("Create shared counter")
    }

    pub fn increment(&self) {
        // TODO: Lock mutex and increment value.
        todo!("Increment shared counter")
    }

    pub fn get(&self) -> i32 {
        // TODO: Lock mutex and read value.
        todo!("Read shared counter")
    }

    pub fn add(&self, value: i32) {
        // TODO: Lock mutex and add value.
        let _ = value;
        todo!("Add to shared counter")
    }
}

pub fn thread_worker(id: usize, work_count: usize) -> i32 {
    // TODO: Compute deterministic worker total.
    let _ = (id, work_count);
    todo!("Compute worker result")
}

pub fn sum_parallel(numbers: Vec<i32>) -> i32 {
    // TODO: Split data across threads and sum through channel communication.
    let _ = numbers;
    todo!("Sum numbers in parallel")
}

pub fn parallel_map<F>(items: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32 + Send + Sync + 'static + Clone,
{
    // TODO: Apply f in parallel and collect results in order.
    let _ = (items, f);
    todo!("Parallel map over items")
}

pub fn counting_threads(thread_count: usize, iterations: usize) -> i32 {
    // TODO: Use mpsc with multiple producer threads and sum all emitted values.
    let _ = (thread_count, iterations);
    todo!("Count values from multiple producer threads")
}

#[doc(hidden)]
pub mod solution;
