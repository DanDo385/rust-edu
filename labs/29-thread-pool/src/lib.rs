//! # Lab 29: Thread Pool
//!
//! Student-facing API for a fixed worker thread pool.

pub type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        let _ = size;
        todo!("Create ThreadPool")
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let _ = f;
        todo!("Execute job")
    }

    pub fn worker_count(&self) -> usize {
        todo!("Return worker count")
    }
}

pub struct Worker {
    pub id: usize,
}

#[doc(hidden)]
pub mod solution;
