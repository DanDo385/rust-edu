//! Integration tests for Lab 21: Async Basics
//!
//! These tests demonstrate the Future trait by manually polling futures.
//! In real async code, the runtime (tokio) handles polling for you.

use async_basics::solution::{
    SimpleFuture, AddFuture, CountingFuture, RetryFuture,
    async_value, async_add, async_multiply, async_sequence,
    explain_async_benefit,
};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Wake, Waker};
use std::sync::Arc;

// ============================================================================
// HELPER: Manual Waker for testing
// ============================================================================

/// A dummy waker for testing (doesn't actually wake anything)
struct DummyWaker;

impl Wake for DummyWaker {
    fn wake(self: Arc<Self>) {}
}

fn dummy_waker() -> Waker {
    Waker::from(Arc::new(DummyWaker))
}

// ============================================================================
// HELPER: Manual future polling
// ============================================================================

/// Manually poll a future to test it
fn poll_once<F: Future + Unpin>(mut future: F) -> Poll<F::Output> {
    let waker = dummy_waker();
    let mut cx = Context::from_waker(&waker);
    Pin::new(&mut future).poll(&mut cx)
}

/// Poll a future until it's ready (for testing custom futures)
fn block_on<F: Future + Unpin>(mut future: F) -> F::Output {
    let waker = dummy_waker();
    let mut cx = Context::from_waker(&waker);

    // This is NOT how real async runtimes work!
    // They use efficient event loops, not busy loops.
    // This is just for testing futures.
    loop {
        match Pin::new(&mut future).poll(&mut cx) {
            Poll::Ready(output) => return output,
            Poll::Pending => {
                // In real runtime, this would wait for events
                // Here we just loop (inefficient but works for tests)
            }
        }
    }
}

/// Poll a boxed future until it's ready (for testing async functions)
fn block_on_boxed<F: Future>(mut future: Pin<Box<F>>) -> F::Output {
    let waker = dummy_waker();
    let mut cx = Context::from_waker(&waker);

    loop {
        match future.as_mut().poll(&mut cx) {
            Poll::Ready(output) => return output,
            Poll::Pending => {}
        }
    }
}

// ============================================================================
// SIMPLE FUTURE TESTS
// ============================================================================

#[test]
fn test_simple_future_initially_pending() {
    let future = SimpleFuture::new();
    let result = poll_once(future);
    assert!(matches!(result, Poll::Pending));
}

#[test]
fn test_simple_future_eventually_ready() {
    let future = SimpleFuture::new();
    let result = block_on(future);
    assert_eq!(result, 42);
}

#[test]
fn test_simple_future_two_polls() {
    let mut future = SimpleFuture::new();
    let waker = dummy_waker();
    let mut cx = Context::from_waker(&waker);

    // First poll: should be Pending
    let result1 = Pin::new(&mut future).poll(&mut cx);
    assert!(matches!(result1, Poll::Pending));

    // Second poll: should be Ready
    let result2 = Pin::new(&mut future).poll(&mut cx);
    assert_eq!(result2, Poll::Ready(42));
}

// ============================================================================
// ADD FUTURE TESTS
// ============================================================================

#[test]
fn test_add_future_creates() {
    let _future = AddFuture::new(5, 3);
}

#[test]
fn test_add_future_simple_addition() {
    let future = AddFuture::new(5, 3);
    let result = block_on(future);
    assert_eq!(result, 8);
}

#[test]
fn test_add_future_negative() {
    let future = AddFuture::new(-5, 3);
    let result = block_on(future);
    assert_eq!(result, -2);
}

#[test]
fn test_add_future_zero() {
    let future = AddFuture::new(0, 0);
    let result = block_on(future);
    assert_eq!(result, 0);
}

#[test]
fn test_add_future_large_numbers() {
    let future = AddFuture::new(1000, 2000);
    let result = block_on(future);
    assert_eq!(result, 3000);
}

