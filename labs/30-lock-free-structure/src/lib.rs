//! # Lock-Free Stack - Your Implementation
//!
//! Student-facing API for a Treiber-style lock-free stack.

use std::marker::PhantomData;
use std::ptr;
use std::sync::atomic::{AtomicPtr, Ordering};

pub struct Node<T> {
    _marker: PhantomData<T>,
}

pub struct LockFreeStack<T> {
    head: AtomicPtr<Node<T>>,
}

impl<T> LockFreeStack<T> {
    pub fn new() -> Self {
        // TODO: Initialize an empty atomic head pointer.
        let _ = ptr::null_mut::<Node<T>>();
        todo!("Initialize the head of the stack")
    }

    pub fn push(&self, val: T) {
        // TODO: Implement Treiber push with compare_exchange loop.
        let _ = (self, val, Ordering::Acquire, Ordering::Release, Ordering::Relaxed);
        todo!("Implement the lock-free push operation")
    }

    pub fn pop(&self) -> Option<T> {
        // TODO: Implement Treiber pop with compare_exchange loop.
        let _ = (self, Ordering::Acquire, Ordering::AcqRel);
        todo!("Implement the lock-free pop operation")
    }
}

impl<T> Drop for LockFreeStack<T> {
    fn drop(&mut self) {
        // TODO: Drain remaining nodes to avoid leaks.
    }
}

#[doc(hidden)]
pub mod solution;
