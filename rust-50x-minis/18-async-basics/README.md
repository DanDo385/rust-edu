# Project 18: Async Basics

## Overview
Learn asynchronous programming in Rust with async/await. Async code enables concurrent I/O operations without the overhead of threads. This project covers the Future trait, async/await syntax, tokio runtime, and the difference between threads and async tasks.

## Concepts Taught
- **async/await** syntax for asynchronous functions
- **Future trait** and how async works under the hood
- **tokio runtime** for executing async code
- **Concurrent I/O** without threads
- **.await** operator for waiting on futures
- **async blocks** and async closures
- **tokio::spawn** for concurrent tasks
- **Threads vs async tasks** (when to use each)

## Why Rust Behaves This Way

### The Problem with Threads for I/O
Threads are great for CPU-bound parallelism but wasteful for I/O:
- Each OS thread: ~2MB stack + OS overhead
- 10,000 threads = ~20GB memory just for stacks!
- I/O operations (network, disk) spend 99% of time waiting
- While waiting, thread does nothing but waste memory

### The Async Solution
Async enables many concurrent operations with few threads:
- Single thread can handle thousands of tasks
- Tasks are multiplexed on a thread pool
- When a task awaits I/O, another task runs
- Memory: ~few KB per task vs ~2MB per thread
- Perfect for I/O-bound work (servers, HTTP clients, etc.)

**Analogy:**
- **Threads**: One chef per customer (expensive!)
- **Async**: One chef juggling multiple orders (efficient!)

### How Async Works in Rust

Unlike JavaScript (single-threaded event loop) or Go (green threads), Rust's async is:
1. **Zero-cost**: No runtime overhead unless you use it
2. **Explicit**: You choose when to await
3. **Bring your own runtime**: tokio, async-std, etc.
4. **State machine**: async functions compile to state machines

```rust
async fn fetch() -> String {
    // Compiler transforms this into a state machine
    let data = fetch_data().await;  // Suspend point
    process(data).await             // Another suspend point
}
```

## Comparison: Rust vs JavaScript vs Go vs Python

| Feature | Rust | JavaScript | Go | Python |
|---------|------|------------|----|----|
| Runtime | Explicit (tokio) | Built-in (event loop) | Built-in (goroutines) | Built-in (asyncio) |
| Threading model | Multi-threaded async | Single-threaded | Green threads | Single-threaded |
| Syntax | async/await | async/await | goroutines/channels | async/await |
| Zero-cost | Yes | N/A | No (always present) | No (GIL overhead) |
| Safety | Compile-time | Runtime | Runtime | Runtime |
| Cancellation | Drop future | AbortController | Context cancellation | Task.cancel() |

## Beginner Pitfalls & Borrow Checker Notes

### Pitfall 1: Forgetting to Await
```rust
async fn fetch() -> String {
    "data".to_string()
}

fn main() {
    let data = fetch();  // ❌ This is a Future, not the data!
    println!("{}", data);  // ERROR: Future doesn't implement Display
}
```
**Fix**: Await the future:
```rust
#[tokio::main]
async fn main() {
    let data = fetch().await;  // ✅ Get the actual data
    println!("{}", data);
}
```

### Pitfall 2: Blocking in Async Functions
```rust
async fn bad_async() {
    thread::sleep(Duration::from_secs(1));  // ❌ Blocks entire thread!
}
```
**Fix**: Use async sleep:
```rust
async fn good_async() {
    tokio::time::sleep(Duration::from_secs(1)).await;  // ✅ Yields to other tasks
}
```

### Pitfall 3: Borrowing Across Await Points
```rust
async fn process(data: &str) {
    let reference = &data[0..2];
    some_async_fn().await;  // ❌ Might fail if reference lifetime is complex
    println!("{}", reference);
}
```
**Fix**: Clone data or restructure to avoid borrow across await:
```rust
async fn process(data: &str) {
    let owned = data[0..2].to_string();
    some_async_fn().await;  // ✅ Owned data, no borrow issues
    println!("{}", owned);
}
```

### Pitfall 4: Missing #[tokio::main]
```rust
async fn main() {  // ❌ ERROR: main function is not allowed to be async
    fetch_data().await;
}
```
**Fix**: Add tokio::main attribute:
```rust
#[tokio::main]
async fn main() {  // ✅ tokio::main creates runtime
    fetch_data().await;
}
```

## Code Walkthrough

See `src/main.rs` for detailed examples of:
1. Basic async functions and .await
2. Concurrent async tasks with tokio::spawn
3. Simulated async I/O operations
4. Comparing sync vs async performance
5. Async file operations
6. Future trait and how async works under the hood
7. Real-world patterns

## Performance Considerations

**When to Use Async:**
- ✅ I/O-bound work (network, disk, databases)
- ✅ High concurrency (thousands of connections)
- ✅ Web servers and HTTP clients
- ✅ Microservices and APIs

**When to Use Threads:**
- ✅ CPU-bound work (computation, encryption)
- ✅ Low concurrency (< 100 tasks)
- ✅ Blocking operations (FFI, legacy code)
- ✅ Simpler code (no async complexity)

**Async Overhead:**
- Task creation: ~few nanoseconds
- Memory: ~few KB per task
- Context switch: ~10ns (vs ~1μs for threads)
- Zero overhead if not used (unlike Go/Python)

**Tokio Runtime:**
- Multi-threaded work-stealing scheduler
- Number of threads = number of CPU cores by default
- Tasks are scheduled cooperatively (must .await to yield)

## Additional Challenges

1. **Concurrent HTTP Fetcher**: Make multiple HTTP requests in parallel and aggregate results.

2. **Async File Reader**: Read multiple files concurrently and process their contents.

3. **Rate Limiter**: Implement async rate limiting for API calls.

4. **Timeout Pattern**: Add timeouts to async operations using tokio::time::timeout.

5. **Connection Pool**: Build an async connection pool for database connections.

## Key Takeaways

1. **async/await** enables concurrent I/O without thread overhead
2. **Futures are lazy** - they don't run until awaited
3. **tokio** is the most popular async runtime for Rust
4. Use **tokio::spawn** for concurrent async tasks
5. Never block in async code (use async alternatives)
6. Async compiles to state machines (zero-cost abstraction)
7. Threads for CPU work, async for I/O work
8. One task can be moved between threads (Send requirement)
9. Async reduces memory usage for high-concurrency scenarios
10. Rust's async is explicit and composable

## Common Mistakes

❌ Forgetting to .await a future (it won't run!)
❌ Using thread::sleep instead of tokio::time::sleep
❌ Blocking in async code (kills concurrency)
❌ Not using #[tokio::main] on async main function
❌ Creating too many tokio::spawn tasks (has overhead)
❌ Using async for CPU-bound work (threads are better)
❌ Complex borrow across .await points
❌ Not handling errors from async functions
❌ Mixing runtimes (tokio and async-std together)
❌ Using .block_on inside async context (nested runtimes)

## Future Directions

- **Next**: Build a chat server with async (Project 19)
- **Advanced**: Web server with axum (Project 25)
- **Deep Dive**: Lock-free concurrent structures (Project 27)

## Running This Project

```bash
cd 18-async-basics
cargo run
```

Note: This project requires tokio. Add to Cargo.toml:
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

## Expected Output

You'll see:
- Async tasks executing concurrently
- Performance comparison: sync vs async
- Simulated I/O operations completing out of order
- Demonstration of task spawning and joining
- Concurrent operations with minimal memory usage
