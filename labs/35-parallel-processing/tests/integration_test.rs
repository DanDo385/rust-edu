//! Integration tests for Lab 35: Parallel Processing
//!
//! These tests verify that the parallel implementations produce the exact
//! same results as their sequential counterparts.

use parallel_processing::solution::*;

#[test]
fn test_sum_of_squares_parallel_matches_sequential() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let seq_result = sum_of_squares_sequential(&numbers);
    let par_result = sum_of_squares_parallel(&numbers);
    assert_eq!(seq_result, par_result);
}

#[test]
fn test_sum_of_squares_empty_slice() {
    let numbers: Vec<i32> = vec![];
    assert_eq!(sum_of_squares_sequential(&numbers), 0);
    assert_eq!(sum_of_squares_parallel(&numbers), 0);
}

#[test]
fn test_sum_of_squares_large_slice() {
    let numbers: Vec<i32> = (0..1000).collect();
    let seq_result = sum_of_squares_sequential(&numbers);
    let par_result = sum_of_squares_parallel(&numbers);
    assert_eq!(seq_result, par_result);
}

#[test]
fn test_find_primes_parallel_matches_sequential() {
    let limit = 1000;
    let seq_result = find_primes_sequential(limit);
    let par_result = find_primes_parallel(limit);
    assert_eq!(seq_result, par_result);
}

#[test]
fn test_find_primes_small_limit() {
    let limit = 10;
    let expected = vec![2, 3, 5, 7];
    assert_eq!(find_primes_parallel(limit), expected);
}

#[test]
fn test_find_primes_no_primes() {
    let limit = 1;
    assert!(find_primes_parallel(limit).is_empty());
}

#[test]
fn test_parallel_map_doubles_numbers() {
    let data = vec![1, 2, 3, 4, 5];
    let double = |x| x * 2;
    let result = parallel_map(&data, double);
    assert_eq!(result, vec![2, 4, 6, 8, 10]);
}

#[test]
fn test_parallel_map_empty_slice() {
    let data: Vec<i32> = vec![];
    let identity = |x| x;
    let result = parallel_map(&data, identity);
    assert!(result.is_empty());
}

#[test]
fn test_parallel_map_string_conversion() {
    let data = vec![10, 20, 30];
    let to_string = |x| format!("Number: {}", x);
    let result = parallel_map(&data, to_string);
    assert_eq!(result, vec!["Number: 10", "Number: 20", "Number: 30"]);
}