# Project 26: Thread Pool

## Overview
This project implements a custom thread pool for efficient parallel task execution. You'll learn about worker threads, job queues, and the thread pool pattern that's used in production servers and async runtimes. This is the same pattern used internally by web servers like Actix and database connection pools.

## Concepts Taught
- **Thread pool pattern** for reusing threads
- **Job queue** with `mpsc` channels
- **Arc<Mutex<...>>** for shared state across threads
- **Worker threads** that wait for tasks
- **Graceful shutdown** of thread pools
- **Message passing** between threads
- **RAII pattern** with Drop trait
- **Thread synchronization** and communication

## Why Thread Pools?

### The Problem with Spawning Threads
Creating a new thread for each task is expensive:
- Thread creation has overhead (~1-2ms per thread)
- Each thread uses ~2MB of stack memory
- Context switching slows down with too many threads
- No control over maximum concurrency

### The Thread Pool Solution
A thread pool:
- **Pre-spawns** a fixed number of worker threads
- **Reuses** threads for multiple tasks
- **Limits** maximum concurrent tasks (prevents resource exhaustion)
- **Queues** tasks when all workers are busy
- **Reduces** overhead and latency

**Real-world usage:**
- **Web servers**: Handle HTTP requests (Actix, Rocket)
- **Databases**: Connection pools (sqlx, diesel)
- **Tokio runtime**: Uses thread pools internally
- **Rayon**: Data parallelism with work stealing

## Architecture

```
ThreadPool
├── workers: Vec<Worker>
└── sender: mpsc::Sender<Job>

Worker
├── id: usize
└── thread: JoinHandle<()>

Job Queue (mpsc channel)
├── Sender (shared among all submission points)
└── Receiver (shared among workers via Arc<Mutex<...>>)
```

## Why Arc<Mutex<Receiver>>?

This is a common pattern that confuses beginners:

```rust
Arc<Mutex<mpsc::Receiver<Job>>>
```

**Why Arc?**
- Multiple workers need to share the receiver
- Arc = Atomic Reference Counting (thread-safe shared ownership)

**Why Mutex?**
- Only ONE worker can receive a job at a time
- `recv()` requires mutable access
- Mutex ensures synchronized access

**Why not Arc<Receiver>?**
- Receiver is not Clone or Sync by itself
- We need mutable access to call `recv()`

## Beginner Pitfalls & Borrow Checker Notes

### Pitfall 1: Moving Closure Environment
```rust
let receiver = Arc::new(Mutex::new(receiver));
for id in 0..size {
    let receiver = receiver.clone();  // ✅ Clone Arc before move
    thread::spawn(move || {
        // receiver is moved into this thread
    });
}
```
**Why**: Each thread needs its own Arc clone. The `move` closure takes ownership.

### Pitfall 2: Deadlock with Mutex
```rust
let lock = receiver.lock().unwrap();
// Don't hold lock across await points or long operations!
drop(lock);  // Explicitly drop to release lock
```

### Pitfall 3: Forgetting Shutdown
Without implementing `Drop`, threads will keep running after ThreadPool is dropped:
```rust
impl Drop for ThreadPool {
    fn drop(&mut self) {
        // Send shutdown signal to all workers
        // Join all threads to ensure clean shutdown
    }
}
```

## Code Walkthrough

See `src/main.rs` for a detailed implementation that demonstrates:
1. Creating a thread pool with N workers
2. Submitting jobs to the pool
3. Worker threads processing jobs from a shared queue
4. Graceful shutdown using the Drop trait
5. Handling panics in worker threads

## Performance Considerations

### Thread Count
- **Too few threads**: Underutilized CPU (waiting on I/O)
- **Too many threads**: Context switching overhead, memory waste
- **Good default**: `num_cpus` for CPU-bound, `2 * num_cpus` for I/O-bound
- **Production**: Often configurable (8-16 for web servers)

### Memory Usage
- Each thread: ~2MB stack + heap allocations
- 10 threads = ~20MB minimum
- Job queue: Unbounded grows memory if producers > consumers

### Comparison with Rayon
| Feature | Custom Pool | Rayon |
|---------|-------------|-------|
| Task type | Any closure | Fork-join, parallel iterators |
| Scheduling | FIFO queue | Work stealing |
| Use case | Server tasks, async | Data parallelism |
| Setup | Manual | Automatic |

## Comparison: Rust vs Go vs Python

| Feature | Rust ThreadPool | Go goroutines | Python ThreadPoolExecutor |
|---------|-----------------|---------------|--------------------------|
| Thread creation | Pre-spawned | Cheap (green threads) | Pre-spawned |
| Memory per thread | ~2MB | ~2KB | ~2MB |
| Type safety | Compile-time | Runtime panics | Runtime errors |
| Send constraint | Enforced by compiler | No compile-time check | GIL limits parallelism |
| Shutdown | Explicit (Drop) | No guarantee | context manager |

**Rust advantage**: Compiler enforces Send + 'static, preventing data races at compile time.

## Additional Challenges

1. **Bounded Queue**: Modify the thread pool to use a bounded queue that blocks when full.

2. **Worker Statistics**: Track jobs completed per worker, average job duration, queue length.

3. **Panic Recovery**: Handle worker panics gracefully and respawn failed workers.

4. **Priority Queue**: Implement job priorities so high-priority tasks run first.

5. **Thread Pool Scaling**: Dynamically add/remove workers based on queue length.

6. **Work Stealing**: Implement per-worker queues with work stealing (like Rayon).

## Real-World Improvements

Production thread pools add:
- **Timeouts**: Kill jobs that run too long
- **Metrics**: Prometheus/OpenTelemetry integration
- **Dynamic sizing**: Scale workers based on load
- **Named threads**: For debugging
- **Error handling**: Capture and report panics
- **Backpressure**: Reject tasks when overloaded

## Future Directions

- **Next**: Lock-free structures (Project 27)
- **Related**: Async/await (Project 20), channels (Project 18)
- **Advanced**: Implement work-stealing scheduler like Tokio

## Running This Project

```bash
cd 26-thread-pool
cargo run
```

## Expected Output

You should see:
- Thread pool starting with N workers
- Jobs being executed by different workers
- Completion messages from each job
- Graceful shutdown when pool is dropped
- Statistics showing job distribution across workers
