// Lab 38: Memory-Mapped File Search
//
// Demonstrates high-performance file processing concepts using memory-mapped I/O.
// Shows byte-level pattern matching, parallel scanning, and file I/O utilities.
//
// This lab uses pure std -- no external crate dependency. In production,
// you would use the `memmap2` crate for actual mmap system calls.

use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};

// ============================================================================
// PATTERN COUNTING
// ============================================================================

/// Counts non-overlapping occurrences of `pattern` in `data`.
///
/// Uses a simple linear scan: when a match is found, the scan advances
/// past the entire pattern (non-overlapping). This is O(n * m) worst case
/// where n = data length and m = pattern length.
///
/// # Edge Cases
/// - Returns 0 if `pattern` is empty.
/// - Returns 0 if `data` is shorter than `pattern`.
///
/// # Examples
/// ```
/// use memmap_search::count_pattern;
/// assert_eq!(count_pattern(b"hello world hello", b"hello"), 2);
/// assert_eq!(count_pattern(b"aaa", b"aa"), 1); // non-overlapping
/// ```
pub fn count_pattern(data: &[u8], pattern: &[u8]) -> usize {
    if pattern.is_empty() {
        return 0;
    }

    let mut count = 0;
    let mut i = 0;

    while i + pattern.len() <= data.len() {
        if &data[i..i + pattern.len()] == pattern {
            count += 1;
            i += pattern.len(); // Non-overlapping: skip past match
        } else {
            i += 1;
        }
    }

    count
}

/// Counts non-overlapping occurrences of `pattern` in `data` (string version).
///
/// Convenience wrapper that converts string arguments to byte slices.
pub fn count_pattern_str(data: &str, pattern: &str) -> usize {
    count_pattern(data.as_bytes(), pattern.as_bytes())
}

// ============================================================================
// FILE SEARCH
// ============================================================================

/// Reads a file and counts occurrences of `pattern`.
///
/// # Ownership Model
/// The file contents are read into a `Vec<u8>` owned by this function.
/// The vector is allocated on the heap and dropped when the function returns.
/// This is the "traditional I/O" approach -- the entire file is buffered.
///
/// # Performance
/// For large files, this requires O(n) memory where n = file size.
/// In production, memory-mapped I/O (`mmap`) avoids this copy by mapping
/// the file directly into the process address space.
pub fn search_file(path: &str, pattern: &str) -> io::Result<usize> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(count_pattern(&buffer, pattern.as_bytes()))
}

/// Reads a file and returns all line numbers (1-based) containing `pattern`.
///
/// # Examples
/// ```no_run
/// use memmap_search::search_file_lines;
/// let lines = search_file_lines("test.txt", "TODO").unwrap();
/// // lines contains [3, 17, 42] if "TODO" appears on those lines
/// ```
pub fn search_file_lines(path: &str, pattern: &str) -> io::Result<Vec<usize>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let matching_lines: Vec<usize> = contents
        .lines()
        .enumerate()
        .filter(|(_, line)| line.contains(pattern))
        .map(|(i, _)| i + 1) // 1-based line numbers
        .collect();

    Ok(matching_lines)
}

// ============================================================================
// CASE-INSENSITIVE SEARCH
// ============================================================================

/// Counts non-overlapping occurrences of `pattern` in `data`,
/// ignoring ASCII case.
///
/// Both `data` and `pattern` are converted to lowercase before matching.
/// This only handles ASCII case folding (a-z/A-Z), not full Unicode.
///
/// # Examples
/// ```
/// use memmap_search::count_pattern_case_insensitive;
/// assert_eq!(count_pattern_case_insensitive(b"Hello HELLO hello", b"hello"), 3);
/// ```
pub fn count_pattern_case_insensitive(data: &[u8], pattern: &[u8]) -> usize {
    if pattern.is_empty() {
        return 0;
    }

    let data_lower: Vec<u8> = data.iter().map(|b| b.to_ascii_lowercase()).collect();
    let pattern_lower: Vec<u8> = pattern.iter().map(|b| b.to_ascii_lowercase()).collect();

    count_pattern(&data_lower, &pattern_lower)
}

/// Case-insensitive search (string convenience wrapper).
pub fn count_pattern_case_insensitive_str(data: &str, pattern: &str) -> usize {
    count_pattern_case_insensitive(data.as_bytes(), pattern.as_bytes())
}

// ============================================================================
// PARALLEL SEARCH
// ============================================================================