// ============================================================================
// COUNTING FUTURE TESTS
// ============================================================================

#[test]
fn test_counting_future_zero_max() {
    let future = CountingFuture::new(0);
    let result = block_on(future);
    assert_eq!(result, 1);  // Counts to 1, exceeds max of 0
}

#[test]
fn test_counting_future_single() {
    let future = CountingFuture::new(1);
    let result = block_on(future);
    assert_eq!(result, 2);  // Counts 1, 2 (exceeds max)
}

#[test]
fn test_counting_future_multiple() {
    let future = CountingFuture::new(5);
    let result = block_on(future);
    assert_eq!(result, 6);  // Counts from 1 to 6
}

#[test]
fn test_counting_future_large() {
    let future = CountingFuture::new(100);
    let result = block_on(future);
    assert_eq!(result, 101);
}

// ============================================================================
// RETRY FUTURE TESTS
// ============================================================================

#[test]
fn test_retry_future_zero_attempts() {
    let future = RetryFuture::new(0);
    let result = block_on(future);
    assert!(result.is_err());
}

#[test]
fn test_retry_future_single_attempt() {
    let future = RetryFuture::new(1);
    let result = block_on(future);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Success");
}

#[test]
fn test_retry_future_multiple_attempts() {
    let future = RetryFuture::new(3);
    let result = block_on(future);
    assert!(result.is_ok());
}

// ============================================================================
// ASYNC FUNCTION TESTS (using block_on_boxed)
// ============================================================================

#[test]
fn test_async_value() {
    // Note: async_value() returns a Future
    // async fns are not Unpin, so we use Box::pin
    let future = Box::pin(async_value());
    let result = block_on_boxed(future);
    assert_eq!(result, 42);
}

#[test]
fn test_async_add() {
    let future = Box::pin(async_add(10, 5));
    let result = block_on_boxed(future);
    assert_eq!(result, 15);
}

#[test]
fn test_async_multiply() {
    let future = Box::pin(async_multiply(6, 7));
    let result = block_on_boxed(future);
    assert_eq!(result, 42);
}

#[test]
fn test_async_sequence() {
    // This chains multiple async operations
    let future = Box::pin(async_sequence(5));
    let result = block_on_boxed(future);
    // 5 + 10 = 15, then 15 * 2 = 30
    assert_eq!(result, 30);
}

#[test]
fn test_async_sequence_zero() {
    let future = Box::pin(async_sequence(0));
    let result = block_on_boxed(future);
    // 0 + 10 = 10, then 10 * 2 = 20
    assert_eq!(result, 20);
}

#[test]
fn test_async_sequence_negative() {
    let future = Box::pin(async_sequence(-5));
    let result = block_on_boxed(future);
    // -5 + 10 = 5, then 5 * 2 = 10
    assert_eq!(result, 10);
}

// ============================================================================
// EXPLANATION TESTS
// ============================================================================

#[test]
fn test_async_benefit_explanation() {
    let explanation = explain_async_benefit();
    assert!(!explanation.is_empty());
    assert!(explanation.to_lowercase().contains("async"));
    assert!(explanation.contains("threads"));
}

// ============================================================================
// FUTURE TRAIT BEHAVIOR TESTS
// ============================================================================

#[test]
fn test_simple_future_pending_then_ready() {
    let mut future = SimpleFuture::new();
    let waker = dummy_waker();
    let mut cx = Context::from_waker(&waker);

    // First poll: Pending
    match Pin::new(&mut future).poll(&mut cx) {
        Poll::Pending => {},
        Poll::Ready(_) => panic!("Should be Pending on first poll"),
    }

    // Second poll: Ready
    match Pin::new(&mut future).poll(&mut cx) {
        Poll::Ready(value) => assert_eq!(value, 42),
        Poll::Pending => panic!("Should be Ready on second poll"),
    }
}

