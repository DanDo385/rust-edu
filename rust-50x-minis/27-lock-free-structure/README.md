# Project 27: Lock-Free Structure

## Overview
This project implements a lock-free stack using atomic operations and the Compare-And-Swap (CAS) pattern. You'll learn low-level concurrent programming, memory ordering, and when lock-free structures are worth the complexity. This is the foundation of high-performance concurrent systems.

## Concepts Taught
- **Lock-free programming** without mutexes
- **Atomic operations** (load, store, compare_and_swap)
- **Memory ordering** (Relaxed, Acquire, Release, SeqCst)
- **ABA problem** in lock-free structures
- **Compare-And-Swap (CAS)** loops
- **unsafe code** for raw pointers
- **Pointer tagging** techniques
- **Linearizability** and concurrent correctness

## Why Lock-Free?

### Problems with Locks (Mutex)
- **Blocking**: One thread waits for another
- **Priority inversion**: Low-priority thread holds lock, blocking high-priority
- **Deadlock**: Circular lock dependencies
- **Convoy effect**: Many threads queued behind one lock
- **No progress guarantee**: A thread can hold lock forever

### Lock-Free Advantages
- **Non-blocking**: Threads never wait for locks
- **Progress guarantee**: System makes progress even if one thread stalls
- **Better worst-case latency**: No lock contention spikes
- **Composability**: Easier to combine operations

### Lock-Free Disadvantages
- **Complexity**: Much harder to implement correctly
- **ABA problem**: Values can change and change back
- **Memory reclamation**: Can't free memory safely (need hazard pointers or epoch-based)
- **Often slower**: CAS loops can spin, more cache coherency traffic
- **Not always worth it**: Locks are fine for most use cases!

**When to use lock-free:**
- High contention scenarios (many threads)
- Real-time systems (predictable latency)
- Wait-free progress required
- Building blocks for lock-free algorithms (queue, stack)

**Real-world usage:**
- **crossbeam**: Lock-free queues and epoch-based memory reclamation
- **parking_lot**: Fast mutex using atomic operations
- **Linux kernel**: Lock-free ring buffers, per-CPU caches
- **Tokio**: Lock-free task queue

## Memory Ordering Explained

Memory ordering controls how atomic operations are synchronized:

```rust
Ordering::Relaxed   // No synchronization (just atomicity)
Ordering::Acquire   // Loads synchronized (see all previous stores)
Ordering::Release   // Stores synchronized (others see this store)
Ordering::AcqRel    // Both Acquire and Release
Ordering::SeqCst    // Sequentially consistent (total order)
```

**Simple rule for beginners**: Use `SeqCst` (strongest, slowest). Optimize later with profiling.

**Advanced optimization**: Use Release/Acquire pairs for publish/subscribe patterns.

## The ABA Problem

Lock-free algorithms suffer from the "ABA problem":

```
Thread 1                Thread 2
Read: head = A
                        Pop A (head = B)
                        Pop B (head = C)
                        Push A (head = A again)
CAS(A, new)
  Success! But B is lost!
```

**Solutions:**
1. **Tagged pointers**: Increment counter with each CAS
2. **Hazard pointers**: Track which pointers are in use
3. **Epoch-based reclamation** (crossbeam-epoch)

Our implementation uses tagged pointers (simplest).

## Beginner Pitfalls & Unsafe Code Notes

### Pitfall 1: Incorrect Memory Ordering
```rust
head.load(Ordering::Relaxed)  // ❌ May see stale value!
head.load(Ordering::Acquire)  // ✅ Synchronizes with Release store
```

### Pitfall 2: Memory Leaks
Lock-free structures can't free nodes safely:
```rust
// ❌ WRONG - another thread might still be reading this!
Box::from_raw(old_head);

// ✅ Use epoch-based reclamation (crossbeam-epoch)
// Or leak memory (acceptable for long-lived structures)
```

### Pitfall 3: Dereferencing Invalid Pointers
```rust
unsafe {
    (*old_head).next  // ❌ old_head might be freed by another thread!
}
// Need hazard pointers or careful reasoning
```

