// Project 26: Thread Pool
//
// A custom thread pool implementation demonstrating worker threads, job queues,
// and the Arc<Mutex<...>> pattern for shared state. This is the foundation
// of production servers and async runtimes.

use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Thread Pool Implementation ===\n");

    // ============================================================================
    // BASIC USAGE
    // ============================================================================
    // Create a thread pool with 4 worker threads
    let pool = ThreadPool::new(4);

    println!("Thread pool created with 4 workers\n");

    // Submit 10 jobs to the pool
    for i in 0..10 {
        pool.execute(move || {
            println!("Job {} started on thread {:?}", i, thread::current().id());

            // Simulate work
            thread::sleep(Duration::from_millis(100));

            println!("Job {} completed", i);
        });
    }

    println!("\nAll jobs submitted. Waiting for completion...\n");

    // Sleep to let jobs complete
    thread::sleep(Duration::from_secs(1));

    println!("\n=== Demonstrating Job Distribution ===\n");

    // Submit more jobs with different workloads
    for i in 0..8 {
        pool.execute(move || {
            let work_duration = (i % 3 + 1) * 50;
            println!("Heavy job {} starting ({}ms work)", i, work_duration);
            thread::sleep(Duration::from_millis(work_duration));
            println!("Heavy job {} done", i);
        });
    }

    thread::sleep(Duration::from_secs(1));

    println!("\n=== Thread Pool will now shut down gracefully ===");

    // When pool goes out of scope, Drop is called
    drop(pool);

    println!("\n=== All workers have shut down ===");
}

// ============================================================================
// TYPE ALIASES FOR CLARITY
// ============================================================================
// Jobs are boxed closures that can be sent between threads

type Job = Box<dyn FnOnce() + Send + 'static>;

// Message enum for worker communication
enum Message {
    NewJob(Job),
    Terminate,
}

// ============================================================================
// THREAD POOL STRUCTURE
// ============================================================================

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Message>>,
}

impl ThreadPool {
    /// Creates a new ThreadPool with the specified number of workers.
    ///
    /// # Panics
    /// Panics if size is 0
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0, "Thread pool size must be greater than 0");

        // Create a channel for sending jobs to workers
        let (sender, receiver) = mpsc::channel();

        // Wrap receiver in Arc<Mutex<...>> so all workers can share it
        // Arc = multiple ownership (one per worker thread)
        // Mutex = only one worker can receive() at a time
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
    /// The closure must be Send (can be sent to another thread)
    /// and 'static (doesn't reference non-static data)
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        // Send job through channel
        // Workers are listening on the other end
        self.sender.as_ref().unwrap().send(Message::NewJob(job)).unwrap();
    }
}

// ============================================================================
// GRACEFUL SHUTDOWN WITH DROP
// ============================================================================
// This is the RAII pattern - Resource Acquisition Is Initialization
// When ThreadPool is dropped, we clean up all workers

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("\nSending terminate message to all workers...");

        // First, send terminate message to all workers
        for _ in &self.workers {
            self.sender.as_ref().unwrap().send(Message::Terminate).unwrap();
        }

        println!("Waiting for workers to finish...");

        // Then, wait for each worker to finish
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }

        println!("All workers shut down successfully!");
    }
}

// ============================================================================
// WORKER STRUCTURE
// ============================================================================
// Each worker runs in its own thread, waiting for jobs

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    /// Creates a new worker that listens for jobs on the given receiver
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        // Spawn a new thread
        let thread = thread::spawn(move || {
            println!("Worker {} started", id);

            loop {
                // Lock the mutex to get access to the receiver
                // This lock is released when `message` goes out of scope
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        // Execute the job
                        job();
                    }
                    Message::Terminate => {
                        println!("Worker {} received terminate signal", id);
                        break;
                    }
                }
            }

            println!("Worker {} shutting down", id);
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

// ============================================================================
// WHAT RUST DOES UNDER THE HOOD
// ============================================================================
//
// 1. ARC REFERENCE COUNTING
//    Arc<Mutex<Receiver>> uses atomic operations to track references.
//    Each clone() increments the count atomically (thread-safe).
//    When count reaches 0, the Receiver is dropped.
//
// 2. MUTEX LOCKING
//    lock() uses OS primitives (futex on Linux, SRWLock on Windows).
//    If lock is held, thread is put to sleep (not spinning).
//    unlock() wakes one waiting thread.
//
// 3. CHANNEL IMPLEMENTATION
//    mpsc::channel() uses a linked list queue with locks.
//    send() locks, pushes to queue, unlocks.
//    recv() locks, pops from queue (or waits if empty), unlocks.
//
// 4. THREAD SPAWNING
//    thread::spawn() calls pthread_create (Unix) or CreateThread (Windows).
//    Each thread gets ~2MB stack (configurable with Builder).
//
// 5. FNONCE + SEND + 'STATIC
//    FnOnce: Closure can be called once (may consume environment)
//    Send: Closure can be sent to another thread (no non-thread-safe data)
//    'static: Closure doesn't reference non-static data (lives forever)
//    The compiler ENFORCES these at compile time!
//
// 6. DROP ORDER
//    When ThreadPool drops:
//    - sender.drop() closes the channel
//    - Workers see the Terminate message
//    - thread.join() waits for worker threads
//    - Arc<Mutex<Receiver>> is dropped when all workers finish
//    - Everything is cleaned up automatically, no memory leaks!

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. Thread pools reuse threads for better performance
// 2. Arc<Mutex<...>> pattern shares ownership across threads
// 3. mpsc channel acts as a job queue
// 4. Worker threads wait on recv(), which blocks efficiently
// 5. Drop trait enables graceful shutdown (RAII)
// 6. Compiler enforces Send + 'static for thread safety
// 7. No data races possible - checked at compile time!
// 8. Production pools add: timeouts, metrics, dynamic sizing

