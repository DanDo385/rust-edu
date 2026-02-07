// Integration tests for Lab 38: Memory-Mapped File Search
//
// Tests pattern counting, file search, case-insensitive search,
// parallel search, and file creation utilities.

use memmap_search::{
    count_pattern, count_pattern_case_insensitive, count_pattern_case_insensitive_str,
    count_pattern_str, create_small_test_file, create_test_file, parallel_count,
    parallel_search_file, search_file, search_file_lines, SearchResult,
};
use std::fs;

// ============================================================================
// PATTERN COUNTING (BYTE SLICES)
// ============================================================================

#[test]
fn test_count_single_occurrence() {
    assert_eq!(count_pattern(b"hello world", b"hello"), 1);
}

#[test]
fn test_count_multiple_occurrences() {
    assert_eq!(count_pattern(b"abcabcabc", b"abc"), 3);
}

#[test]
fn test_count_no_match() {
    assert_eq!(count_pattern(b"hello world", b"xyz"), 0);
}

#[test]
fn test_count_empty_pattern() {
    assert_eq!(count_pattern(b"hello", b""), 0);
}

#[test]
fn test_count_empty_data() {
    assert_eq!(count_pattern(b"", b"hello"), 0);
}

#[test]
fn test_count_both_empty() {
    assert_eq!(count_pattern(b"", b""), 0);
}

#[test]
fn test_count_pattern_longer_than_data() {
    assert_eq!(count_pattern(b"hi", b"hello"), 0);
}

#[test]
fn test_count_exact_match() {
    assert_eq!(count_pattern(b"abc", b"abc"), 1);
}

#[test]
fn test_count_non_overlapping() {
    // "aaa" contains "aa" once (non-overlapping), not twice (overlapping)
    assert_eq!(count_pattern(b"aaa", b"aa"), 1);
}

#[test]
fn test_count_non_overlapping_longer() {
    // "aaaa" contains "aa" twice (non-overlapping)
    assert_eq!(count_pattern(b"aaaa", b"aa"), 2);
}

#[test]
fn test_count_at_end() {
    assert_eq!(count_pattern(b"xxxabc", b"abc"), 1);
}

#[test]
fn test_count_at_start() {
    assert_eq!(count_pattern(b"abcxxx", b"abc"), 1);
}

#[test]
fn test_count_single_byte_pattern() {
    assert_eq!(count_pattern(b"aabbaabb", b"a"), 4);
}

// ============================================================================
// PATTERN COUNTING (STRINGS)
// ============================================================================

#[test]
fn test_count_str_basic() {
    assert_eq!(count_pattern_str("hello world hello", "hello"), 2);
}

#[test]
fn test_count_str_no_match() {
    assert_eq!(count_pattern_str("hello world", "xyz"), 0);
}

#[test]
fn test_count_str_empty() {
    assert_eq!(count_pattern_str("", "hello"), 0);
    assert_eq!(count_pattern_str("hello", ""), 0);
}

// ============================================================================
// CASE-INSENSITIVE SEARCH
// ============================================================================

#[test]
fn test_case_insensitive_all_cases() {
    assert_eq!(
        count_pattern_case_insensitive(b"Hello HELLO hello HeLLo", b"hello"),
        4
    );
}

#[test]
fn test_case_insensitive_no_match() {
    assert_eq!(
        count_pattern_case_insensitive(b"hello world", b"xyz"),
        0
    );
}

#[test]
fn test_case_insensitive_empty() {
    assert_eq!(count_pattern_case_insensitive(b"hello", b""), 0);
    assert_eq!(count_pattern_case_insensitive(b"", b"hello"), 0);
}

#[test]
fn test_case_insensitive_str() {
    assert_eq!(
        count_pattern_case_insensitive_str("Rust RUST rust", "rust"),
        3
    );
}

#[test]
fn test_case_insensitive_mixed_content() {
    let data = b"The TARGET was found. The target was also found. And also TARGET.";
    assert_eq!(count_pattern_case_insensitive(data, b"target"), 3);
}

// ============================================================================
// FILE SEARCH
// ============================================================================

#[test]
fn test_search_file_basic() {
    let path = "/tmp/memmap_test_search_basic.txt";
    create_small_test_file(path, "hello world\nhello rust\ngoodbye world\n").unwrap();

    let count = search_file(path, "hello").unwrap();
    assert_eq!(count, 2);

    fs::remove_file(path).unwrap();
}

#[test]
fn test_search_file_no_match() {
    let path = "/tmp/memmap_test_search_no_match.txt";
    create_small_test_file(path, "hello world\n").unwrap();

    let count = search_file(path, "xyz").unwrap();
    assert_eq!(count, 0);

    fs::remove_file(path).unwrap();
}

#[test]
fn test_search_file_nonexistent() {
    let result = search_file("/tmp/nonexistent_file_xyz_12345.txt", "hello");
    assert!(result.is_err());
}

#[test]
fn test_search_file_empty_file() {
    let path = "/tmp/memmap_test_search_empty.txt";
    create_small_test_file(path, "").unwrap();

    let count = search_file(path, "hello").unwrap();
    assert_eq!(count, 0);

    fs::remove_file(path).unwrap();
}

// ============================================================================
// FILE LINE SEARCH
// ============================================================================

#[test]
fn test_search_file_lines_basic() {
    let path = "/tmp/memmap_test_lines_basic.txt";
    create_small_test_file(
        path,
        "line one\nTODO: fix this\nline three\nTODO: also this\nline five\n",
    )
    .unwrap();

    let lines = search_file_lines(path, "TODO").unwrap();
    assert_eq!(lines, vec![2, 4]);

    fs::remove_file(path).unwrap();
}

