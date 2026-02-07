//! Integration tests for Lab 20: Multithreading Basics

use multithreading_basics::{
    spawn_workers, process_data_in_thread, SharedCounter,
    thread_worker, sum_parallel, parallel_map, counting_threads,
};

// ============================================================================
// SPAWN WORKERS TESTS
// ============================================================================

#[test]
fn test_spawn_workers_zero() {
    let results = spawn_workers(0);
    assert_eq!(results.len(), 0);
}

#[test]
fn test_spawn_workers_single() {
    let results = spawn_workers(1);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0], 0);
}

#[test]
fn test_spawn_workers_multiple() {
    let results = spawn_workers(5);
    assert_eq!(results.len(), 5);
    assert_eq!(results, vec![0, 2, 4, 6, 8usize]);
}

#[test]
fn test_spawn_workers_large() {
    let results = spawn_workers(100);
    assert_eq!(results.len(), 100);
    for (i, &result) in results.iter().enumerate() {
        assert_eq!(result, i * 2);
    }
}

// ============================================================================
// PROCESS DATA IN THREAD TESTS
// ============================================================================

#[test]
fn test_process_data_empty() {
    let data = vec![];
    let result = process_data_in_thread(data);
    assert_eq!(result, 0);
}

#[test]
fn test_process_data_single() {
    let data = vec![42];
    let result = process_data_in_thread(data);
    assert_eq!(result, 42);
}

#[test]
fn test_process_data_multiple() {
    let data = vec![1, 2, 3, 4, 5];
    let result = process_data_in_thread(data);
    assert_eq!(result, 15);
}

#[test]
fn test_process_data_large() {
    let data: Vec<i32> = (1..=100).collect();
    let result = process_data_in_thread(data);
    assert_eq!(result, 5050);  // Sum of 1..=100
}

#[test]
fn test_process_data_negative() {
    let data = vec![5, -3, 2, -4];
    let result = process_data_in_thread(data);
    assert_eq!(result, 0);
}

// ============================================================================
// SHARED COUNTER TESTS
// ============================================================================

#[test]
fn test_shared_counter_new() {
    let counter = SharedCounter::new(0);
    assert_eq!(counter.get(), 0);
}

#[test]
fn test_shared_counter_initial_value() {
    let counter = SharedCounter::new(42);
    assert_eq!(counter.get(), 42);
}

#[test]
fn test_shared_counter_increment() {
    let counter = SharedCounter::new(0);
    counter.increment();
    assert_eq!(counter.get(), 1);
}

#[test]
fn test_shared_counter_multiple_increments() {
    let counter = SharedCounter::new(0);
    for _ in 0..10 {
        counter.increment();
    }
    assert_eq!(counter.get(), 10);
}

#[test]
fn test_shared_counter_add() {
    let counter = SharedCounter::new(10);
    counter.add(5);
    assert_eq!(counter.get(), 15);
}

#[test]
fn test_shared_counter_add_negative() {
    let counter = SharedCounter::new(10);
    counter.add(-3);
    assert_eq!(counter.get(), 7);
}

#[test]
fn test_shared_counter_clone() {
    let counter = SharedCounter::new(0);
    let counter2 = counter.clone();

    counter.increment();
    assert_eq!(counter2.get(), 1);  // Both see same value
}

#[test]
fn test_shared_counter_multiple_clones() {
    let counter = SharedCounter::new(0);
    let c1 = counter.clone();
    let c2 = counter.clone();
    let c3 = counter.clone();

    c1.increment();
    c2.add(5);
    c3.add(2);

    assert_eq!(counter.get(), 8);
    assert_eq!(c1.get(), 8);
    assert_eq!(c2.get(), 8);
    assert_eq!(c3.get(), 8);
}

