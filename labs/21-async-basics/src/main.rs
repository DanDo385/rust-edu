// Project 18: Async Basics
//
// Asynchronous programming enables concurrent I/O operations without the memory
// overhead of threads. Rust's async/await is a zero-cost abstraction that compiles
// to efficient state machines. This project demonstrates async fundamentals.
//
// NOTE: This code requires the tokio runtime. Add to Cargo.toml:
// [dependencies]
// tokio = { version = "1", features = ["full"] }

use tokio::time::{sleep, Duration};
use std::time::Instant;

// The #[tokio::main] macro transforms async main into a regular main
// that creates a tokio runtime and blocks on the async function.
#[tokio::main]
async fn main() {
    println!("=== Rust Async Basics ===\n");

    demonstrate_basic_async().await;
    demonstrate_concurrent_tasks().await;
    demonstrate_sync_vs_async().await;
    demonstrate_async_spawning().await;
    demonstrate_future_trait().await;
}

// ============================================================================
// BASIC ASYNC FUNCTIONS
// ============================================================================
// An async function returns a Future. The Future doesn't execute until
// you .await it. This is called "lazy evaluation".

async fn demonstrate_basic_async() {
    println!("--- Basic Async Functions ---");

    // Simple async function
    async fn fetch_data() -> String {
        // Simulate async I/O with a sleep
        sleep(Duration::from_millis(100)).await;
        "Data from async function".to_string()
    }

    // Calling fetch_data() creates a Future but doesn't execute it yet
    let future = fetch_data();
    println!("Created future (not executed yet)");

    // .await executes the future and waits for completion
    let result = future.await;
    println!("Result: {}", result);

    // WHY ASYNC?
    // Traditional blocking I/O wastes CPU while waiting.
    // Async allows other tasks to run while this one waits.
    // One thread can handle thousands of async tasks!

    println!();
}

// ============================================================================
// CONCURRENT TASKS WITH TOKIO::SPAWN
// ============================================================================
// tokio::spawn runs a task concurrently. Multiple tasks can run at the
// same time, even on a single thread (cooperative multitasking).

async fn demonstrate_concurrent_tasks() {
    println!("--- Concurrent Async Tasks ---");

    // Simulate multiple I/O operations
    async fn fetch_from_api(id: u32, delay_ms: u64) -> String {
        println!("Task {}: Starting request...", id);
        sleep(Duration::from_millis(delay_ms)).await;
        println!("Task {}: Request complete!", id);
        format!("Data from API {}", id)
    }

    let start = Instant::now();

    // Launch three tasks concurrently
    let task1 = fetch_from_api(1, 200);
    let task2 = fetch_from_api(2, 100);
    let task3 = fetch_from_api(3, 150);

    // Wait for all tasks to complete (runs concurrently!)
    let (result1, result2, result3) = tokio::join!(task1, task2, task3);

    let duration = start.elapsed();

    println!("Results:");
    println!("  {}", result1);
    println!("  {}", result2);
    println!("  {}", result3);
    println!("Total time: {:?}", duration);
    println!("(Notice: All tasks ran concurrently, not sequentially!)");

    // KEY INSIGHT:
    // If these ran sequentially: 200 + 100 + 150 = 450ms
    // With async concurrency: ~200ms (limited by slowest task)
    // All on a SINGLE thread!

    println!();
}

// ============================================================================
// SYNC VS ASYNC PERFORMANCE COMPARISON
// ============================================================================
// Let's compare synchronous blocking I/O with asynchronous I/O.

