//! # Lab 20: Multithreading Basics
//!
//! Rust's type system prevents data races at compile time through Send and Sync traits.
//! If your code compiles, it's thread-safe! Two concurrency patterns: message passing (channels)
//! and shared state (Arc<Mutex<T>>).

use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;

/// Spawns multiple threads and returns their results.
///
/// **Teaching: thread::spawn and JoinHandle**
/// - spawn() creates a new OS thread
/// - Returns JoinHandle to wait for completion
/// - join() blocks until thread finishes
/// - unwrap() gets the thread's return value
///
/// **From the borrow checker's perspective:**
/// - Each thread needs ownership of data (via move)
/// - JoinHandle ensures we can wait for completion
/// - Rust prevents thread from accessing dropped data
pub fn spawn_workers(count: usize) -> Vec<usize> {
    let mut handles = vec![];

    // Spawn threads
    for i in 0..count {
        let handle = thread::spawn(move || {
            // Each thread owns its own data (i)
            let result = i * 2;
            result
        });
        handles.push(handle);
    }

    // Wait for all threads and collect results
    // join().unwrap() panics if the joined thread panicked - acceptable for demo
    handles.into_iter().map(|h| h.join().unwrap()).collect()
}

/// Demonstrates ownership transfer to threads with move.
///
/// **Teaching: move keyword and ownership**
/// - `move` transfers ownership from parent to thread
/// - Thread outlives parent function, so must own data
/// - Parent can't use data after move
/// - Prevents use-after-free bugs in concurrent code
pub fn process_data_in_thread(numbers: Vec<i32>) -> i32 {
    // **Why move is required:**
    // The thread might outlive this function.
    // If we borrowed numbers, it would be dropped while thread still runs!
    let handle = thread::spawn(move || {
        numbers.iter().sum()
    });

    handle.join().unwrap()
}

/// Shared counter using Arc<Mutex<T>>.
///
/// **Teaching: Shared mutable state**
/// - Arc: Atomic Reference Counted (thread-safe Rc)
/// - Mutex: Mutual Exclusion lock
/// - Together: Share ownership and control access
/// - lock() returns Result<MutexGuard> - panics on poisoned mutex
#[derive(Clone)]
pub struct SharedCounter {
    count: Arc<Mutex<i32>>,
}

impl SharedCounter {
    /// Create a new shared counter
    pub fn new(initial: i32) -> Self {
        SharedCounter {
            count: Arc::new(Mutex::new(initial)),
        }
    }

    /// Increment the counter (thread-safe)
    ///
    /// **Why lock() is needed:**
    /// - Multiple threads can't access simultaneously
    /// - lock() ensures exclusive access
    /// - guard is automatically released when dropped
    /// - lock().unwrap() panics if the mutex is poisoned (another thread panicked while holding it)
    pub fn increment(&self) {
        let mut guard = self.count.lock().unwrap();
        *guard += 1;
    }

    /// Get current value (thread-safe)
    pub fn get(&self) -> i32 {
        *self.count.lock().unwrap()
    }

    /// Add a value (thread-safe)
    pub fn add(&self, value: i32) {
        *self.count.lock().unwrap() += value;
    }
}

/// Returns a function that can be sent between threads.
///
/// **Teaching: Send trait**
/// - Most types are Send (can be transferred to threads)
/// - Rc is NOT Send
/// - Arc IS Send
/// - Compiler enforces this
pub fn thread_worker(id: usize, work_count: usize) -> i32 {
    // **Why this works:**
    // - Closure captures id and work_count (both Copy types, thus Send)
    // - We can move this to another thread safely
    let total: i32 = (0..work_count).map(|i| (id as i32) + (i as i32)).sum();
    total
}

