//! Integration tests for Lab 16: Collections & Iterators

use collections_iterators::solution::*;

#[test]
fn test_sum_evens_mixed() {
    let nums = vec![10, 20, 30, 40, 50];
    assert_eq!(sum_evens(&nums), 150); // All are even
}

#[test]
fn test_sum_evens_empty() {
    let nums: Vec<i32> = vec![];
    assert_eq!(sum_evens(&nums), 0);
}

#[test]
fn test_sum_evens_odd_only() {
    let nums = vec![1, 3, 5, 7];
    assert_eq!(sum_evens(&nums), 0); // No evens
}

#[test]
fn test_count_matching_custom() {
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let greater_than_5 = count_matching(&nums, |n| n > 5);
    assert_eq!(greater_than_5, 3); // 6, 7, 8
}

#[test]
fn test_all_positive_edge_zero() {
    let nums = vec![1, 2, 0, 3];
    assert!(!all_positive(&nums)); // 0 is not positive
}

#[test]
fn test_all_positive_single() {
    let nums = vec![1];
    assert!(all_positive(&nums));
}

#[test]
fn test_group_consecutive() {
    let nums = vec![1, 1, 2, 2, 2, 3, 1, 1];
    let groups = group_consecutive(&nums);
    assert_eq!(groups.len(), 4);
    assert_eq!(groups[0], vec![1, 1]);
    assert_eq!(groups[1], vec![2, 2, 2]);
}

#[test]
fn test_to_strings() {
    let nums = vec![1, 2, 3];
    let strings = to_strings(&nums);
    assert_eq!(strings, vec!["1", "2", "3"]);
}

#[test]
fn test_to_strings_ownership() {
    // Verify we can modify strings (we own them)
    let nums = vec![10, 20];
    let mut strings = to_strings(&nums);
    strings.push("30".to_string());
    assert_eq!(strings.len(), 3);
}

#[test]
fn test_find_max_negative() {
    let nums = vec![-5, -2, -10];
    assert_eq!(find_max(&nums), Some(-2));
}

#[test]
fn test_find_first_even_multiple() {
    let nums = vec![1, 3, 2, 4, 6];
    assert_eq!(find_first_even(&nums), Some(2)); // Returns first even
}

#[test]
fn test_iterator_lazy_evaluation() {
    // This demonstrates that iterators are lazy (don't compute until needed)
    let nums = vec![1, 2, 3, 4, 5];
    let _iter = nums
        .iter()
        .filter(|&&n| n > 2)
        .map(|n| n * 2);
    // Iteration doesn't happen until we consume with collect/sum/etc
    // This is a zero-cost abstraction - the compiler optimizes it away
}