// ============================================================================
// WHY ARC<MUTEX<RECEIVER>> IS NECESSARY
// ============================================================================
//
// Q: Why not just pass Receiver to each thread?
// A: Receiver is not Clone. Only one owner can exist.
//
// Q: Why not Arc<Receiver>?
// A: recv() requires &mut self. Arc gives &self.
//
// Q: Why not just Mutex<Receiver>?
// A: Mutex isn't Send by itself when T isn't Sync.
//
// Q: Why not channels for each worker?
// A: Then we'd need to distribute jobs manually (load balancing).
//
// The Arc<Mutex<Receiver>> pattern is the idiomatic Rust solution!

// ============================================================================
// PERFORMANCE ANALYSIS
// ============================================================================
//
// OVERHEAD PER JOB:
// - Channel send: ~50-100ns
// - Mutex lock/unlock: ~20-50ns
// - Total: ~100-200ns
//
// THREAD CREATION (if we spawned per-job):
// - Thread spawn: ~1-2ms
// - 10,000x slower than thread pool!
//
// MEMORY:
// - 4 workers: ~8MB (stack) + channel queue
// - 4 threads per job: Unbounded memory growth!
//
// SCALABILITY:
// - Thread pool: Constant threads, queue grows
// - Spawn per job: Threads grow unbounded (system limit ~10k threads)

// ============================================================================
// COMMON BEGINNER MISTAKES
// ============================================================================
// ❌ Forgetting to clone Arc before moving into thread
//    let thread = thread::spawn(move || receiver.lock())  // ERROR
//    Fix: let receiver = Arc::clone(&receiver); before spawn
//
// ❌ Holding Mutex lock too long
//    let lock = receiver.lock().unwrap();
//    do_expensive_work();  // Other threads can't receive!
//    Fix: Drop lock before expensive work
//
// ❌ Not implementing Drop
//    Threads keep running after ThreadPool drops
//    Fix: Send Terminate message and join threads
//
// ❌ Using thread::spawn instead of pool
//    pool.execute(|| thread::spawn(|| {...}))  // Defeats the purpose!
//    Fix: Just use pool.execute(|| {...})
//
// ❌ Panicking in jobs without recovery
//    Job panics kill the worker thread permanently
//    Fix: Use std::panic::catch_unwind or respawn workers

// ============================================================================
// COMPARISON WITH REAL IMPLEMENTATIONS
// ============================================================================
//
// RAYON (work-stealing thread pool):
// - Per-worker job queues (not single queue)
// - Workers "steal" from others when idle
// - Better for parallel algorithms
// - More complex implementation
//
// TOKIO (async runtime):
// - Uses thread pool internally
// - Schedules async tasks (not blocking closures)
// - Work-stealing scheduler
// - Integration with async I/O
//
// ACTIX (actor framework):
// - Multiple thread pools (one per actor system)
// - Message-based (not closure-based)
// - Supervisor trees for fault tolerance
//
// Our implementation is simpler but demonstrates the core concepts!

// ============================================================================
// EXTENDING THIS IMPLEMENTATION
// ============================================================================
//
// To make this production-ready, add:
//
// 1. BOUNDED QUEUE
//    Use mpsc::sync_channel(capacity) instead of channel()
//    Backpressure when queue is full
//
// 2. PANIC RECOVERY
//    Catch panics and respawn workers:
//    if let Err(e) = std::panic::catch_unwind(|| job()) {
//        eprintln!("Job panicked: {:?}", e);
//    }
//
// 3. METRICS
//    Track: jobs completed, queue length, worker utilization
//    Expose via Prometheus or logs
//
// 4. TIMEOUTS
//    Kill jobs that run too long:
//    let timeout = Duration::from_secs(30);
//    Use thread::spawn with timeout and abort
//
// 5. PRIORITY QUEUE
//    Use BinaryHeap instead of FIFO queue
//    High-priority jobs run first
//
// 6. DYNAMIC SIZING
//    Add/remove workers based on queue length:
//    if queue_len > threshold { add_worker(); }
