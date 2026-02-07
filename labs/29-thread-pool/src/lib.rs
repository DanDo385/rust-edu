// Lab 29: Thread Pool
//
// A custom thread pool implementation demonstrating worker threads, job queues,
// and the Arc<Mutex<...>> pattern for shared state. This is the foundation
// of production servers and async runtimes.
//
// Key Concepts:
// - Worker threads that wait for jobs on a shared channel
// - Arc<Mutex<Receiver>> pattern for shared ownership of a single receiver
// - RAII-based graceful shutdown via Drop trait
// - FnOnce + Send + 'static bounds for thread-safe closures

use std::sync::{Arc, Mutex, mpsc};
use std::thread;

// ============================================================================
// TYPE ALIASES
// ============================================================================
// Jobs are boxed closures that can be sent between threads.
// FnOnce: can be called once (may consume captured environment)
// Send: can be transferred to another thread
// 'static: does not borrow non-static data

/// A job is a boxed closure that runs once on a worker thread.
pub type Job = Box<dyn FnOnce() + Send + 'static>;

// ============================================================================
// MESSAGE ENUM (INTERNAL)
// ============================================================================
// Workers receive either a new job or a terminate signal.

enum Message {
    NewJob(Job),
    Terminate,
}

// ============================================================================
// THREAD POOL
// ============================================================================

/// A thread pool that distributes work across a fixed number of worker threads.
///
/// Jobs are submitted via `execute()` and run on the next available worker.
/// When the pool is dropped, all workers are gracefully shut down.
///
/// # Ownership Model
/// ```text
/// ThreadPool
///   ├── sender: Option<Sender<Message>>   -- sends jobs to workers
///   └── workers: Vec<Worker>              -- each holds a JoinHandle
///         └── (thread) ──> Arc<Mutex<Receiver<Message>>>
///                          ^--- shared by all worker threads
/// ```
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Message>>,
}

impl ThreadPool {
    /// Creates a new ThreadPool with the specified number of workers.
    ///
    /// Each worker spawns a thread that blocks on recv(), waiting for jobs.
    /// The receiver is wrapped in Arc<Mutex<...>> so all workers share it:
    /// - Arc: multiple ownership (one clone per worker thread)
    /// - Mutex: only one worker can call recv() at a time
    ///
    /// # Panics
    /// Panics if `size` is 0.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0, "Thread pool size must be greater than 0");

        // Create a channel for sending jobs to workers
        let (sender, receiver) = mpsc::channel();

        // Wrap receiver in Arc<Mutex<...>> so all workers can share it
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // Each worker gets a clone of Arc (increments ref count)
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    /// Executes a closure on a worker thread from the pool.
    ///
    /// The closure is boxed and sent through the channel. The next worker
    /// to call recv() on the shared receiver will pick it up and execute it.
    ///
    /// # Bounds
    /// - `FnOnce()`: closure can be called once (may consume environment)
    /// - `Send`: closure can be sent to another thread
    /// - `'static`: closure doesn't reference non-static data
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender
            .as_ref()
            .unwrap()
            .send(Message::NewJob(job))
            .unwrap();
    }

    /// Returns the number of worker threads in the pool.
    pub fn worker_count(&self) -> usize {
        self.workers.len()
    }
}

// ============================================================================
// GRACEFUL SHUTDOWN WITH DROP (RAII)
// ============================================================================
// When ThreadPool is dropped, we:
// 1. Send Terminate to each worker
// 2. Join each worker thread (wait for completion)

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // Send terminate message to all workers
        for _ in &self.workers {
            self.sender
                .as_ref()
                .unwrap()
                .send(Message::Terminate)
                .unwrap();
        }

        // Wait for each worker to finish
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

// ============================================================================
// WORKER
// ============================================================================

/// A worker thread that listens for jobs on a shared receiver.
///
/// Each worker runs a loop:
/// 1. Lock the mutex on the receiver
/// 2. Call recv() to get the next message (blocks if queue is empty)
/// 3. Release the lock (automatic when recv() returns)
/// 4. Execute the job or terminate
pub struct Worker {
    /// The worker's identifier (for debugging/logging).
    pub id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    /// Creates a new worker that listens for jobs on the given receiver.
    ///
    /// Spawns a thread that loops:
    /// - Lock mutex -> recv() -> release lock -> execute job
    /// - On Terminate message, breaks out of the loop
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                // Lock the mutex to get access to the receiver.
                // The lock is released as soon as recv() returns because
                // the MutexGuard is dropped at the semicolon.
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        job();
                    }
                    Message::Terminate => {
                        break;
                    }
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

// ============================================================================
// OWNERSHIP & MEMORY MODEL
// ============================================================================
//
// THREAD POOL CREATION:
//   ThreadPool::new(4) creates:
//   - 1 Sender<Message> (owned by ThreadPool)
//   - 1 Receiver<Message> wrapped in Arc<Mutex<...>>
//   - 4 Worker structs, each with a JoinHandle
//   - 4 OS threads, each holding an Arc clone to the receiver
//
// JOB EXECUTION:
//   execute(closure) does:
//   1. Box the closure (heap-allocate)
//   2. Wrap in Message::NewJob
//   3. Send through channel (ownership transfers to receiver)
//   4. Worker locks mutex, calls recv() (takes ownership of Message)
//   5. Extracts Job from Message, calls job() (closure runs)
//   6. Job is dropped after execution (heap memory freed)
//
// DROP SEQUENCE:
//   When ThreadPool drops:
//   1. Send Terminate to each worker (N messages for N workers)
//   2. Each worker breaks out of loop when it receives Terminate
//   3. join() waits for each thread to finish
//   4. JoinHandles are dropped
//   5. Arc<Mutex<Receiver>> ref count reaches 0, Receiver is dropped
//   6. Sender is dropped (channel fully closed)
//
// WHY ARC<MUTEX<RECEIVER>>:
//   - Receiver is not Clone (only one can exist)
//   - recv() needs &mut self, but Arc only gives &self
//   - Mutex provides interior mutability: &self -> &mut Receiver
//   - Arc provides shared ownership across threads
