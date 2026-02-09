//! # High-Performance Search with Memory-Mapped Files - Your Implementation
//!
//! This project involves implementing and comparing different file searching
//! strategies, including using memory-mapped files for high performance.
//!
//! ## Your Task
//!
//! Implement the three functions below.
//!
//! 1.  **`create_test_file()`**: A helper to generate a large file with pseudo-random
//!     data but with a known pattern sprinkled throughout.
//!
//! 2.  **`search_with_read()`**: The baseline implementation. It should read the
//!     file chunk by chunk using a `BufReader` and count occurrences of the pattern.
//!
//! 3.  **`search_with_mmap()`**: The high-performance version. It should use the
//!     `memmap2` crate to map the file into memory and then search for the
//!     pattern directly in the memory slice.
//!
//! 4.  **`parallel_search_with_mmap()` (Stretch Goal)**: Combine the memory map
//!     with `rayon` to perform the search in parallel.
//!
//! ## Running Your Code
//!
//! ```bash
//! cargo test -p memmap-search
//! cargo run -p memmap-search --release
//! ```
//!
//! ## Stuck?
//!
//! Check out `src/solution.rs` for a complete, heavily-commented solution.

use std::fs::File;
use std::io::{self, BufReader, Read, Write};
use std::path::Path;
use memmap2::Mmap;

/// Creates a large test file with a given size in megabytes.
///
/// The file will contain pseudo-random bytes, with the `pattern`
/// inserted at regular intervals.
pub fn create_test_file(
    path: &Path,
    size_mb: usize,
    pattern: &str,
) -> io::Result<()> {
    // TODO: Implement the test file creation.
    // 1. Open a file at `path` for writing.
    // 2. Use a `BufWriter` for efficiency.
    // 3. Loop `size_mb` times to write megabytes of data.
    // 4. In each iteration, write a large chunk of pseudo-random bytes.
    //    You can just repeat a simple sequence.
    // 5. Sprinkle the `pattern` into the data periodically.
    // 6. Make sure the total file size is approximately `size_mb`.
    todo!("Implement create_test_file");
}

/// Searches for a pattern in a file using standard buffered reading.
pub fn search_with_read(path: &Path, pattern: &str) -> io::Result<usize> {
    // TODO: Implement search with a BufReader.
    // 1. Open the file and wrap it in a `BufReader`.
    // 2. Read the file chunk by chunk into a buffer.
    // 3. Search for the pattern in the buffer. Be careful about matches
    //    that could span across chunk boundaries! A simple way to handle
    //    this is to use overlapping chunks.
    // 4. Count and return the total number of occurrences.
    todo!("Implement search_with_read");
}

/// Searches for a pattern in a file using a memory map.
pub fn search_with_mmap(path: &Path, pattern: &str) -> io::Result<usize> {
    // TODO: Implement search with a memory map.
    // 1. Open the file.
    // 2. Use `memmap2::Mmap::map()` to map the file into memory. This is an
    //    `unsafe` operation because the file could be modified by another
    //    process.
    // 3. Treat the mmap object as a byte slice (`&[u8]`).
    // 4. Use slice methods like `.windows()` to efficiently find all
    //    non-overlapping occurrences of the pattern.
    // 5. Return the count.
    todo!("Implement search_with_mmap");
}

/// (Stretch Goal) Searches for a pattern in a memory-mapped file in parallel.
pub fn parallel_search_with_mmap(path: &Path, pattern: &str) -> io::Result<usize> {
    // TODO: Implement parallel search.
    // 1. Memory-map the file as in `search_with_mmap`.
    // 2. Use Rayon's `.par_windows()` on the byte slice.
    // 3. Use `.filter()` and `.count()` to count occurrences in parallel.
    todo!("Implement parallel_search_with_mmap");
}


// Re-export the solution module so people can compare
#[doc(hidden)]
pub mod solution;