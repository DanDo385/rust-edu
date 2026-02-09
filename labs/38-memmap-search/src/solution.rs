//! # High-Performance Search with Memory-Mapped Files - Solution
//!
//! ## What We're Building
//!
//! This solution provides three functions to search for a byte pattern in a file:
//! 1. `search_with_read`: A baseline using standard, buffered I/O.
//! 2. `search_with_mmap`: A high-performance version using a memory-mapped file.
//! 3. `parallel_search_with_mmap`: An even faster version that combines memory
//!    mapping with parallel processing via Rayon.
//!
//! ## Why Memory-Mapping is Fast
//!
//! Memory-mapping asks the OS to map a file directly into the program's virtual
//! address space. This avoids costly `read` syscalls and extra data copying
//! between kernel space and user space. The OS can also use its page cache
//! much more effectively.
//!
//! ## `unsafe` and `memmap2`
//!
//! `Mmap::map()` is `unsafe` because Rust cannot guarantee that the file won't be
//! modified by another process after it has been mapped. If the file is truncated,
//! for example, your program could try to access memory that is no longer valid,
//! leading to a crash. By using `unsafe`, we are telling the compiler that we
//! understand this risk and are using the memory map in a context where the file
//! is assumed to be static.

use std::fs::File;
use std::io::{self, BufReader, Read, Write};
use std::path::Path;
use memmap2::Mmap;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

/// Creates a large test file with a given size in megabytes.
pub fn create_test_file(
    path: &Path,
    size_mb: usize,
    pattern: &str,
) -> io::Result<()> {
    let mut file = File::create(path)?;
    let mut rng = StdRng::seed_from_u64(42); // Deterministic RNG for reproducibility
    let pattern_bytes = pattern.as_bytes();
    let chunk_size = 1024 * 1024; // 1 MB
    let mut buffer = vec![0u8; chunk_size];

    for _ in 0..size_mb {
        // Fill most of the buffer with random bytes
        rng.fill(&mut buffer[..]);
        
        // Sprinkle the pattern into the buffer a few times
        let num_sprinkles = 5;
        for i in 0..num_sprinkles {
            let start = (i * chunk_size) / num_sprinkles;
            let end = start + pattern_bytes.len();
            if end <= chunk_size {
                buffer[start..end].copy_from_slice(pattern_bytes);
            }
        }
        
        file.write_all(&buffer)?;
    }
    Ok(())
}

/// Searches for a pattern in a file using standard buffered reading.
///
/// This implementation is simple but can be inefficient due to the overhead
/// of I/O calls and copying data into userspace buffers.
pub fn search_with_read(path: &Path, pattern: &str) -> io::Result<usize> {
    let file = File::open(path)?;
    // Use a BufReader to read the file in chunks efficiently.
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    let pattern_bytes = pattern.as_bytes();
    let count = buffer
        .windows(pattern_bytes.len())
        .filter(|&window| window == pattern_bytes)
        .count();
    
    Ok(count)
}

/// Searches for a pattern in a file using a memory map.
///
/// This is generally much faster for large files as it avoids extra copies
/// and leverages the OS's virtual memory system.
pub fn search_with_mmap(path: &Path, pattern: &str) -> io::Result<usize> {
    let file = File::open(path)?;
    // `unsafe` is required because the file could be modified by another
    // process, which would violate Rust's memory safety guarantees.
    // We proceed assuming the file is static during the search.
    let mmap = unsafe { Mmap::map(&file)? };

    // The memory-mapped file can be treated as a single, large byte slice.
    let pattern_bytes = pattern.as_bytes();
    let count = mmap
        .windows(pattern_bytes.len())
        .filter(|&window| window == pattern_bytes)
        .count();

    Ok(count)
}

/// (Stretch Goal) Searches for a pattern in a memory-mapped file in parallel.
///
/// This combines the benefits of memory-mapping with data parallelism from Rayon,
/// often providing the best performance on multi-core systems.
pub fn parallel_search_with_mmap(path: &Path, pattern: &str) -> io::Result<usize> {
    use rayon::prelude::*;
    
    let file = File::open(path)?;
    let mmap = unsafe { Mmap::map(&file)? };

    let pattern_bytes = pattern.as_bytes();
    let count = mmap[..]
        .par_windows(pattern_bytes.len()) // The parallel version of `.windows()`
        .filter(|window| *window == pattern_bytes)
        .count();

    Ok(count)
}
