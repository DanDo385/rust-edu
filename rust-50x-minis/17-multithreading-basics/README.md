# Project 17: Multithreading Basics

## Overview
Learn Rust's fearless concurrency with threads. Rust's type system prevents data races at compile time through the Send and Sync traits. This project covers thread spawning, joining, message passing, and shared state with Arc<Mutex<T>>.

## Concepts Taught
- **std::thread::spawn** for creating threads
- **JoinHandle** for waiting on thread completion
- **Send trait** for transferring ownership between threads
- **Sync trait** for sharing references between threads
- **Arc<Mutex<T>>** for shared mutable state
- **Message passing** with channels
- **Thread safety** guaranteed by the type system
- **Parallel computation** patterns

## Why Rust Behaves This Way

### Fearless Concurrency
Most languages make concurrency scary because of data races:
- **C/C++**: No protection, undefined behavior possible
- **Go**: Goroutines are easy but data races still possible
- **Java**: synchronized keyword, but easy to forget
- **Rust**: Data races are **impossible** at compile time!

Rust achieves this through:
1. **Ownership**: Only one owner can mutate data
2. **Send**: Types that can be transferred between threads
3. **Sync**: Types that can be referenced from multiple threads
4. **Type system**: Compiler enforces these rules

### Send and Sync Traits

**Send**: A type is Send if it can be transferred to another thread safely.
- Most types are Send (i32, String, Vec, etc.)
- Rc<T> is NOT Send (not thread-safe)
- Arc<T> IS Send (atomic reference counting)

