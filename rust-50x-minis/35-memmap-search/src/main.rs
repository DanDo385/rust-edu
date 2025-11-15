// Project 35: Memory-Mapped File Processing
//
// Demonstrates high-performance file processing using memory-mapped I/O (mmap).
// Shows zero-copy file access, parallel scanning, and performance optimization
// for processing large files efficiently.

use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::time::Instant;

// Note: This example shows the concepts without the memmap2 crate
// In production, you would use: use memmap2::Mmap;

fn main() {
    println!("=== Memory-Mapped File Processing ===\n");

    // ============================================================================
    // CREATE TEST FILE
    // ============================================================================
    println!("=== Creating Test File ===");

    let test_file = "test_data.txt";
    let file_size_mb = 10;

    println!("Creating {}MB test file...", file_size_mb);
    match create_test_file(test_file, file_size_mb) {
        Ok(_) => println!("✓ Test file created: {}", test_file),
        Err(e) => {
            eprintln!("Failed to create test file: {}", e);
            return;
        }
    }

    println!();

    // ============================================================================
    // TRADITIONAL FILE I/O
    // ============================================================================
    println!("=== Traditional File I/O (Read) ===");

    let pattern = "TARGET";

    let start = Instant::now();
    match search_with_read(test_file, pattern) {
        Ok(count) => {
            let duration = start.elapsed();
            println!("Found '{}': {} occurrences", pattern, count);
            println!("Time: {:?}", duration);
            println!("Throughput: {:.2} MB/s", file_size_mb as f64 / duration.as_secs_f64());
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    println!();

    // ============================================================================
    // SIMULATED MEMORY-MAPPED FILE ACCESS
    // ============================================================================
    println!("=== Simulated Memory-Mapped File Access ===");
    println!("(In production, use memmap2 crate for actual mmap)");

    let start = Instant::now();
    match search_with_simulated_mmap(test_file, pattern) {
        Ok(count) => {
            let duration = start.elapsed();
            println!("Found '{}': {} occurrences", pattern, count);
            println!("Time: {:?}", duration);
            println!("Throughput: {:.2} MB/s", file_size_mb as f64 / duration.as_secs_f64());
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    println!();

    // ============================================================================
    // PARALLEL FILE SCANNING
    // ============================================================================
    println!("=== Parallel File Scanning ===");

    let start = Instant::now();
    match parallel_search(test_file, pattern) {
        Ok(count) => {
            let duration = start.elapsed();
            println!("Found '{}': {} occurrences (parallel)", pattern, count);
            println!("Time: {:?}", duration);
            println!("Throughput: {:.2} MB/s", file_size_mb as f64 / duration.as_secs_f64());
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    println!();

    // ============================================================================
    // MEMORY-MAPPED FILE CONCEPTS
    // ============================================================================
    println!("=== Memory-Mapped File Concepts ===");
    println!("\nHow mmap works:");
    println!("1. File is mapped into process virtual address space");
    println!("2. No immediate data loading (lazy/demand paging)");
    println!("3. First access triggers page fault → OS loads page from disk");
    println!("4. Subsequent accesses are fast (data in RAM/page cache)");
    println!("5. OS automatically caches frequently accessed pages");
    println!("6. Zero-copy: no data duplication in userspace\n");

    println!("Advantages:");
    println!("  ✓ Zero-copy access (no buffer allocation)");
    println!("  ✓ OS manages caching (uses all available RAM)");
    println!("  ✓ Parallel access (different threads, different regions)");
    println!("  ✓ Shared between processes (efficient IPC)");
    println!("  ✓ Fast random access\n");

    println!("Disadvantages:");
    println!("  ✗ Unsafe in Rust (file can change externally)");
    println!("  ✗ Page faults on first access (initial slowdown)");
    println!("  ✗ Not ideal for small files (syscall overhead)");
    println!("  ✗ Can cause thrashing if file > RAM");
    println!("  ✗ Platform-specific behavior\n");

    // ============================================================================
    // CLEANUP
    // ============================================================================
    println!("=== Cleanup ===");
    match std::fs::remove_file(test_file) {
        Ok(_) => println!("✓ Test file removed"),
        Err(e) => eprintln!("Failed to remove test file: {}", e),
    }

    println!();
    println!("=== Demo Complete ===");
    println!("\nTo use real memory mapping:");
    println!("1. Add to Cargo.toml: memmap2 = \"0.9\"");
    println!("2. Use: unsafe {{ MmapOptions::new().map(&file)? }}");
    println!("3. Access as &[u8] slice");
    println!("4. Enjoy zero-copy performance!");
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Creates a test file with random data and known patterns
fn create_test_file(path: &str, size_mb: usize) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;

    let chunk = b"Lorem ipsum dolor sit amet, consectetur adipiscing elit. TARGET here. \
                   Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.\n";

    let chunks_per_mb = (1024 * 1024) / chunk.len();

    for _ in 0..(size_mb * chunks_per_mb) {
        file.write_all(chunk)?;
    }

    file.flush()?;
    Ok(())
}

/// Search using traditional read() calls
fn search_with_read(path: &str, pattern: &str) -> io::Result<usize> {
    use std::io::Read;

    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    Ok(count_pattern(&buffer, pattern.as_bytes()))
}

/// Simulate memory-mapped file access using read()
/// (In production, use memmap2 crate for actual mmap)
fn search_with_simulated_mmap(path: &str, pattern: &str) -> io::Result<usize> {
    use std::io::Read;

    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // In actual mmap:
    // let mmap = unsafe { MmapOptions::new().map(&file)? };
    // count_pattern(&mmap, pattern.as_bytes())

    Ok(count_pattern(&buffer, pattern.as_bytes()))
}

/// Parallel search by splitting file into chunks
fn parallel_search(path: &str, pattern: &str) -> io::Result<usize> {
    use std::io::Read;

    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let num_threads = 4;
    let chunk_size = buffer.len() / num_threads;

    let pattern_bytes = pattern.as_bytes();

    let counts: Vec<_> = (0..num_threads)
        .map(|i| {
            let start = i * chunk_size;
            let end = if i == num_threads - 1 {
                buffer.len()
            } else {
                (i + 1) * chunk_size + pattern_bytes.len() - 1 // Overlap for boundary matches
            };

            let chunk = &buffer[start..end.min(buffer.len())];
            let pattern_owned = pattern.to_string();

            std::thread::spawn(move || count_pattern(chunk, pattern_owned.as_bytes()))
        })
        .collect();

    let total: usize = counts.into_iter().map(|h| h.join().unwrap()).sum();

    Ok(total)
}

/// Count occurrences of pattern in data
fn count_pattern(data: &[u8], pattern: &[u8]) -> usize {
    if pattern.is_empty() {
        return 0;
    }

    let mut count = 0;
    let mut i = 0;

    while i + pattern.len() <= data.len() {
        if &data[i..i + pattern.len()] == pattern {
            count += 1;
            i += pattern.len();
        } else {
            i += 1;
        }
    }

    count
}

// ============================================================================
// PRODUCTION EXAMPLE (commented out - requires memmap2 crate)
// ============================================================================

/*
use memmap2::{Mmap, MmapOptions};

fn search_with_mmap(path: &str, pattern: &str) -> io::Result<usize> {
    let file = File::open(path)?;

    // Create memory mapping (unsafe because file can change externally)
    let mmap = unsafe { MmapOptions::new().map(&file)? };

    // Now we can access file as &[u8] slice
    // This is zero-copy - no data is copied to userspace
    Ok(count_pattern(&mmap, pattern.as_bytes()))
}

fn parallel_mmap_search(path: &str, pattern: &str) -> io::Result<usize> {
    let file = File::open(path)?;
    let mmap = unsafe { MmapOptions::new().map(&file)? };

    let num_threads = num_cpus::get();
    let chunk_size = mmap.len() / num_threads;

    let pattern_bytes = pattern.as_bytes();

    // Split mmap into chunks and search in parallel
    let handles: Vec<_> = (0..num_threads)
        .map(|i| {
            let start = i * chunk_size;
            let end = if i == num_threads - 1 {
                mmap.len()
            } else {
                ((i + 1) * chunk_size + pattern_bytes.len() - 1).min(mmap.len())
            };

            let chunk = &mmap[start..end];
            let pattern_owned = pattern.to_string();

            std::thread::spawn(move || {
                count_pattern(chunk, pattern_owned.as_bytes())
            })
        })
        .collect();

    let total: usize = handles
        .into_iter()
        .map(|h| h.join().unwrap())
        .sum();

    Ok(total)
}

// Advanced: Read-only vs mutable mapping
fn create_mutable_mapping(path: &str) -> io::Result<Mmap> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)?;

    // Mutable mapping - changes written back to file!
    unsafe { MmapOptions::new().map_mut(&file) }
}

// Advanced: Anonymous mapping (not backed by file)
fn create_anonymous_mapping(size: usize) -> io::Result<Mmap> {
    // Creates memory-only mapping (no file backing)
    // Useful for shared memory between processes
    MmapOptions::new().len(size).map_anon()
}
*/

// ============================================================================
// WHAT RUST DOES UNDER THE HOOD
// ============================================================================
// 1. MMAP SYSCALL
//    - On Unix: mmap(2) system call
//    - On Windows: CreateFileMapping + MapViewOfFile
//    - OS maps file pages into process virtual address space
//    - No actual data loading yet (lazy)
//
// 2. PAGE FAULTS
//    - First access to page triggers page fault
//    - CPU traps to OS
//    - OS loads page from disk to RAM
//    - OS updates page table
//    - CPU retries instruction (succeeds this time)
//    - Subsequent accesses: no page fault (already in RAM)
//
// 3. PAGE CACHE
//    - OS maintains page cache (file system cache)
//    - All file I/O goes through page cache
//    - mmap uses same cache as read/write
//    - Multiple processes share cached pages
//
// 4. MEMORY OVERHEAD
//    - Virtual memory: file size (doesn't count against RAM)
//    - Physical memory: pages actually loaded
//    - Page table entries: ~8 bytes per 4KB page
//    - For 1GB file: ~2MB page table overhead
//
// 5. UNMAPPING
//    - When Mmap is dropped, munmap(2) called
//    - Removes mapping from virtual address space
//    - Pages stay in page cache (until evicted)
//    - File descriptor can be closed (mapping persists)
//
// 6. UNSAFE JUSTIFICATION
//    - File can be modified by other processes (race)
//    - File can be truncated (access beyond EOF = segfault)
//    - No way to prevent this in safe Rust
//    - Hence: unsafe block required

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. Memory mapping provides zero-copy file access
// 2. OS manages paging and caching automatically
// 3. Great for large files with random access
// 4. Parallel access is natural (different regions)
// 5. First access is slow (page faults), subsequent accesses fast
// 6. Inherently unsafe - requires careful handling
// 7. Use memmap2 crate for production code
// 8. Not always faster than buffered I/O (profile first!)
// 9. OS can use all available RAM for caching
// 10. Shared mappings enable efficient IPC

// ============================================================================
// WHEN TO USE MMAP
// ============================================================================
// ✅ GOOD USE CASES:
// - Large files (> 1MB)
// - Random access patterns
// - Multiple passes over data
// - Parallel processing
// - Read-modify-write operations
// - Shared memory between processes
// - Database files
// - Index files (search engines)
//
// ❌ BAD USE CASES:
// - Small files (< page size = 4KB)
// - Sequential one-time read
// - Network filesystems (high latency)
// - Limited RAM (file > available RAM)
// - Untrusted file sources (security)
// - Frequently modified files

// ============================================================================
// PERFORMANCE OPTIMIZATION
// ============================================================================
// 1. SEQUENTIAL ACCESS HINT
//    - madvise(MADV_SEQUENTIAL) tells OS to prefetch
//    - Linux: readahead more aggressively
//    - Can double sequential read performance
//
// 2. RANDOM ACCESS HINT
//    - madvise(MADV_RANDOM) tells OS not to prefetch
//    - Reduces wasted I/O for random access
//
// 3. WILLNEED HINT
//    - madvise(MADV_WILLNEED) prefetches pages
//    - Useful before accessing cold data
//    - Hides latency by loading asynchronously
//
// 4. HUGE PAGES
//    - madvise(MADV_HUGEPAGE) uses 2MB pages
//    - Reduces TLB misses (page table lookups)
//    - Can give 10-20% speedup for large files
//
// 5. PARALLEL ACCESS
//    - Split file into chunks
//    - Process each chunk on separate thread
//    - Near-linear speedup for CPU-bound work

// ============================================================================
// COMPARISON: MMAP VS READ VS IO_URING
// ============================================================================
// MMAP:
//   - Best for: Large files, random access, multiple passes
//   - Pros: Zero-copy, OS-managed caching, simple API
//   - Cons: Unsafe, page faults, not ideal for sequential
//
// READ (BUFFERED):
//   - Best for: Sequential access, small files, network I/O
//   - Pros: Safe, predictable, works everywhere
//   - Cons: Data copying, manual buffering
//
// IO_URING (LINUX):
//   - Best for: High-throughput I/O, many files, async
//   - Pros: Fastest for high concurrency, no syscalls
//   - Cons: Linux-only, complex, overkill for simple cases
//
// RECOMMENDATION:
//   - Start with buffered I/O (simplest)
//   - Profile to find bottlenecks
//   - Use mmap if random access or multiple passes
//   - Use io_uring for extreme performance needs

// ============================================================================
// COMMON BEGINNER MISTAKES
// ============================================================================
// ❌ Using mmap for small files (overhead > benefit)
// ❌ Not checking file size before accessing
// ❌ Assuming mmap is always faster (profile first!)
// ❌ Forgetting to handle truncation races
// ❌ Holding mapping too long (leaking file descriptors)
// ❌ Not using read-only mappings when possible
// ❌ Ignoring cross-platform differences
// ❌ Not handling SIGBUS (access beyond mapped region)

// ============================================================================
// ADVANCED: MEMORY MAPPING INTERNALS
// ============================================================================
// VIRTUAL MEMORY:
//   - Each process has 48-bit address space (256TB on x86-64)
//   - Divided into pages (typically 4KB)
//   - Page table maps virtual → physical addresses
//
// PAGE TABLE:
//   - Multi-level tree (4 levels on x86-64)
//   - TLB (Translation Lookaside Buffer) caches mappings
//   - TLB miss: walk page table (~100 cycles)
//   - TLB hit: ~1 cycle
//
// PAGE FAULT:
//   - Minor fault: page in RAM but not mapped (cheap)
//   - Major fault: page on disk, must load (expensive)
//   - Segfault: access unmapped region (SIGSEGV)
//
// FILE-BACKED VS ANONYMOUS:
//   - File-backed: changes written to file (if MAP_SHARED)
//   - Anonymous: no file, used for heap/stack
//   - Copy-on-write: shared until write (MAP_PRIVATE)
