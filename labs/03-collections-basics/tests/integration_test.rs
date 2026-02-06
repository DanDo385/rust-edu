//! Integration tests for collections-basics

use collections_basics::solution::*;

#[test]
fn test_sum_of_evens_basic() {
    assert_eq!(sum_of_evens(&[1, 2, 3, 4, 5, 6]), 12);
}

#[test]
fn test_sum_of_evens_empty() {
    assert_eq!(sum_of_evens(&[]), 0);
}

#[test]
fn test_sum_of_evens_all_odd() {
    assert_eq!(sum_of_evens(&[1, 3, 5, 7]), 0);
}

#[test]
fn test_sum_of_evens_all_even() {
    assert_eq!(sum_of_evens(&[2, 4, 6, 8]), 20);
}

#[test]
fn test_sum_of_evens_negatives() {
    assert_eq!(sum_of_evens(&[-4, -2, 0, 2, 4]), 0);
}

#[test]
fn test_word_frequency_basic() {
    let freq = word_frequency("hello world hello");
    assert_eq!(freq.get("hello"), Some(&2));
    assert_eq!(freq.get("world"), Some(&1));
}

#[test]
fn test_word_frequency_empty() {
    let freq = word_frequency("");
    assert!(freq.is_empty());
}

#[test]
fn test_word_frequency_case_insensitive() {
    let freq = word_frequency("Hello HELLO hello");
    assert_eq!(freq.get("hello"), Some(&3));
}

#[test]
fn test_word_frequency_multiple_spaces() {
    let freq = word_frequency("hello    world");
    assert_eq!(freq.get("hello"), Some(&1));
    assert_eq!(freq.get("world"), Some(&1));
    assert_eq!(freq.len(), 2);
}

#[test]
fn test_filter_and_sort_basic() {
    let result = filter_and_sort(&[5, 2, 8, 1, 9, 3], 2, 5);
    assert_eq!(result, vec![2, 3, 5]);
}

#[test]
fn test_filter_and_sort_empty() {
    let result = filter_and_sort(&[], 0, 10);
    assert_eq!(result, Vec::<i32>::new());
}

#[test]
fn test_filter_and_sort_none_in_range() {
    let result = filter_and_sort(&[1, 2, 3], 10, 20);
    assert_eq!(result, Vec::<i32>::new());
}

#[test]
fn test_filter_and_sort_all_in_range() {
    let result = filter_and_sort(&[3, 1, 2], 1, 10);
    assert_eq!(result, vec![1, 2, 3]);
}

#[test]
fn test_filter_and_sort_negative() {
    let result = filter_and_sort(&[-5, -2, 0, 3, 8], -3, 5);
    assert_eq!(result, vec![-2, 0, 3]);
}

#[test]
fn test_most_common_word_basic() {
    let result = most_common_word("the cat and the dog");
    assert_eq!(result, Some("the".to_string()));
}

#[test]
fn test_most_common_word_empty() {
    let result = most_common_word("");
    assert_eq!(result, None);
}

#[test]
fn test_most_common_word_tie() {
    // With tie, any of the most common words is valid
    let result = most_common_word("cat dog cat dog");
    assert!(result == Some("cat".to_string()) || result == Some("dog".to_string()));
}

#[test]
fn test_most_common_word_single() {
    let result = most_common_word("hello");
    assert_eq!(result, Some("hello".to_string()));
}

#[test]
fn test_most_common_word_case_insensitive() {
    let result = most_common_word("Rust rust RUST");
    assert_eq!(result, Some("rust".to_string()));
}