/// Demonstrates message passing between threads.
///
/// **Teaching: Message passing (channels)**
/// - "Don't share memory by communicating; communicate by sharing memory"
/// - Sender and Receiver transfer ownership of messages
/// - No shared state = no races
/// - Like Go channels
pub fn sum_parallel(numbers: Vec<i32>) -> i32 {
    let (tx, rx) = mpsc::channel();
    let chunk_size = (numbers.len() + 1) / 2;  // Split into 2 chunks

    // Split work between threads
    let chunk1 = numbers[..chunk_size].to_vec();
    let chunk2 = numbers[chunk_size..].to_vec();

    // Thread 1: process chunk1
    let tx1 = tx.clone();
    thread::spawn(move || {
        let sum: i32 = chunk1.iter().sum();
        let _ = tx1.send(sum);
    });

    // Thread 2: process chunk2
    let tx2 = tx.clone();
    thread::spawn(move || {
        let sum: i32 = chunk2.iter().sum();
        let _ = tx2.send(sum);
    });

    // Drop remaining senders so receiver knows when done
    drop(tx);

    // Receive results
    rx.iter().sum()
}

/// A simple parallel map operation.
///
/// **Teaching: Distributing work across threads**
/// - Each thread gets a chunk of work
/// - Joins all threads to wait for completion
pub fn parallel_map<F>(items: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32 + Send + Sync + 'static + Clone,
{
    // **Why F needs Send + Sync:**
    // - Send: can be moved to threads
    // - Sync: can be shared between threads
    // - 'static: no borrowed data
    let num_threads = 2;
    let chunk_size = (items.len() + num_threads - 1) / num_threads;

    let mut handles = vec![];

    for chunk in items.chunks(chunk_size) {
        let chunk = chunk.to_vec();
        let f = f.clone();

        let handle = thread::spawn(move || {
            chunk.iter().map(|&x| f(x)).collect::<Vec<_>>()
        });

        handles.push(handle);
    }

    handles
        .into_iter()
        .flat_map(|h| h.join().unwrap())
        .collect()
}

/// Demonstrates multiple producer, single consumer pattern.
///
/// **Teaching: Multiple Senders**
/// - mpsc: Multi-Producer, Single-Consumer
/// - Multiple threads can send on same channel
/// - clone() Sender to share it
pub fn counting_threads(thread_count: usize, iterations: usize) -> i32 {
    let (tx, rx) = mpsc::channel();

    for id in 0..thread_count {
        let tx = tx.clone();

        thread::spawn(move || {
            for i in 0..iterations {
                let value = (id * iterations + i) as i32;
                let _ = tx.send(value);
            }
        });
    }

    // Drop original sender
    drop(tx);

    // Sum all values from all threads
    rx.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spawn_workers() {
        let results = spawn_workers(5);
        assert_eq!(results.len(), 5);
        for (i, &result) in results.iter().enumerate() {
            assert_eq!(result, i * 2);
        }
    }

    #[test]
    fn test_process_data_in_thread() {
        let data = vec![1, 2, 3, 4, 5];
        let result = process_data_in_thread(data);
        assert_eq!(result, 15);
    }

    #[test]
    fn test_shared_counter_single_thread() {
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
    fn test_thread_worker() {
        let result = thread_worker(5, 10);
        assert_eq!(result, 5 * 10 + (0..10).sum::<usize>() as i32);
    }

    #[test]
    fn test_sum_parallel() {
        let data = vec![1, 2, 3, 4, 5];
        let result = sum_parallel(data);
        assert_eq!(result, 15);
    }

    #[test]
    fn test_parallel_map() {
        let data = vec![1, 2, 3, 4, 5];
        let result = parallel_map(data, |x| x * 2);
        assert_eq!(result.len(), 5);
        assert_eq!(result.iter().sum::<i32>(), 30);
    }

    #[test]
    fn test_counting_threads() {
        let result = counting_threads(3, 5);
        // 3 threads, 5 iterations each = 0..15
        assert_eq!(result, (0..15).sum::<usize>() as i32);
    }
}