async fn demonstrate_sync_vs_async() {
    println!("--- Sync vs Async Performance ---");

    // Synchronous version (simulated with thread::sleep)
    fn sync_fetch(id: u32) -> String {
        std::thread::sleep(Duration::from_millis(100));
        format!("Sync data {}", id)
    }

    // Asynchronous version
    async fn async_fetch(id: u32) -> String {
        sleep(Duration::from_millis(100)).await;
        format!("Async data {}", id)
    }

    // Synchronous approach: One after another
    println!("Synchronous (sequential):");
    let start = Instant::now();
    for i in 0..5 {
        let _data = sync_fetch(i);
    }
    let sync_duration = start.elapsed();
    println!("  Time: {:?}", sync_duration);

    // Asynchronous approach: All concurrent
    println!("Asynchronous (concurrent):");
    let start = Instant::now();
    let futures: Vec<_> = (0..5).map(|i| async_fetch(i)).collect();
    let _results = futures::future::join_all(futures).await;
    let async_duration = start.elapsed();
    println!("  Time: {:?}", async_duration);

    println!("Speedup: {:.2}x faster!",
             sync_duration.as_secs_f64() / async_duration.as_secs_f64());

    // WHY THE DIFFERENCE?
    // - Sync: Each operation blocks the thread (5 * 100ms = 500ms)
    // - Async: All operations run concurrently (~100ms total)
    // - Same thread, but async yields control during waits

    println!();
}

// ============================================================================
// SPAWNING ASYNC TASKS
// ============================================================================
// tokio::spawn creates an independent task that runs on the tokio runtime.
// It returns a JoinHandle (like thread::spawn).

async fn demonstrate_async_spawning() {
    println!("--- Spawning Async Tasks ---");

    // Spawn multiple independent tasks
    let mut handles = vec![];

    for i in 0..5 {
        // tokio::spawn requires the future to be 'static
        // This means it can't borrow from the current scope
        let handle = tokio::spawn(async move {
            sleep(Duration::from_millis(100 - i * 10)).await;
            println!("Task {} completed", i);
            i * 2  // Return value
        });

        handles.push(handle);
    }

    // Wait for all spawned tasks and collect results
    println!("Waiting for all tasks...");
    let mut results = vec![];
    for handle in handles {
        let result = handle.await.unwrap();
        results.push(result);
    }

    println!("All tasks completed!");
    println!("Results: {:?}", results);

    // TOKIO::SPAWN vs TOKIO::JOIN:
    // - spawn: Creates independent task, might run on different thread
    // - join!: Waits for multiple futures, guaranteed to run on current task
    //
    // Use spawn when:
    // - Tasks are independent
    // - You want true parallelism (runtime can use multiple threads)
    //
    // Use join! when:
    // - Tasks are related
    // - You want to keep them on the same thread
    // - Simpler than managing JoinHandles

    println!();
}

// ============================================================================
// UNDERSTANDING THE FUTURE TRAIT
// ============================================================================
// Under the hood, async functions return types that implement Future.
// Let's peek at how this works.

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

async fn demonstrate_future_trait() {
    println!("--- Future Trait Under the Hood ---");

    // Every async function returns something that implements Future
    async fn simple_async() -> i32 {
        42
    }

    let future = simple_async();
    let result = future.await;
    println!("Async function returned: {}", result);

    // You can also create Futures manually (advanced)
    struct SimpleFuture {
        completed: bool,
    }

    impl Future for SimpleFuture {
        type Output = String;

        fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
            if self.completed {
                Poll::Ready("Future completed!".to_string())
            } else {
                self.completed = true;
                Poll::Pending  // Not ready yet, poll again later
            }
        }
    }

    // This is roughly what the compiler generates for async functions!
    // async/await is syntactic sugar over this state machine.

    println!("Created custom Future");

    // HOW FUTURES WORK:
    // 1. Future is polled (checked if ready)
    // 2. If ready: Returns Poll::Ready(value)
    // 3. If not ready: Returns Poll::Pending, runtime will poll again later
    // 4. Futures are state machines - they track where they are in execution
    //
    // When you .await:
    // 1. Current task is suspended
    // 2. Runtime saves the state (which .await point we're at)
    // 3. Runtime runs other tasks
    // 4. When I/O completes, runtime wakes the task
    // 5. Task resumes from the .await point

    println!();
}

// ============================================================================
// COMMON ASYNC PATTERNS
// ============================================================================

// Pattern 1: Select (first to complete)
async fn _select_pattern() {
    async fn task1() -> &'static str {
        sleep(Duration::from_millis(100)).await;
        "Task 1"
    }

    async fn task2() -> &'static str {
        sleep(Duration::from_millis(50)).await;
        "Task 2"
    }

    // tokio::select! runs multiple futures and returns first to complete
    tokio::select! {
        result = task1() => println!("Task 1 finished first: {}", result),
        result = task2() => println!("Task 2 finished first: {}", result),
    }
}

