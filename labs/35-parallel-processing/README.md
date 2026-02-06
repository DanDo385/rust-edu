# Project 32: Parallel Processing with Rayon

## Overview
Learn data parallelism using the Rayon crate, which provides easy-to-use parallel iterators. This project demonstrates work-stealing, parallel image processing, and how Rayon automatically distributes work across CPU cores for massive performance gains with minimal code changes.

## Concepts Taught
- **Rayon crate**: data parallelism library
- **Parallel iterators**: `par_iter()` instead of `iter()`
- **Work-stealing**: dynamic load balancing across threads
- **Data parallelism**: same operation on different data
- **Thread pools**: Rayon manages threads automatically
- **Performance measurement**: comparing sequential vs parallel
- **Chunk-based processing**: dividing work efficiently
- **Scope-based parallelism**: `rayon::scope` for complex patterns

## Why Rayon Works

### Data Parallelism vs Task Parallelism
- **Data parallelism**: Apply same operation to different data elements (map, filter, reduce)
- **Task parallelism**: Different operations running concurrently (threads, async)
- **Rayon**: Specializes in data parallelism with ergonomic APIs

### Work-Stealing Algorithm
Rayon uses a sophisticated work-stealing scheduler:
1. Each thread has its own work queue (deque)
2. When a thread finishes, it "steals" work from other threads
3. This balances load dynamically - no manual tuning needed
4. Much better than simple thread pool or divide-and-conquer

**Example**: 100 tasks, 4 cores
- Traditional: 25 tasks per core (unbalanced if tasks have different durations)
- Work-stealing: Fast threads steal from slow threads (balanced automatically)

### Why It's Fast
- **No locks** on the hot path (lock-free work queues)
- **Cache-friendly**: threads work on nearby data
- **Minimal overhead**: adding `par_` often gives linear speedup
- **Compiler optimizations**: LLVM can vectorize parallel code

## Why Rust Behaves This Way

### Fearless Concurrency
Rayon guarantees data race freedom at **compile time**:
- Can't accidentally share mutable state
- Iterator methods enforce Send/Sync bounds
- Race conditions are impossible (in safe code)

**Comparison with other languages:**
- **Python**: GIL prevents true parallelism (use multiprocessing instead)
- **Go**: Easy goroutines but data races are possible (need mutexes)
- **C++**: Powerful but easy to create race conditions and deadlocks
- **Rust + Rayon**: Parallel by default, safe by design

### Send and Sync Traits
Rayon leverages Rust's type system:
- **Send**: Type can be transferred between threads
- **Sync**: Type can be shared between threads (with immutable references)
- Compiler automatically checks these at compile time
- If it compiles, it's data-race free!

## Beginner Pitfalls & Borrow Checker Notes

### Pitfall 1: Trying to Mutate Shared State
```rust
let mut sum = 0;
(0..100).into_par_iter().for_each(|x| {
    sum += x;  // ❌ ERROR: can't capture mutable reference
});
```
**Fix**: Use `reduce()` or `fold()`:
```rust
let sum = (0..100).into_par_iter().reduce(|| 0, |a, b| a + b);  // ✅ OK
```

### Pitfall 2: Sequential Bottlenecks
```rust
vec.par_iter()
    .map(expensive_computation)
    .collect::<Vec<_>>()  // Parallel
    .iter()               // ❌ Sequential again!
    .sum()
```
**Fix**: Keep the chain parallel:
```rust
vec.par_iter()
    .map(expensive_computation)
    .sum()  // ✅ Still parallel
```

### Pitfall 3: Too Fine-Grained Parallelism
```rust
(0..100).into_par_iter().map(|x| x + 1)  // ❌ Overhead > benefit
```
**Fix**: Parallelize coarse-grained work (milliseconds, not nanoseconds):
```rust
large_images.par_iter().map(|img| process_image(img))  // ✅ Good
```

### Pitfall 4: Forgetting to Add Rayon to Cargo.toml
```toml
[dependencies]
rayon = "1.8"
```

## Code Walkthrough

See `src/main.rs` for a detailed, commented implementation that demonstrates:
1. Basic parallel iteration with `par_iter()`
2. Parallel image processing (grayscale conversion)
3. Prime number calculation using work-stealing
4. Performance benchmarking (sequential vs parallel)
5. Parallel reduce/fold operations
6. Custom chunk sizes for optimization

## Performance Considerations

**When to Use Rayon:**
- Large datasets (10,000+ items)
- CPU-intensive operations (not I/O bound)
- Independent computations (no shared mutable state)
- Embarrassingly parallel problems

**Speedup Expectations:**
- 4 cores: 3-3.5x speedup (not perfect 4x due to overhead)
- 8 cores: 6-7x speedup
- 16 cores: 10-14x speedup
- Diminishing returns after ~8 cores for most workloads

**Overhead:**
- Thread creation: ~50-100μs (Rayon reuses threads)
- Work-stealing: ~100ns per steal
- Chunk coordination: ~10-50ns
- Worth it when work > ~1ms per item

**Memory:**
- Thread pool: ~8MB per thread (stack size)
- Work queues: ~1KB per thread
- Minimal compared to sequential version

## Comparison: Rust vs Go vs Python

| Feature | Rust + Rayon | Go | Python |
|---------|--------------|----|----|
| Parallel iteration | `par_iter()` | Manual goroutines + sync | `multiprocessing.Pool` |
| Safety | Compile-time race freedom | Runtime data races possible | GIL limits parallelism |
| Performance | Fastest, near-linear scaling | Fast, good concurrency | Slow, process overhead |
| Ease of use | Very easy (just add `par_`) | Moderate (need channels/mutexes) | Moderate (process serialization) |
| Memory overhead | Low | Low | High (process duplication) |

## Additional Challenges

1. **Parallel QuickSort**: Implement parallel quicksort using `rayon::join`.

2. **Mandelbrot Set**: Generate Mandelbrot fractal in parallel.

3. **Log File Analysis**: Parse millions of log lines in parallel.

4. **Matrix Multiplication**: Parallelize matrix multiply for large matrices.

5. **Web Crawler**: Parallel URL fetching (combine with async).

6. **Compare with Threads**: Implement the same task with manual threading and compare.

## Real-World Usage

Rayon is used in production by:
- **ripgrep**: Parallel grep that's faster than GNU grep
- **fd**: Parallel file finder
- **bat**: Syntax highlighting in parallel
- **Polars**: DataFrame library (like pandas but faster)
- **Spotify**: Audio processing pipelines
- **Image processing**: Encoding, resizing, filters
- **Scientific computing**: Simulations, machine learning

## Running This Project

```bash
cd 32-parallel-processing
cargo build --release  # Important! Debug builds are much slower
cargo run --release
```

**Note**: Add to `Cargo.toml`:
```toml
[dependencies]
rayon = "1.8"
image = "0.24"  # For image processing example
```

## Expected Output

You should see:
1. Prime calculation (sequential vs parallel) with timing
2. Image processing demonstration (if image file provided)
3. Parallel reduce/fold examples
4. Speedup comparison showing multi-core benefit
5. Work-stealing in action with unbalanced workloads
6. Chunk processing optimization