**Sync**: A type is Sync if it can be referenced from multiple threads safely.
- Immutable types are Sync (&i32, &str)
- Mutex<T> is Sync (provides exclusive access)
- RefCell<T> is NOT Sync (runtime borrow checking isn't thread-safe)

The compiler automatically checks these traits! If your code compiles, it's thread-safe.

### Shared State vs Message Passing

Two approaches to concurrency:

**Message Passing** (channels):
- "Don't communicate by sharing memory; share memory by communicating"
- Channels transfer ownership of data
- No shared state = no data races
- Like Go's goroutines and channels

**Shared State** (Arc<Mutex<T>>):
- Multiple threads access the same data
- Mutex ensures exclusive access
- Arc provides shared ownership
- Like traditional threading in C/Java

Rust supports both! Use channels for most cases, shared state when needed.

## Beginner Pitfalls & Borrow Checker Notes

### Pitfall 1: Forgetting to Join Threads
```rust
thread::spawn(|| {
    println!("Hello from thread!");
});
// ❌ Program might exit before thread runs!
```
**Fix**: Call join() to wait for completion:
```rust
let handle = thread::spawn(|| {
    println!("Hello from thread!");
});
handle.join().unwrap();  // ✅ Wait for thread to finish
```

### Pitfall 2: Using Rc in Threads
```rust
let data = Rc::new(vec![1, 2, 3]);
thread::spawn(move || {  // ❌ ERROR: Rc is not Send
    println!("{:?}", data);
});
```
**Fix**: Use Arc instead:
```rust
let data = Arc::new(vec![1, 2, 3]);
thread::spawn(move || {  // ✅ Arc is Send
    println!("{:?}", data);
});
```

### Pitfall 3: Deadlocks with Multiple Locks
```rust
let data1 = Arc::new(Mutex::new(0));
let data2 = Arc::new(Mutex::new(0));

// Thread 1: locks data1 then data2
// Thread 2: locks data2 then data1
// ❌ DEADLOCK! Both waiting for each other
```
**Fix**: Always acquire locks in the same order, or use try_lock().

### Pitfall 4: Holding Locks Too Long
```rust
let data = Arc::new(Mutex::new(vec![1, 2, 3]));
let guard = data.lock().unwrap();
// ... lots of code ...
// ❌ Other threads blocked the entire time!
```
**Fix**: Lock for minimal time:
```rust
{
    let mut guard = data.lock().unwrap();
    guard.push(4);  // Just the critical section
}  // Lock released here
```

### Pitfall 5: Poisoned Mutexes
```rust
let data = Arc::new(Mutex::new(0));
thread::spawn(move || {
    let _guard = data.lock().unwrap();
    panic!("Oops!");  // ❌ Mutex is now poisoned!
});
```
**Fix**: Handle poison errors or use into_inner().

## Code Walkthrough

See `src/main.rs` for detailed examples of:
1. Basic thread spawning and joining
2. Thread ownership and the move keyword
3. Parallel computation with multiple threads
4. Sharing data with Arc<Mutex<T>>
5. Message passing with channels
6. Demonstrating Send and Sync traits
7. Common threading patterns

## Performance Considerations

**Thread Creation:**
- OS threads are expensive (~2MB stack per thread)
- Thread creation: ~100 microseconds
- Use thread pools for many short tasks
- Consider async/await for I/O-bound tasks (Project 18)

**Arc<Mutex<T>> Overhead:**
- Arc: Atomic increment/decrement on clone/drop
- Mutex: Lock acquisition (syscall on contention)
- Cache line bouncing with shared mutable state
- Minimize time holding locks

**CPU-Bound Work:**
- Use threads for CPU parallelism
- Number of threads ≈ number of CPU cores
- Use rayon for data parallelism (easier than manual threads)

**I/O-Bound Work:**
- Threads work but waste memory (each thread has stack)
- Async/await is more efficient (Project 18)

## Comparison: Rust vs Go vs Java

| Feature | Rust | Go | Java |
|---------|------|----|----|
| Thread creation | std::thread::spawn | go keyword | new Thread() |
| Compile-time safety | Yes (Send/Sync) | No | No |
| Data race prevention | Compile time | Runtime (race detector) | Runtime (FindBugs) |
| Shared state | Arc<Mutex<T>> | sync.Mutex | synchronized |
| Message passing | channels | channels | BlockingQueue |
| Default | No preference | Channels encouraged | Shared state common |
| Overhead | OS threads | Green threads (lightweight) | OS threads |

## Additional Challenges

1. **Parallel Sum**: Calculate sum of large array in parallel using multiple threads.

2. **Producer-Consumer**: Implement producer-consumer pattern with channels.

3. **Thread Pool**: Build a simple thread pool that executes tasks.

4. **Dining Philosophers**: Solve the classic synchronization problem.

5. **Parallel File Processing**: Read multiple files in parallel and aggregate results.

## Key Takeaways

1. **Rust prevents data races at compile time** with Send/Sync
2. **thread::spawn** creates OS threads, returns JoinHandle
3. **move** keyword transfers ownership to thread closure
4. **Arc<Mutex<T>>** enables shared mutable state across threads
5. **Channels** enable message passing between threads
6. **Send** means "safe to transfer between threads"
7. **Sync** means "safe to reference from multiple threads"
8. Lock for minimal time to avoid contention
9. Always join threads or use scoped threads
10. Rust's type system makes fearless concurrency possible

## Common Mistakes

❌ Forgetting to join threads (work might not complete)
❌ Using Rc instead of Arc (not Send)
❌ Using RefCell instead of Mutex (not Sync)
❌ Holding locks too long (performance killer)
❌ Creating too many threads (use thread pools)
❌ Not handling join errors (thread panics)
❌ Deadlocks from acquiring locks in different orders
❌ Using threads for I/O-bound work (use async instead)
❌ Sharing data without Arc (ownership issues)
❌ Not considering cache line contention with shared mutable state

## Future Directions

- **Next**: Learn async/await for concurrent I/O (Project 18)
- **Advanced**: Build a thread pool (Project 26)
- **Deep Dive**: Lock-free data structures (Project 27)

## Running This Project

```bash
cd 17-multithreading-basics
cargo run
```

## Expected Output

You'll see:
- Multiple threads executing in parallel
- Thread IDs and execution order (non-deterministic)
- Shared counter incremented by multiple threads
- Parallel computation results
- Message passing between threads
- Demonstrations of thread safety
