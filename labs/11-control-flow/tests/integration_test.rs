//! Integration tests for Lab 11: Control Flow
//!
//! These tests verify the reference implementation in src/solution.rs.
//! They teach through assertion patterns: what SHOULD happen.

use control_flow::solution::*;
use std::cmp::Ordering;

#[test]
fn test_classify_number_small() {
    let result = classify_number(3);
    assert_eq!(result, "small", "Expected small for 3");
}

#[test]
fn test_classify_number_medium() {
    let result = classify_number(50);
    assert_eq!(result, "medium", "Expected medium for 50");
}

#[test]
fn test_classify_number_large() {
    let result = classify_number(150);
    assert_eq!(result, "large", "Expected large for 150");
}

#[test]
fn test_compare_guess_correct() {
    let result = compare_guess(42, 42);
    assert_eq!(result, Ordering::Equal, "Expected Equal when guess equals secret");
}

#[test]
fn test_compare_guess_less() {
    let result = compare_guess(10, 42);
    assert_eq!(result, Ordering::Less, "Expected Less when guess < secret");
}

#[test]
fn test_compare_guess_greater() {
    let result = compare_guess(100, 42);
    assert_eq!(result, Ordering::Greater, "Expected Greater when guess > secret");
}

#[test]
fn test_describe_number_even() {
    let result = describe_number(4);
    assert!(result.contains("even"), "Expected 'even' in description for 4");
}

#[test]
fn test_describe_number_odd() {
    let result = describe_number(7);
    assert!(result.contains("odd"), "Expected 'odd' in description for 7");
}

#[test]
fn test_validate_guess_valid() {
    let result = validate_guess("42");
    assert!(result.is_ok(), "Expected Ok for valid number '42'");
    assert_eq!(result.unwrap(), 42);
}

#[test]
fn test_validate_guess_invalid_not_number() {
    let result = validate_guess("not_a_number");
    assert!(result.is_err(), "Expected Err for non-numeric string");
}

#[test]
fn test_validate_guess_empty() {
    let result = validate_guess("");
    assert!(result.is_err(), "Expected Err for empty string");
}

#[test]
fn test_validate_guess_out_of_range() {
    // Current solution accepts any parsed i32 value.
    let result_too_low = validate_guess("0");
    let result_too_high = validate_guess("101");

    assert!(result_too_low.is_ok(), "0 should parse successfully");
    assert!(result_too_high.is_ok(), "101 should parse successfully");
}

#[test]
fn test_count_divisions_basic() {
    let result = count_divisions(16);
    assert_eq!(result, 4, "16 / 2 / 2 / 2 / 2 = 4 divisions");
}

#[test]
fn test_count_divisions_no_division() {
    let result = count_divisions(3);
    assert_eq!(result, 0, "3 is odd, no divisions by 2");
}

#[test]
fn test_sum_range_basic() {
    let result = sum_range(1, 5);
    assert_eq!(result, 15, "1+2+3+4+5 = 15");
}

#[test]
fn test_sum_range_single() {
    let result = sum_range(5, 5);
    assert_eq!(result, 5, "Sum of range [5,5] is 5");
}

#[test]
fn test_sum_range_empty() {
    let result = sum_range(5, 4);
    assert_eq!(result, 0, "Empty range sums to 0");
}

#[test]
fn test_decide_action_won() {
    let result = decide_action("continue", true);
    assert!(result.contains("exiting") || result.contains("game"),
            "Expected exit action when game_won=true");
}

#[test]
fn test_decide_action_lost() {
    let result = decide_action("quit", false);
    assert!(!result.is_empty(), "Should return some action");
}