#[test]
fn test_multiple_futures_independently() {
    let f1 = AddFuture::new(1, 2);
    let f2 = AddFuture::new(3, 4);

    let r1 = block_on(f1);
    let r2 = block_on(f2);

    assert_eq!(r1, 3);
    assert_eq!(r2, 7);
}

#[test]
fn test_counting_future_state_progression() {
    let mut future = CountingFuture::new(2);
    let waker = dummy_waker();
    let mut cx = Context::from_waker(&waker);

    // Poll multiple times to see state progression
    let p1 = Pin::new(&mut future).poll(&mut cx);
    assert!(matches!(p1, Poll::Pending));

    let p2 = Pin::new(&mut future).poll(&mut cx);
    assert!(matches!(p2, Poll::Pending));

    let p3 = Pin::new(&mut future).poll(&mut cx);
    assert_eq!(p3, Poll::Ready(3));
}

// ============================================================================
// ASYNC COMPOSITION TESTS
// ============================================================================

#[test]
fn test_async_chaining() {
    // Test chaining multiple async operations
    let f1 = Box::pin(async_add(2, 3));  // 5
    let f2 = Box::pin(async_multiply(10, 4));  // 40

    let r1 = block_on_boxed(f1);
    let r2 = block_on_boxed(f2);

    assert_eq!(r1, 5);
    assert_eq!(r2, 40);
}

#[test]
fn test_async_sequence_varies_with_input() {
    let results: Vec<_> = (0..5)
        .map(|x| {
            let future = Box::pin(async_sequence(x));
            block_on_boxed(future)
        })
        .collect();

    // For each x: (x + 10) * 2
    assert_eq!(results[0], 20);  // (0 + 10) * 2
    assert_eq!(results[1], 22);  // (1 + 10) * 2
    assert_eq!(results[2], 24);  // (2 + 10) * 2
    assert_eq!(results[3], 26);  // (3 + 10) * 2
    assert_eq!(results[4], 28);  // (4 + 10) * 2
}

// ============================================================================
// FUTURE CONCEPTS TESTS
// ============================================================================

#[test]
fn test_futures_are_lazy() {
    // Creating a future doesn't execute it
    let _future = AddFuture::new(5, 3);
    // No computation happens yet - futures are lazy!
}

#[test]
fn test_futures_can_be_stored() {
    let f1 = AddFuture::new(1, 2);
    let f2 = AddFuture::new(3, 4);
    let f3 = AddFuture::new(5, 6);

    // All futures created but not executed
    let futures = vec![f1, f2, f3];

    // Execute them all
    let results: Vec<i32> = futures
        .into_iter()
        .map(|f| block_on(f))
        .collect();

    assert_eq!(results, vec![3, 7, 11]);
}

#[test]
fn test_futures_can_fail() {
    // Retry future can return error
    let future = RetryFuture::new(0);
    let result = block_on(future);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Max attempts exceeded");
}

#[test]
fn test_futures_can_succeed() {
    // Retry future can return success
    let future = RetryFuture::new(5);
    let result = block_on(future);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Success");
}

// ============================================================================
// REAL-WORLD PATTERN TESTS
// ============================================================================

#[test]
fn test_simulating_async_workflow() {
    // Simulate a workflow: fetch data, process it, save it
    let fetch = Box::pin(async_add(10, 20));  // "fetch" 30
    let process_result = block_on_boxed(fetch);
    assert_eq!(process_result, 30);

    let save = Box::pin(async_multiply(process_result, 2));  // "save" (multiply by 2)
    let final_result = block_on_boxed(save);
    assert_eq!(final_result, 60);
}

#[test]
fn test_error_handling_with_retry() {
    // In real code, retry would test actual operations
    let retry = RetryFuture::new(1);
    let result = block_on(retry);

    match result {
        Ok(msg) => assert_eq!(msg, "Success"),
        Err(_) => panic!("Expected success"),
    }
}
