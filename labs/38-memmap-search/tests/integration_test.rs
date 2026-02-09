//! Integration tests for Lab 38: Memory-Mapped File Search

use memmap_search::solution::{
    create_test_file, search_with_read, search_with_mmap, parallel_search_with_mmap,
};
use std::io;
use tempfile::Builder;

fn run_search_test(file_size_mb: usize, pattern: &str) -> io::Result<()> {
    let temp_dir = Builder::new().prefix("memmap_test").tempdir()?;
    let file_path = temp_dir.path().join("test.txt");

    create_test_file(&file_path, file_size_mb, pattern)?;

    // The number of sprinkles in the solution's create_test_file
    let expected_count = 5 * file_size_mb;

    // Test all three methods
    let count_read = search_with_read(&file_path, pattern)?;
    assert_eq!(count_read, expected_count, "Read method failed");

    let count_mmap = search_with_mmap(&file_path, pattern)?;
    assert_eq!(count_mmap, expected_count, "Mmap method failed");
    
    let count_par_mmap = parallel_search_with_mmap(&file_path, pattern)?;
    assert_eq!(count_par_mmap, expected_count, "Parallel Mmap method failed");

    Ok(())
}

#[test]
fn test_searches_on_small_file() -> io::Result<()> {
    run_search_test(1, "RUST")
}

#[test]
fn test_searches_on_medium_file() -> io::Result<()> {
    run_search_test(10, "PATTERN")
}

#[test]
fn test_pattern_not_found() -> io::Result<()> {
    let temp_dir = Builder::new().prefix("not_found").tempdir()?;
    let file_path = temp_dir.path().join("test.txt");
    create_test_file(&file_path, 1, "SOME_DATA")?;

    assert_eq!(search_with_read(&file_path, "NOT_HERE")?, 0);
    assert_eq!(search_with_mmap(&file_path, "NOT_HERE")?, 0);
    assert_eq!(parallel_search_with_mmap(&file_path, "NOT_HERE")?, 0);

    Ok(())
}

#[test]
fn test_empty_file() -> io::Result<()> {
    let temp_dir = Builder::new().prefix("empty_file").tempdir()?;
    let file_path = temp_dir.path().join("test.txt");
    std::fs::write(&file_path, "")?;

    assert_eq!(search_with_read(&file_path, "a")?, 0);
    assert_eq!(search_with_mmap(&file_path, "a")?, 0);
    assert_eq!(parallel_search_with_mmap(&file_path, "a")?, 0);

    Ok(())
}