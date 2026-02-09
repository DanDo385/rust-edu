//! # Lab 21: Async Basics
//!
//! Asynchronous programming with async/await enables concurrent I/O without thread overhead.
//! Async functions return Futures - lazy computations that don't run until awaited.
//! Rust's async is zero-cost: it compiles to efficient state machines.

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

/// A simple custom Future that completes immediately.
///
/// **Teaching: Future trait**
/// - Future has one required method: poll()
/// - poll() returns Poll<Output> (Ready or Pending)
/// - Ready means "I'm done, here's the result"
/// - Pending means "I'm not done yet, call me again"
///
/// **From the borrow checker's perspective:**
/// - Futures are trait objects that represent pending work
/// - They can borrow data or own it
/// - async/await syntax desugars to Future implementations
pub struct SimpleFuture {
    done: bool,
}

impl SimpleFuture {
    /// Create a new simple future
    pub fn new() -> Self {
        SimpleFuture { done: false }
    }
}

impl Default for SimpleFuture {
    fn default() -> Self {
        Self::new()
    }
}

impl Future for SimpleFuture {
    type Output = i32;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        // **Why poll() has Pin and Context:**
        // - Pin: Future can't move once started (important for async!)
        // - Context: Lets us wake the executor when ready
        if self.done {
            Poll::Ready(42)
        } else {
            self.done = true;
            Poll::Pending  // Next time we're polled, return Ready
        }
    }
}

/// A future that adds two numbers (teaching example).
///
/// **Teaching: Polling in action**
/// - First poll(): not ready yet (busy flag set to false)
/// - Executor polls again
/// - Second poll(): ready with result
pub struct AddFuture {
    a: i32,
    b: i32,
    polled: bool,
}

impl AddFuture {
    /// Create a future that will add two numbers
    pub fn new(a: i32, b: i32) -> Self {
        AddFuture {
            a,
            b,
            polled: false,
        }
    }
}

impl Future for AddFuture {
    type Output = i32;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.polled {
            Poll::Ready(self.a + self.b)
        } else {
            self.polled = true;
            Poll::Pending
        }
    }
}

/// An async function that returns a value.
///
/// **Teaching: async fn syntax**
/// - async fn returns impl Future<Output=T>
/// - Function body doesn't run until awaited
/// - .await waits for the future to complete
/// - Compiler generates a state machine for suspend points
pub async fn async_value() -> i32 {
    // **Why this is async:**
    // In real code, this might be network I/O
    // Here we just return immediately
    42
}

/// An async function that combines multiple operations.
///
/// **Teaching: Multiple awaits**
/// - Each .await is a potential suspend point
/// - Executor can switch to other tasks while waiting
/// - State machine tracks which suspend point we're at
pub async fn async_sequence(x: i32) -> i32 {
    // **First operation:**
    let result1 = async_add(x, 10).await;

    // **Second operation:**
    let result2 = async_multiply(result1, 2).await;

    result2
}

/// Helper async function
pub async fn async_add(a: i32, b: i32) -> i32 {
    a + b
}

/// Another helper async function
pub async fn async_multiply(a: i32, b: i32) -> i32 {
    a * b
}

/// A future that completes with computed result.
///
/// **Teaching: Custom futures with state**
/// - Futures can maintain state between polls
/// - Useful for modeling complex async operations
pub struct CountingFuture {
    count: i32,
    max: i32,
}

impl CountingFuture {
    /// Create a future that counts from 0 to max
    pub fn new(max: i32) -> Self {
        CountingFuture { count: 0, max }
    }
}

impl Future for CountingFuture {
    type Output = i32;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.count += 1;

        if self.count > self.max {
            Poll::Ready(self.count)
        } else {
            Poll::Pending
        }
    }
}

/// Demonstrates why async is useful for I/O.
///
/// **Teaching: Async vs blocking**
/// - Blocking thread: waits, wastes 1 OS thread per operation
/// - Async task: yields control while waiting, 1 thread handles many tasks
/// - Memory: ~2MB per thread vs ~few KB per async task
pub fn explain_async_benefit() -> &'static str {
    // **Why choose async:**
    // 1. Web server: 10,000 concurrent connections
    //    - Threads: 10,000 OS threads = ~20GB memory!
    //    - Async: 1 thread handling 10,000 tasks = few MB
    //
    // 2. Sequential I/O: reading 100 files
    //    - Blocking: 100 file operations block the thread
    //    - Async: 100 tasks run concurrently on few threads
    //
    // 3. Most of I/O time is waiting
    //    - Thread hangs doing nothing while disk/network works
    //    - Async task yields, letting other tasks use the thread
    "Async programming enables many concurrent operations with few threads."
}

/// A future-based retry mechanism.
///
/// **Teaching: Futures as composable abstractions**
/// - Futures can be combined and transformed
/// - Building blocks for complex async logic
pub struct RetryFuture {
    attempt: u32,
    max_attempts: u32,
}

impl RetryFuture {
    /// Create a future that retries up to max_attempts times
    pub fn new(max_attempts: u32) -> Self {
        RetryFuture {
            attempt: 0,
            max_attempts,
        }
    }
}

impl Future for RetryFuture {
    type Output = Result<String, String>;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.attempt += 1;

        if self.attempt <= self.max_attempts {
            if self.attempt == self.max_attempts {
                Poll::Ready(Ok("Success".to_string()))
            } else {
                Poll::Pending
            }
        } else {
            Poll::Ready(Err("Max attempts exceeded".to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_future_creates() {
        let _future = SimpleFuture::new();
        // Future created but not polled
    }

    #[test]
    fn test_add_future_creates() {
        let _future = AddFuture::new(5, 3);
        // Future created but not polled
    }

    #[test]
    fn test_counting_future_creates() {
        let _future = CountingFuture::new(10);
        // Future created but not polled
    }

    #[test]
    fn test_retry_future_creates() {
        let _future = RetryFuture::new(3);
        // Future created but not polled
    }

    #[test]
    fn test_async_benefit_message() {
        let msg = explain_async_benefit();
        assert!(!msg.is_empty());
        assert!(msg.to_lowercase().contains("async"));
    }
}
