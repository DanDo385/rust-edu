// Integration tests for Lab 29: Thread Pool
//
// These tests verify the thread pool's core functionality:
// - Pool creation with correct worker count
// - Single and multiple job execution
// - Jobs actually run on worker threads (verified via atomic counters)
// - Graceful shutdown via Drop
// - Concurrent job execution
// - Edge cases: pool size of 1, many jobs on few workers

use thread_pool::ThreadPool;

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Barrier};
use std::thread;
use std::time::Duration;

// ============================================================================
// POOL CREATION
// ============================================================================

#[test]
fn test_create_pool_with_4_workers() {
    let pool = ThreadPool::new(4);
    assert_eq!(pool.worker_count(), 4);
}

#[test]
fn test_create_pool_with_1_worker() {
    let pool = ThreadPool::new(1);
    assert_eq!(pool.worker_count(), 1);
}

#[test]
fn test_create_pool_with_many_workers() {
    let pool = ThreadPool::new(16);
    assert_eq!(pool.worker_count(), 16);
}

#[test]
#[should_panic(expected = "Thread pool size must be greater than 0")]
fn test_create_pool_with_zero_panics() {
    let _pool = ThreadPool::new(0);
}

// ============================================================================
// SINGLE JOB EXECUTION
// ============================================================================

#[test]
fn test_execute_single_job() {
    let pool = ThreadPool::new(2);
    let counter = Arc::new(AtomicUsize::new(0));
    let counter_clone = Arc::clone(&counter);

    pool.execute(move || {
        counter_clone.fetch_add(1, Ordering::SeqCst);
    });

    // Give the worker time to execute the job
    thread::sleep(Duration::from_millis(100));

    assert_eq!(counter.load(Ordering::SeqCst), 1);
}

#[test]
fn test_execute_job_runs_on_different_thread() {
    let pool = ThreadPool::new(2);
    let main_thread_id = thread::current().id();
    let worker_thread_id = Arc::new(std::sync::Mutex::new(None));
    let worker_thread_id_clone = Arc::clone(&worker_thread_id);

    pool.execute(move || {
        *worker_thread_id_clone.lock().unwrap() = Some(thread::current().id());
    });

    thread::sleep(Duration::from_millis(100));

    let worker_id = worker_thread_id.lock().unwrap().unwrap();
    assert_ne!(
        main_thread_id, worker_id,
        "Job should run on a worker thread, not the main thread"
    );
}

// ============================================================================
// MULTIPLE JOB EXECUTION
// ============================================================================

#[test]
fn test_execute_multiple_jobs() {
    let pool = ThreadPool::new(4);
    let counter = Arc::new(AtomicUsize::new(0));

    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        pool.execute(move || {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });
    }

    thread::sleep(Duration::from_millis(200));

    assert_eq!(counter.load(Ordering::SeqCst), 10);
}

#[test]
fn test_execute_many_jobs_on_few_workers() {
    // More jobs than workers: tests the job queue behavior
    let pool = ThreadPool::new(2);
    let counter = Arc::new(AtomicUsize::new(0));

    for _ in 0..50 {
        let counter_clone = Arc::clone(&counter);
        pool.execute(move || {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });
    }

    thread::sleep(Duration::from_millis(500));

    assert_eq!(counter.load(Ordering::SeqCst), 50);
}

#[test]
fn test_jobs_with_varying_duration() {
    let pool = ThreadPool::new(4);
    let counter = Arc::new(AtomicUsize::new(0));

    for i in 0..8 {
        let counter_clone = Arc::clone(&counter);
        pool.execute(move || {
            // Varying work duration
            thread::sleep(Duration::from_millis((i % 3 + 1) * 10));
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });
    }

    thread::sleep(Duration::from_millis(500));

    assert_eq!(counter.load(Ordering::SeqCst), 8);
}

// ============================================================================
// CONCURRENT EXECUTION VERIFICATION
// ============================================================================

#[test]
fn test_jobs_run_concurrently() {
    // Use a barrier to prove that multiple workers execute simultaneously.
    // If only one thread ran at a time, the barrier would deadlock.
    let pool = ThreadPool::new(4);
    let barrier = Arc::new(Barrier::new(4));
    let counter = Arc::new(AtomicUsize::new(0));

    for _ in 0..4 {
        let barrier_clone = Arc::clone(&barrier);
        let counter_clone = Arc::clone(&counter);
        pool.execute(move || {
            // All 4 threads must reach this point before any can continue
            barrier_clone.wait();
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });
    }

    thread::sleep(Duration::from_millis(500));

    assert_eq!(
        counter.load(Ordering::SeqCst),
        4,
        "All 4 jobs should complete, proving concurrent execution"
    );
}

