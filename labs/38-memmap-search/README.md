# Project 35: Memory-Mapped File Processing

## Overview
Implement high-performance file processing using memory-mapped I/O (mmap). This project demonstrates zero-copy file access, parallel scanning, and performance optimization techniques for processing large files efficiently.

## Concepts Taught
- **Memory-mapped files**: using `memmap2` crate
- **Zero-copy I/O**: accessing file data without copying to userspace
- **Virtual memory**: how OS maps files into process address space
- **Page faults**: lazy loading of file contents
- **Parallel file scanning**: splitting mmap across threads
- **Performance optimization**: comparing mmap vs traditional I/O
- **Unsafe Rust**: handling unsafe memory operations safely
- **SIMD potential**: vectorized operations on mapped memory

## Why Memory Mapping Works

### Traditional File I/O
1. Application calls `read()`
2. OS copies data from disk → kernel buffer
3. OS copies data from kernel buffer → application buffer
4. **Two copies!** Slow for large files

### Memory-Mapped I/O
1. Application calls `mmap()`
2. OS maps file into process virtual address space
3. Access file data like memory (no explicit read/write)
4. **Zero copies!** OS handles paging automatically

### When Pages Are Loaded
Memory mapping is **lazy**:
- `mmap()` returns immediately (doesn't load file)
- First access to page causes page fault
- OS loads page from disk into RAM
- Subsequent accesses are fast (already in RAM)

### Why It's Fast
- **No copying**: file data accessed directly
- **OS manages caching**: uses all available RAM efficiently
- **Shared mappings**: multiple processes can share same pages
- **Parallel access**: different threads can access different regions
- **Sequential prefetch**: OS predicts and preloads pages

## Why Rust Behaves This Way

### Safety Concerns with mmap
Memory-mapped files are **inherently unsafe**:
- File can be modified externally (race conditions)
- File can be truncated (access beyond EOF)
- Multiple processes might write concurrently
- Undefined behavior if file changes during access

**Rust's approach**: Use `unsafe` but encapsulate safely:
```rust
let mmap = unsafe { MmapOptions::new().map(&file)? };  // Unsafe creation
// But using it as &[u8] is safe if we control the file
```

**Comparison with other languages:**
- **C**: `mmap()` - completely manual, easy to corrupt memory
- **Python**: `mmap` module - safe but slower (Python overhead)
- **Go**: `syscall.Mmap()` - manual, some safety via slices
- **Rust**: `memmap2` - unsafe creation, but typed access

### Zero-Cost Abstraction
The `memmap2` crate wraps `mmap()` syscall with zero overhead:
- Same performance as C
- Type safety where possible
- Clear unsafe boundaries

## Beginner Pitfalls & Borrow Checker Notes

### Pitfall 1: File Modification During Access
```rust
let mmap = unsafe { MmapOptions::new().map(&file)? };
// If file is modified externally: UNDEFINED BEHAVIOR!
```
**Fix**: Ensure exclusive access or use read-only mapping.

### Pitfall 2: Accessing Beyond EOF
```rust
let mmap = unsafe { MmapOptions::new().map(&file)? };
file.set_len(0)?;  // Truncate file
let _ = mmap[0];   // ❌ SEGFAULT or undefined behavior
```
**Fix**: Don't truncate file while mapped.

### Pitfall 3: Not Checking File Size
```rust
let mmap = unsafe { MmapOptions::new().map(&file)? };
let value = mmap[1_000_000_000];  // ❌ Out of bounds if file is smaller
```
**Fix**: Check `mmap.len()` before accessing.

### Pitfall 4: Holding Mmap Too Long
```rust
let mmap = unsafe { MmapOptions::new().map(&file)? };
// File descriptor stays open until mmap is dropped
// Can hit open file limit!
```
**Fix**: Drop mmap when done, or use scoped access.

## Code Walkthrough

See `src/main.rs` for a detailed, commented implementation that demonstrates:
1. Creating and reading memory-mapped files
2. Searching large files with zero-copy access
3. Parallel scanning across multiple threads
4. Performance comparison: mmap vs read()
5. Safe encapsulation of unsafe operations
6. Creating test files for benchmarking
7. Byte pattern matching in mapped memory

## Performance Considerations

**When mmap is FASTER:**
- Large files (> 1MB)
- Random access patterns
- Multiple passes over data
- Parallel processing
- Memory available for caching

**When mmap is SLOWER:**
- Small files (< 4KB) - syscall overhead
- Sequential one-time reads - buffered I/O is optimized
- Limited RAM - thrashing due to paging
- Network filesystems - latency amplified

**Benchmark Results** (typical, 100MB file):
- `read()` sequential: ~300 MB/s
- `mmap` first pass: ~500 MB/s (includes page faults)
- `mmap` second pass: ~2000 MB/s (data in page cache)
- `mmap` parallel: ~5000 MB/s (4 cores, data cached)

**Memory Usage:**
- `read()`: File size in RAM (your buffer)
- `mmap`: OS manages, uses available RAM, can exceed file size
- Virtual memory: File appears in process, but not fully in RAM

## Comparison: Rust vs Go vs Python

| Feature | Rust (memmap2) | Go (syscall.Mmap) | Python (mmap) |
|---------|----------------|-------------------|---------------|
| Safety | Unsafe creation, safe use | Manual, some safety | Safe but slow |
| Performance | Fastest (zero overhead) | Fast | Slower (interpreter) |
| Ease of use | Moderate (unsafe block) | Moderate | Easy |
| Parallel access | Excellent (Send/Sync) | Good (goroutines) | Poor (GIL) |
| Cross-platform | Yes (Windows/Unix) | Partial | Yes |

## Additional Challenges

1. **Line Counter**: Count lines in a large text file using mmap.

2. **Pattern Search**: Implement Boyer-Moore or similar algorithm on mmap.

3. **Log File Analyzer**: Parse and analyze large log files in parallel.

4. **File Deduplication**: Find duplicate files using mmap for fast comparison.

5. **Binary Parser**: Parse binary file formats (images, videos) with zero-copy.

6. **Database Page Cache**: Implement simple database buffer pool with mmap.

7. **Compression**: Implement LZ4 or similar on memory-mapped data.

8. **Compare with io_uring**: Benchmark against modern async I/O.

## Real-World Usage

Memory-mapped files are used in:
- **Databases**: SQLite, LMDB, RocksDB use mmap for data files
- **Search engines**: Lucene, Elasticsearch for index files
- **Log processing**: ripgrep, ag for fast text search
- **Image processing**: Loading large images without full copy
- **Game engines**: Loading assets (textures, models)
- **Scientific computing**: Processing large datasets
- **Compilers**: Reading source files (LLVM uses mmap)
- **Operating systems**: Executable loading (demand paging)

## Running This Project

```bash
cd 35-memmap-search
cargo run --release  # Always use --release for accurate benchmarks
```

**Note**: Add to `Cargo.toml`:
```toml
[dependencies]
memmap2 = "0.9"
```

## Expected Output

You should see:
1. Test file generation (creating large file for testing)
2. Sequential mmap search with timing
3. Parallel mmap search with timing (faster)
4. Performance comparison: mmap vs read()
5. Memory usage statistics
6. Pattern matching results
7. Speedup from parallelization
8. Cache effects (second run faster than first)
