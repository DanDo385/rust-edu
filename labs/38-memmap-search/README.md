# Project 38 - High-Performance Search with Memory-Mapped Files

## What You're Building (Plain English)

You're building a very fast file searching tool. Instead of reading a file piece by piece into memory (which can be slow), you're going to use a technique called "memory mapping." This tells the operating system (OS) to map a file on disk directly into your program's address space.

To your program, it will look like the entire file is just one giant slice (`&[u8]`) in memory. You can search through it incredibly quickly without issuing explicit read calls. The OS handles loading the necessary parts of the file into physical RAM as you access them. This is a common technique used in high-performance tools like `grep` and in databases.

## New Rust Concepts in This Project

-   **Memory-Mapped Files**: The core concept. You'll learn what a memory map is and why it's so fast for read-heavy workloads.
-   **The `memmap2` Crate**: A safe and convenient wrapper around the low-level, platform-specific OS APIs for memory mapping.
-   **Working with Byte Slices (`&[u8]`)**: A memory-mapped file is exposed as a slice of bytes. You'll practice searching for patterns within these slices.
-   **`unsafe` Code**: Creating a memory map can be an `unsafe` operation because the underlying file could be changed by another process while you're reading it, leading to memory safety issues. The `memmap2` crate provides safe abstractions, but understanding the underlying `unsafe` nature is important.
-   **Performance Comparison**: You'll compare the performance of searching a file via a memory map versus traditional buffered reading.

## Rust Syntax You'll See

```rust
use memmap2::Mmap;
use std::fs::File;

// Open a file
let file = File::open("my_large_file.txt")?;

// Memory-map the file into the address space.
// This is `unsafe` because the file can be modified by another process
// at any time, which could violate memory safety.
let mmap = unsafe { Mmap::map(&file)? };

// Now you can treat `mmap` like a giant byte slice!
// `&mmap[..]` gives you a `&[u8]`
let byte_slice = &mmap[..];

// Search for a pattern
let found = byte_slice.windows(pattern.len()).any(|window| window == pattern);
```

## How to Run

```bash
# Run the main binary (creates a test file and compares search methods)
cargo run -p memmap-search --release

# Run the tests
cargo test -p memmap-search

# Check if code compiles
cargo check -p memmap-search
```

## The Exercises

You will implement functions to search for a byte pattern in a file using different methods.

1.  **`create_test_file()` (Helper)**: A function to generate a large file with known content, which you'll use for searching and benchmarking.

2.  **`search_with_read()`**: The "slow" way. This function will read the file in chunks using a `BufReader` and search for the pattern. This serves as your baseline.

3.  **`search_with_mmap()`**: The "fast" way. This function will use `memmap2::Mmap` to map the file into memory and then perform a highly efficient search directly on the resulting byte slice.

4.  **`parallel_search_with_mmap()` (Stretch Goal)**: Combine memory mapping with the `rayon` crate from the previous lab. Since the memory-mapped slice can be safely shared between threads, you can use `.par_windows()` to search for the pattern in parallel, which can be even faster on very large files and multi-core systems.

## Solution Explanation (No Code - Just Ideas)

**Why is Memory-Mapping Fast?**
1.  **No Data Copying**: When you use `read()`, data is copied from the file on disk, into a kernel buffer in the OS, and then copied *again* from the kernel buffer into your program's buffer. A memory map avoids these copies. The OS "pretends" the file is in memory, and your program accesses it directly.
2.  **Lazy Loading**: The entire file isn't loaded into RAM at once. The OS loads pages (small chunks, often 4KB) of the file on demand as your program's code touches that part of the memory map.
3.  **OS-Level Caching**: The OS is very good at caching files. If you access the same part of the memory map again, it's likely the data is already in a fast OS cache.

The trade-off is that it's less suitable for files that change frequently or for writing, and it has a higher startup cost than a simple read. But for searching large, static files, it's often the fastest method.

## Where Rust Shines

-   **Safety Wrappers**: The `memmap2` crate provides a safe API around the underlying `unsafe` OS calls, which is a common pattern in the Rust ecosystem. It handles details like ensuring the file handle outlives the memory map.
-   **Zero-Cost Abstractions**: Accessing the memory map as a slice (`&[u8]`) has no overhead. You're working with a pointer and a length, just as you would in C, but with all of Rust's safety guarantees (like bounds checking) on top.
-   **Integration with `rayon`**: Because the memory-mapped slice is `Sync`, it can be safely used with Rayon's parallel iterators for massive performance gains.

## Common Beginner Mistakes

1.  **File Handle Lifetime**: The memory map is only valid as long as the underlying `File` handle is open. If the `File` is dropped, the map becomes invalid. The `memmap2` crate's API is designed to prevent this with lifetimes.
2.  **Forgetting `unsafe`**: Directly calling `Mmap::map()` is `unsafe`. You are making a promise to the compiler that you will handle the risks of the underlying file changing.
3.  **Treating Text as Bytes**: A memory map is a `&[u8]` (a slice of bytes). If the file is UTF-8 text, you can't just assume every byte is a valid character boundary. Searching for byte patterns is safe, but interpreting the data as a `&str` requires validation (`std::str::from_utf8`).

This lab gives you a glimpse into the world of systems programming and performance optimization, showing how Rust can provide high-level abstractions without sacrificing low-level speed.