### Unsafe Code Justification
This project uses `unsafe` for:
1. **Raw pointer manipulation**: Required for intrusive linked list
2. **Atomic pointer operations**: AtomicPtr requires raw pointers
3. **Box into_raw/from_raw**: Manual memory management

**Safety reasoning**: We use tagged pointers to avoid ABA, but still leak memory on pop. A production implementation would use crossbeam-epoch.

## Code Walkthrough

See `src/main.rs` for a detailed implementation that demonstrates:
1. Lock-free stack using CAS loops
2. Tagged pointers to mitigate ABA problem
3. Different memory ordering semantics
4. Stress testing with concurrent threads
5. Comparison with Mutex-based implementation
6. Careful unsafe code with comments

## Performance Considerations

### When Lock-Free Wins
- **High contention**: 10+ threads accessing same structure
- **Short critical sections**: CAS is cheaper than mutex lock/unlock
- **Real-time**: Predictable worst-case latency

### When Mutex Wins
- **Low contention**: Mutex is faster (1-2 threads)
- **Long critical sections**: CAS loops waste CPU spinning
- **Complex operations**: Easier to reason about with locks

### Benchmark Results (typical)
```
1 thread:  Mutex faster (no contention)
2 threads: Roughly equal
4 threads: Lock-free 20-30% faster
8 threads: Lock-free 50-70% faster
16 threads: Lock-free 2-3x faster
```

**Important**: Always benchmark YOUR workload. Lock-free is not always faster!

### Memory Ordering Performance
```
Relaxed: Fastest (no synchronization)
Acquire/Release: Moderate (synchronization barriers)
SeqCst: Slowest (total ordering, memory fence)
```

On x86: SeqCst is nearly free (strong memory model)
On ARM: SeqCst is expensive (weak memory model, needs barriers)

## Comparison: Rust vs C++ vs Go

| Feature | Rust | C++ | Go |
|---------|------|-----|-----|
| Atomic types | std::sync::atomic | std::atomic | sync/atomic |
| Memory ordering | Explicit (Ordering enum) | Explicit (memory_order) | Implicit (happens-before) |
| Safety | unsafe required for raw pointers | Unchecked everywhere | No low-level atomics |
| CAS operation | compare_and_swap | compare_exchange_weak | CompareAndSwapPointer |
| Type safety | Compile-time checks | Minimal checks | Runtime checks |

**Rust advantage**: unsafe is explicit and localized. Easier to audit.

## Additional Challenges

1. **Lock-Free Queue**: Implement MPSC queue (Michael-Scott algorithm)

2. **Lock-Free Hash Table**: Use atomic arrays and CAS for insert/lookup

3. **Compare Performance**: Benchmark against Mutex, RwLock, crossbeam

4. **Memory Reclamation**: Integrate crossbeam-epoch for safe memory reclamation

5. **Elimination Array**: Add elimination backoff to reduce contention

6. **Wait-Free Stack**: Implement wait-free variant (harder!)

## Real-World Lock-Free Libraries

### crossbeam
The gold standard for lock-free structures in Rust:
- Lock-free queues (MPMC, MPSC)
- Epoch-based memory reclamation
- Deque with work stealing
- Production-ready and well-tested

### parking_lot
Fast synchronization primitives:
- Mutex faster than std::sync::Mutex
- RwLock with no reader starvation
- Uses atomic operations internally

### concurrent-queue
Bounded and unbounded lock-free queues:
- Based on crossbeam
- Optimized for different workloads

## Future Directions

- **Next**: Key-value store (Project 28)
- **Related**: Thread pool (Project 26), async/await (Project 20)
- **Advanced**: Study crossbeam-epoch internals

## Running This Project

```bash
cd 27-lock-free-structure
cargo run
cargo run --release  # Much faster for benchmarks
```

## Expected Output

You should see:
- Lock-free stack operations (push/pop)
- Concurrent stress test with multiple threads
- Performance comparison: lock-free vs Mutex
- Demonstration of different memory orderings
- Verification that the stack works correctly under load