#[test]
fn test_shared_counter_concurrent_increments() {
    use std::thread;

    let counter = SharedCounter::new(0);
    let mut handles = vec![];

    for _ in 0..10 {
        let c = counter.clone();
        let handle = thread::spawn(move || {
            c.increment();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    assert_eq!(counter.get(), 10);
}

#[test]
fn test_shared_counter_concurrent_adds() {
    use std::thread;

    let counter = SharedCounter::new(0);
    let mut handles = vec![];

    for i in 0..5 {
        let c = counter.clone();
        let handle = thread::spawn(move || {
            c.add((i + 1) as i32);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Sum of 1+2+3+4+5 = 15
    assert_eq!(counter.get(), 15);
}

// ============================================================================
// THREAD WORKER TESTS
// ============================================================================

#[test]
fn test_thread_worker_basic() {
    let result = thread_worker(0, 0);
    assert_eq!(result, 0);
}

#[test]
fn test_thread_worker_single() {
    let result = thread_worker(1, 1);
    assert_eq!(result, 1);
}

#[test]
fn test_thread_worker_multiple() {
    let result = thread_worker(5, 3);
    // 5*3 + (0+1+2) = 15 + 3 = 18
    assert_eq!(result, 18);
}

// ============================================================================
// SUM PARALLEL TESTS
// ============================================================================

#[test]
fn test_sum_parallel_single() {
    let data = vec![5];
    assert_eq!(sum_parallel(data), 5);
}

#[test]
fn test_sum_parallel_two() {
    let data = vec![3, 7];
    assert_eq!(sum_parallel(data), 10);
}

#[test]
fn test_sum_parallel_multiple() {
    let data = vec![1, 2, 3, 4, 5];
    assert_eq!(sum_parallel(data), 15);
}

#[test]
fn test_sum_parallel_large() {
    let data: Vec<i32> = (1..=100).collect();
    assert_eq!(sum_parallel(data), 5050);
}

#[test]
fn test_sum_parallel_odd_length() {
    let data = vec![1, 2, 3, 4, 5, 6, 7];
    assert_eq!(sum_parallel(data), 28);
}

// ============================================================================
// PARALLEL MAP TESTS
// ============================================================================

#[test]
fn test_parallel_map_double() {
    let data = vec![1, 2, 3, 4, 5];
    let result = parallel_map(data, |x| x * 2);
    assert_eq!(result.len(), 5);
    assert!(result.contains(&2));
    assert!(result.contains(&10));
}

#[test]
fn test_parallel_map_square() {
    let data = vec![1, 2, 3, 4];
    let result = parallel_map(data, |x| x * x);
    assert_eq!(result.len(), 4);
    let sum: i32 = result.iter().sum();
    assert_eq!(sum, 30);  // 1+4+9+16
}

#[test]
fn test_parallel_map_transform() {
    let data = vec![1, 2, 3];
    let result = parallel_map(data, |x| x + 10);
    assert_eq!(result.len(), 3);
    assert!(result.contains(&11));
    assert!(result.contains(&13));
}

#[test]
fn test_parallel_map_large() {
    let data: Vec<i32> = (1..=100).collect();
    let result = parallel_map(data, |x| x * 2);
    assert_eq!(result.len(), 100);
    let sum: i32 = result.iter().sum();
    assert_eq!(sum, 10100);  // 2 * (1+2+...+100)
}

// ============================================================================
// COUNTING THREADS TESTS
// ============================================================================

#[test]
fn test_counting_threads_single_thread_single_iter() {
    let result = counting_threads(1, 1);
    assert_eq!(result, 0);
}

#[test]
fn test_counting_threads_single_thread() {
    let result = counting_threads(1, 5);
    assert_eq!(result, 0 + 1 + 2 + 3 + 4);
}

#[test]
fn test_counting_threads_multiple() {
    let result = counting_threads(3, 2);
    // Thread 0: 0, 1
    // Thread 1: 2, 3
    // Thread 2: 4, 5
    // Sum: 15
    assert_eq!(result, 15);
}

#[test]
fn test_counting_threads_formula() {
    let threads = 5;
    let iters = 4;
    let result = counting_threads(threads, iters);
    // Total values: threads * iters = 20
    // Values range from 0 to 19
    // Sum: (0+1+...+19) = 190
    let expected: i32 = (0..(threads as i32 * iters as i32)).sum();
    assert_eq!(result, expected);
}

// ============================================================================
// INTEGRATION TESTS
// ============================================================================

#[test]
fn test_multiple_shared_counters() {
    let c1 = SharedCounter::new(10);
    let c2 = SharedCounter::new(20);

    c1.increment();
    c2.add(5);

    assert_eq!(c1.get(), 11);
    assert_eq!(c2.get(), 25);
}

#[test]
fn test_shared_counter_in_parallel_work() {
    use std::thread;

    let counter = SharedCounter::new(0);
    let mut handles = vec![];

    // Simulate workers that both do work and increment counter
    for i in 0..3 {
        let c = counter.clone();
        let handle = thread::spawn(move || {
            // Do some work
            let work_result = i * 10;
            // Update shared state
            c.add(work_result as i32);
            work_result
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // 0 + 10 + 20 = 30
    assert_eq!(counter.get(), 30);
}

#[test]
fn test_combining_parallel_operations() {
    // Use multiple concurrent functions together
    let data = vec![1, 2, 3, 4, 5];
    let parallel_sum = sum_parallel(data.clone());
    let mapped = parallel_map(data, |x| x + 1);

    assert_eq!(parallel_sum, 15);
    assert_eq!(mapped.len(), 5);
}