#[test]
fn test_search_file_lines_no_match() {
    let path = "/tmp/memmap_test_lines_no_match.txt";
    create_small_test_file(path, "hello\nworld\n").unwrap();

    let lines = search_file_lines(path, "xyz").unwrap();
    assert!(lines.is_empty());

    fs::remove_file(path).unwrap();
}

#[test]
fn test_search_file_lines_all_match() {
    let path = "/tmp/memmap_test_lines_all.txt";
    create_small_test_file(path, "abc\nabc\nabc\n").unwrap();

    let lines = search_file_lines(path, "abc").unwrap();
    assert_eq!(lines, vec![1, 2, 3]);

    fs::remove_file(path).unwrap();
}

#[test]
fn test_search_file_lines_first_line() {
    let path = "/tmp/memmap_test_lines_first.txt";
    create_small_test_file(path, "TARGET here\nnot here\nnot here\n").unwrap();

    let lines = search_file_lines(path, "TARGET").unwrap();
    assert_eq!(lines, vec![1]);

    fs::remove_file(path).unwrap();
}

#[test]
fn test_search_file_lines_last_line() {
    let path = "/tmp/memmap_test_lines_last.txt";
    create_small_test_file(path, "not here\nnot here\nTARGET here\n").unwrap();

    let lines = search_file_lines(path, "TARGET").unwrap();
    assert_eq!(lines, vec![3]);

    fs::remove_file(path).unwrap();
}

// ============================================================================
// PARALLEL SEARCH
// ============================================================================

#[test]
fn test_parallel_count_basic() {
    let data = b"abc def abc ghi abc jkl abc";
    assert_eq!(parallel_count(data, b"abc", 2), 4);
}

#[test]
fn test_parallel_count_single_thread() {
    let data = b"hello hello hello";
    assert_eq!(parallel_count(data, b"hello", 1), 3);
}

#[test]
fn test_parallel_count_many_threads() {
    let data = b"abc abc abc abc abc abc abc abc";
    // More threads than reasonable for data size
    assert_eq!(parallel_count(data, b"abc", 8), 8);
}

#[test]
fn test_parallel_count_empty_data() {
    assert_eq!(parallel_count(b"", b"abc", 4), 0);
}

#[test]
fn test_parallel_count_empty_pattern() {
    assert_eq!(parallel_count(b"hello", b"", 4), 0);
}

#[test]
fn test_parallel_count_zero_chunks() {
    assert_eq!(parallel_count(b"hello", b"hello", 0), 0);
}

#[test]
fn test_parallel_count_matches_sequential() {
    let data = b"The quick brown fox jumps over the lazy dog. The fox is quick.";
    let pattern = b"the";
    // Case-sensitive: only lowercase "the" matches
    let sequential = count_pattern(data, pattern);
    let parallel = parallel_count(data, pattern, 4);
    assert_eq!(sequential, parallel);
}

#[test]
fn test_parallel_count_boundary_match() {
    // Pattern straddles a chunk boundary
    // With 2 chunks on 10 bytes, split is at position 5
    // "abcde|fghij" -- pattern "ef" crosses boundary
    let data = b"abcdefghij";
    assert_eq!(parallel_count(data, b"ef", 2), 1);
}

// ============================================================================
// FILE-BASED PARALLEL SEARCH
// ============================================================================

#[test]
fn test_parallel_search_file() {
    let path = "/tmp/memmap_test_parallel_file.txt";
    create_small_test_file(
        path,
        "TARGET one\nnothing\nTARGET two\nnothing\nTARGET three\n",
    )
    .unwrap();

    let count = parallel_search_file(path, "TARGET", 2).unwrap();
    assert_eq!(count, 3);

    fs::remove_file(path).unwrap();
}

#[test]
fn test_parallel_search_file_nonexistent() {
    let result = parallel_search_file("/tmp/nonexistent_xyz_99999.txt", "hello", 4);
    assert!(result.is_err());
}

// ============================================================================
// TEST FILE CREATION
// ============================================================================

#[test]
fn test_create_small_test_file() {
    let path = "/tmp/memmap_test_small_create.txt";
    create_small_test_file(path, "hello world").unwrap();

    let content = fs::read_to_string(path).unwrap();
    assert_eq!(content, "hello world");

    fs::remove_file(path).unwrap();
}

#[test]
fn test_create_test_file_contains_target() {
    let path = "/tmp/memmap_test_target_create.txt";
    let expected_count = create_test_file(path, 1).unwrap();

    // Verify the file actually contains that many TARGETs
    let actual_count = search_file(path, "TARGET").unwrap();
    assert_eq!(actual_count, expected_count);

    fs::remove_file(path).unwrap();
}

// ============================================================================
// SEARCH RESULT TYPE
// ============================================================================

#[test]
fn test_search_result_from_memory() {
    let result = SearchResult::from_memory("hello", 42);
    assert_eq!(result.pattern, "hello");
    assert_eq!(result.count, 42);
    assert!(result.file.is_none());
}

#[test]
fn test_search_result_from_file() {
    let result = SearchResult::from_file("hello", 7, "test.txt");
    assert_eq!(result.pattern, "hello");
    assert_eq!(result.count, 7);
    assert_eq!(result.file.as_deref(), Some("test.txt"));
}

#[test]
fn test_search_result_clone() {
    let result = SearchResult::from_file("pat", 3, "f.txt");
    let cloned = result.clone();
    assert_eq!(result.pattern, cloned.pattern);
    assert_eq!(result.count, cloned.count);
    assert_eq!(result.file, cloned.file);
}

#[test]
fn test_search_result_debug() {
    let result = SearchResult::from_memory("test", 1);
    let debug_str = format!("{:?}", result);
    assert!(debug_str.contains("SearchResult"));
}
