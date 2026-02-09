//! # Lab 21: Async Basics
//!
//! Student-facing API for custom Futures and async/await basics.

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct SimpleFuture {
    done: bool,
}

impl SimpleFuture {
    pub fn new() -> Self {
        // TODO: Initialize with done=false.
        todo!("Create SimpleFuture")
    }
}

impl Default for SimpleFuture {
    fn default() -> Self {
        Self::new()
    }
}

impl Future for SimpleFuture {
    type Output = i32;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        // TODO: Return Pending once, then Ready(42).
        let _ = self;
        todo!("Poll SimpleFuture")
    }
}

pub struct AddFuture {
    a: i32,
    b: i32,
    polled: bool,
}

impl AddFuture {
    pub fn new(a: i32, b: i32) -> Self {
        // TODO: Construct AddFuture with polled=false.
        let _ = (a, b);
        todo!("Create AddFuture")
    }
}

impl Future for AddFuture {
    type Output = i32;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        // TODO: Return Pending once, then Ready(a+b).
        let _ = self;
        todo!("Poll AddFuture")
    }
}

pub async fn async_value() -> i32 {
    // TODO: Return 42 from async context.
    todo!("Return async value")
}

pub async fn async_sequence(x: i32) -> i32 {
    // TODO: Await async_add(x,10), then async_multiply(result,2).
    let _ = x;
    todo!("Compose async operations")
}

pub async fn async_add(a: i32, b: i32) -> i32 {
    // TODO: Return a+b.
    let _ = (a, b);
    todo!("Async add")
}

pub async fn async_multiply(a: i32, b: i32) -> i32 {
    // TODO: Return a*b.
    let _ = (a, b);
    todo!("Async multiply")
}

pub struct CountingFuture {
    count: i32,
    max: i32,
}

impl CountingFuture {
    pub fn new(max: i32) -> Self {
        // TODO: Initialize with count=0.
        let _ = max;
        todo!("Create CountingFuture")
    }
}

impl Future for CountingFuture {
    type Output = i32;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        // TODO: Increment count; Pending until count > max.
        let _ = self;
        todo!("Poll CountingFuture")
    }
}

pub fn explain_async_benefit() -> &'static str {
    // TODO: Return short explanation string.
    todo!("Explain async benefits")
}

pub struct RetryFuture {
    attempt: u32,
    max_attempts: u32,
}

impl RetryFuture {
    pub fn new(max_attempts: u32) -> Self {
        // TODO: Initialize with attempt=0.
        let _ = max_attempts;
        todo!("Create RetryFuture")
    }
}

impl Future for RetryFuture {
    type Output = Result<String, String>;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        // TODO: Retry until max_attempts, returning Ok on final retry.
        let _ = self;
        todo!("Poll RetryFuture")
    }
}

#[doc(hidden)]
pub mod solution;