/// Splits `data` into `num_chunks` and counts `pattern` occurrences
/// in parallel using threads.
///
/// # Chunk Overlap
/// Each chunk (except the last) is extended by `pattern.len() - 1` bytes
/// to avoid missing matches that straddle chunk boundaries.
///
/// # Ownership
/// The data slice is borrowed (`&[u8]`). We convert the pattern to an
/// owned `String` for each thread (thread closures must be `'static`
/// unless using scoped threads). We use `std::thread::scope` to avoid
/// the `'static` requirement entirely.
///
/// # Examples
/// ```
/// use memmap_search::parallel_count;
/// let data = b"abc abc abc abc abc abc abc abc";
/// assert_eq!(parallel_count(data, b"abc", 4), 8);
/// ```
pub fn parallel_count(data: &[u8], pattern: &[u8], num_chunks: usize) -> usize {
    if pattern.is_empty() || data.is_empty() || num_chunks == 0 {
        return 0;
    }

    let chunk_size = data.len() / num_chunks;
    if chunk_size == 0 {
        // Data is smaller than number of chunks; just search sequentially
        return count_pattern(data, pattern);
    }

    std::thread::scope(|s| {
        let handles: Vec<_> = (0..num_chunks)
            .map(|i| {
                let start = i * chunk_size;
                let end = if i == num_chunks - 1 {
                    data.len()
                } else {
                    ((i + 1) * chunk_size + pattern.len() - 1).min(data.len())
                };

                let chunk = &data[start..end];

                s.spawn(move || count_pattern(chunk, pattern))
            })
            .collect();

        handles.into_iter().map(|h| h.join().unwrap()).sum()
    })
}

/// Parallel search on a file: reads file, then splits across threads.
///
/// `num_threads` controls parallelism. For CPU-bound pattern matching,
/// use the number of available cores.
pub fn parallel_search_file(
    path: &str,
    pattern: &str,
    num_threads: usize,
) -> io::Result<usize> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    Ok(parallel_count(&buffer, pattern.as_bytes(), num_threads))
}

// ============================================================================
// TEST FILE UTILITIES
// ============================================================================

/// Creates a test file with repeated content containing a known pattern.
///
/// Each line is: `"Lorem ipsum dolor sit amet ... TARGET here ... aliqua.\n"`
/// The file will be approximately `size_mb` megabytes.
///
/// # Returns
/// The number of times "TARGET" appears in the file.
pub fn create_test_file(path: &str, size_mb: usize) -> io::Result<usize> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;

    let chunk = b"Lorem ipsum dolor sit amet, consectetur adipiscing elit. TARGET here. \
                   Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.\n";

    let chunks_per_mb = (1024 * 1024) / chunk.len();
    let total_chunks = size_mb * chunks_per_mb;

    for _ in 0..total_chunks {
        file.write_all(chunk)?;
    }

    file.flush()?;
    Ok(total_chunks) // One TARGET per chunk
}

/// Creates a small test file with specific content for unit testing.
///
/// Returns the file path for convenience.
pub fn create_small_test_file(path: &str, content: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;
    file.write_all(content.as_bytes())?;
    file.flush()?;
    Ok(())
}

// ============================================================================
// SEARCH RESULT TYPE
// ============================================================================

/// A structured search result for reporting.
#[derive(Debug, Clone)]
pub struct SearchResult {
    /// The pattern that was searched for.
    pub pattern: String,
    /// Number of matches found.
    pub count: usize,
    /// The file that was searched (if applicable).
    pub file: Option<String>,
}

impl SearchResult {
    /// Creates a new search result from in-memory search.
    pub fn from_memory(pattern: &str, count: usize) -> Self {
        SearchResult {
            pattern: pattern.to_string(),
            count,
            file: None,
        }
    }

    /// Creates a new search result from file search.
    pub fn from_file(pattern: &str, count: usize, file: &str) -> Self {
        SearchResult {
            pattern: pattern.to_string(),
            count,
            file: Some(file.to_string()),
        }
    }
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_pattern_basic() {
        assert_eq!(count_pattern(b"abcabc", b"abc"), 2);
    }

    #[test]
    fn test_count_pattern_empty_pattern() {
        assert_eq!(count_pattern(b"hello", b""), 0);
    }

    #[test]
    fn test_count_pattern_empty_data() {
        assert_eq!(count_pattern(b"", b"abc"), 0);
    }

    #[test]
    fn test_count_pattern_no_match() {
        assert_eq!(count_pattern(b"hello world", b"xyz"), 0);
    }

    #[test]
    fn test_case_insensitive() {
        assert_eq!(
            count_pattern_case_insensitive(b"Hello HELLO hello", b"hello"),
            3
        );
    }

    #[test]
    fn test_search_result_from_memory() {
        let result = SearchResult::from_memory("test", 5);
        assert_eq!(result.count, 5);
        assert!(result.file.is_none());
    }
}