// Pattern 2: Timeout
async fn _timeout_pattern() {
    async fn slow_operation() -> String {
        sleep(Duration::from_secs(10)).await;
        "Done!".to_string()
    }

    // Fail if operation takes too long
    let result = tokio::time::timeout(
        Duration::from_secs(1),
        slow_operation()
    ).await;

    match result {
        Ok(value) => println!("Completed: {}", value),
        Err(_) => println!("Timed out!"),
    }
}

// Pattern 3: Retry with exponential backoff
async fn _retry_pattern() {
    async fn unreliable_operation() -> Result<String, String> {
        // Simulated failure
        Err("Network error".to_string())
    }

    for attempt in 0..3 {
        match unreliable_operation().await {
            Ok(result) => {
                println!("Success: {}", result);
                break;
            }
            Err(e) => {
                println!("Attempt {} failed: {}", attempt + 1, e);
                sleep(Duration::from_millis(100 * 2_u64.pow(attempt))).await;
            }
        }
    }
}

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. async functions return Futures, which are lazy (don't run until awaited)
// 2. .await suspends the current task and allows others to run
// 3. tokio::join! runs multiple futures concurrently on same task
// 4. tokio::spawn creates independent tasks (can run on different threads)
// 5. Async is perfect for I/O-bound work (network, disk, databases)
// 6. Never use blocking operations in async code (use async alternatives)
// 7. Futures compile to state machines (zero-cost abstraction)
// 8. One thread can run thousands of async tasks (cooperative multitasking)
// 9. #[tokio::main] creates runtime and converts async main to sync
// 10. Rust's async is explicit, composable, and zero-cost

// ============================================================================
// COMMON MISTAKES
// ============================================================================
// ❌ Forgetting to .await (future won't execute!)
// ❌ Using thread::sleep instead of tokio::time::sleep (blocks the thread!)
// ❌ Calling .block_on inside async context (nested runtimes)
// ❌ Not using #[tokio::main] on async main
// ❌ Complex borrows across .await points (lifetime issues)
// ❌ Using async for CPU-bound work (threads are better)
// ❌ Creating too many tokio::spawn tasks (has overhead)
// ❌ Mixing async runtimes (tokio + async-std)
// ❌ Not handling errors from async functions
// ❌ Expecting async to magically make CPU work faster (it doesn't!)

// ============================================================================
// THREADS VS ASYNC: WHEN TO USE WHICH
// ============================================================================
//
// USE ASYNC when:
// ✅ I/O-bound work (network, file system, databases)
// ✅ High concurrency (thousands of connections)
// ✅ Web servers, HTTP clients, microservices
// ✅ You need to minimize memory usage
// ✅ Operations spend most time waiting
//
// USE THREADS when:
// ✅ CPU-bound work (computation, encryption, data processing)
// ✅ Low concurrency (< 100 parallel tasks)
// ✅ Blocking operations (FFI calls, legacy libraries)
// ✅ Simpler code (async has learning curve)
// ✅ You need true parallelism for CPU work
//
// MEMORY COMPARISON:
// - Thread: ~2MB per thread (stack + OS overhead)
// - Async task: ~few KB per task
// - 10,000 threads = ~20GB
// - 10,000 async tasks = ~tens of MB

// ============================================================================
// PERFORMANCE NOTES
// ============================================================================
// 1. Task creation: ~nanoseconds (vs ~100μs for threads)
// 2. Context switch: ~10ns (vs ~1μs for thread context switch)
// 3. Memory per task: ~few KB (vs ~2MB per thread)
// 4. Async is NOT faster for CPU work (use threads for that)
// 5. Async shines when you have many concurrent I/O operations
// 6. Tokio uses work-stealing scheduler (efficient load balancing)
// 7. Number of runtime threads = CPU cores by default
// 8. Async has zero cost if not used (unlike Go's runtime)