#[test]
fn test_multiple_threads_used() {
    // Verify that jobs actually distribute across multiple worker threads
    let pool = ThreadPool::new(4);
    let thread_ids = Arc::new(std::sync::Mutex::new(std::collections::HashSet::new()));

    let barrier = Arc::new(Barrier::new(4));

    for _ in 0..4 {
        let ids = Arc::clone(&thread_ids);
        let barrier_clone = Arc::clone(&barrier);
        pool.execute(move || {
            ids.lock().unwrap().insert(thread::current().id());
            // Hold all threads until all arrive, ensuring we see distinct IDs
            barrier_clone.wait();
        });
    }

    thread::sleep(Duration::from_millis(500));

    let unique_threads = thread_ids.lock().unwrap().len();
    assert_eq!(
        unique_threads, 4,
        "Expected 4 unique thread IDs, got {}",
        unique_threads
    );
}

// ============================================================================
// POOL SHUTDOWN (DROP)
// ============================================================================

#[test]
fn test_pool_drop_completes_pending_jobs() {
    let counter = Arc::new(AtomicUsize::new(0));

    {
        let pool = ThreadPool::new(2);
        for _ in 0..5 {
            let counter_clone = Arc::clone(&counter);
            pool.execute(move || {
                thread::sleep(Duration::from_millis(10));
                counter_clone.fetch_add(1, Ordering::SeqCst);
            });
        }
        // Give jobs a moment to be picked up before drop
        thread::sleep(Duration::from_millis(200));
        // pool is dropped here - Drop sends Terminate and joins threads
    }

    assert_eq!(
        counter.load(Ordering::SeqCst),
        5,
        "All submitted jobs should complete before pool fully shuts down"
    );
}

#[test]
fn test_pool_drop_is_graceful() {
    // Verify that drop doesn't panic or hang
    let pool = ThreadPool::new(4);

    pool.execute(|| {
        thread::sleep(Duration::from_millis(10));
    });

    // Explicit drop should complete without panic
    drop(pool);
}

#[test]
fn test_pool_drop_without_jobs() {
    // Pool with no jobs submitted should still shut down cleanly
    let pool = ThreadPool::new(4);
    drop(pool);
}

// ============================================================================
// SINGLE WORKER POOL
// ============================================================================

#[test]
fn test_single_worker_processes_all_jobs() {
    let pool = ThreadPool::new(1);
    let counter = Arc::new(AtomicUsize::new(0));

    for _ in 0..20 {
        let counter_clone = Arc::clone(&counter);
        pool.execute(move || {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });
    }

    thread::sleep(Duration::from_millis(500));

    assert_eq!(counter.load(Ordering::SeqCst), 20);
}

#[test]
fn test_single_worker_sequential_execution() {
    // With a single worker, jobs must execute in FIFO order
    let pool = ThreadPool::new(1);
    let order = Arc::new(std::sync::Mutex::new(Vec::new()));

    for i in 0..5 {
        let order_clone = Arc::clone(&order);
        pool.execute(move || {
            order_clone.lock().unwrap().push(i);
        });
    }

    thread::sleep(Duration::from_millis(200));

    let result = order.lock().unwrap().clone();
    assert_eq!(result, vec![0, 1, 2, 3, 4], "Single worker should process jobs in FIFO order");
}

// ============================================================================
// STRESS TEST
// ============================================================================

#[test]
fn test_stress_many_jobs() {
    let pool = ThreadPool::new(8);
    let counter = Arc::new(AtomicUsize::new(0));

    let num_jobs = 1000;

    for _ in 0..num_jobs {
        let counter_clone = Arc::clone(&counter);
        pool.execute(move || {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });
    }

    // Wait long enough for 1000 lightweight jobs
    thread::sleep(Duration::from_millis(1000));

    assert_eq!(counter.load(Ordering::SeqCst), num_jobs);
}

// ============================================================================
// JOB CAPTURES ENVIRONMENT CORRECTLY
// ============================================================================

#[test]
fn test_job_captures_move_values() {
    let pool = ThreadPool::new(2);
    let result = Arc::new(std::sync::Mutex::new(String::new()));

    let msg = String::from("hello from thread pool");
    let result_clone = Arc::clone(&result);

    pool.execute(move || {
        *result_clone.lock().unwrap() = msg;
    });

    thread::sleep(Duration::from_millis(100));

    assert_eq!(*result.lock().unwrap(), "hello from thread pool");
}

#[test]
fn test_job_with_computed_result() {
    let pool = ThreadPool::new(2);
    let sum = Arc::new(AtomicUsize::new(0));

    for i in 1..=10 {
        let sum_clone = Arc::clone(&sum);
        pool.execute(move || {
            sum_clone.fetch_add(i, Ordering::SeqCst);
        });
    }

    thread::sleep(Duration::from_millis(200));

    // Sum of 1..=10 = 55
    assert_eq!(sum.load(Ordering::SeqCst), 55);
}

// ============================================================================
// WORKER COUNT
// ============================================================================

#[test]
fn test_worker_count() {
    for size in [1, 2, 4, 8, 16] {
        let pool = ThreadPool::new(size);
        assert_eq!(pool.worker_count(), size);
    }
}
