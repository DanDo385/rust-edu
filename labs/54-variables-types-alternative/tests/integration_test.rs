//! Integration tests for Lab 54: Variables & Types (Alternative)
//!
//! These tests verify basic arithmetic, type conversions, boolean operations,
//! tuple operations, and shadowing/parsing behavior.

use variables_types_alternative::*;

// ============================================================================
// ARITHMETIC TESTS
// ============================================================================

#[test]
fn test_add_positive_numbers() {
    assert_eq!(add_i32(2, 3), 5);
}

#[test]
fn test_add_negative_numbers() {
    assert_eq!(add_i32(-2, -3), -5);
}

#[test]
fn test_add_mixed_sign() {
    assert_eq!(add_i32(-10, 15), 5);
}

#[test]
fn test_add_zero() {
    assert_eq!(add_i32(0, 0), 0);
    assert_eq!(add_i32(42, 0), 42);
}

#[test]
fn test_subtract_basic() {
    assert_eq!(subtract_i32(10, 3), 7);
}

#[test]
fn test_subtract_negative_result() {
    assert_eq!(subtract_i32(3, 10), -7);
}

#[test]
fn test_multiply_basic() {
    assert_eq!(multiply_i32(4, 5), 20);
}

#[test]
fn test_multiply_by_zero() {
    assert_eq!(multiply_i32(999, 0), 0);
}

#[test]
fn test_multiply_negative() {
    assert_eq!(multiply_i32(-3, 4), -12);
    assert_eq!(multiply_i32(-3, -4), 12);
}

// ============================================================================
// BOOLEAN & COMPARISON TESTS
// ============================================================================

#[test]
fn test_is_positive_true() {
    assert!(is_positive(1));
    assert!(is_positive(100));
    assert!(is_positive(i32::MAX));
}

#[test]
fn test_is_positive_false() {
    assert!(!is_positive(0));
    assert!(!is_positive(-1));
    assert!(!is_positive(i32::MIN));
}

#[test]
fn test_is_even() {
    assert!(is_even(0));
    assert!(is_even(2));
    assert!(is_even(-4));
    assert!(is_even(100));
}

#[test]
fn test_is_odd() {
    assert!(!is_even(1));
    assert!(!is_even(-3));
    assert!(!is_even(99));
}

#[test]
fn test_absolute_value_positive() {
    assert_eq!(absolute_value(5), 5);
}

#[test]
fn test_absolute_value_negative() {
    assert_eq!(absolute_value(-5), 5);
}

#[test]
fn test_absolute_value_zero() {
    assert_eq!(absolute_value(0), 0);
}

// ============================================================================
// CHARACTER & TYPE CONVERSION TESTS
// ============================================================================

#[test]
fn test_char_to_u32_ascii_letters() {
    assert_eq!(char_to_u32('A'), 65);
    assert_eq!(char_to_u32('Z'), 90);
    assert_eq!(char_to_u32('a'), 97);
    assert_eq!(char_to_u32('z'), 122);
}

#[test]
fn test_char_to_u32_digits() {
    assert_eq!(char_to_u32('0'), 48);
    assert_eq!(char_to_u32('9'), 57);
}

#[test]
fn test_char_to_u32_unicode() {
    // Unicode characters have code points > 127
    assert!(char_to_u32('中') > 127);
    assert!(char_to_u32('😎') > 127);
}

#[test]
fn test_u8_to_char_ascii() {
    assert_eq!(u8_to_char(65), Some('A'));
    assert_eq!(u8_to_char(48), Some('0'));
    assert_eq!(u8_to_char(32), Some(' '));
}

#[test]
fn test_i32_to_f64_conversion() {
    assert_eq!(i32_to_f64(42), 42.0);
    assert_eq!(i32_to_f64(-10), -10.0);
    assert_eq!(i32_to_f64(0), 0.0);
}

#[test]
fn test_f64_to_i32_truncation() {
    assert_eq!(f64_to_i32(3.14), 3);
    assert_eq!(f64_to_i32(3.99), 3);
    assert_eq!(f64_to_i32(-2.7), -2);
}

// ============================================================================
// TUPLE OPERATION TESTS
// ============================================================================

#[test]
fn test_tuple_sum_positive() {
    assert_eq!(tuple_sum((1, 2, 3)), 6);
}

#[test]
fn test_tuple_sum_mixed() {
    assert_eq!(tuple_sum((-1, 0, 1)), 0);
}

#[test]
fn test_tuple_sum_zeros() {
    assert_eq!(tuple_sum((0, 0, 0)), 0);
}

#[test]
fn test_tuple_sum_large_values() {
    assert_eq!(tuple_sum((100, 200, 300)), 600);
}

#[test]
fn test_tuple_min() {
    assert_eq!(tuple_min((3, 1, 2)), 1);
    assert_eq!(tuple_min((-5, 0, 5)), -5);
    assert_eq!(tuple_min((7, 7, 7)), 7);
}

#[test]
fn test_tuple_max() {
    assert_eq!(tuple_max((3, 1, 2)), 3);
    assert_eq!(tuple_max((-5, 0, 5)), 5);
    assert_eq!(tuple_max((7, 7, 7)), 7);
}

#[test]
fn test_swap_pair() {
    assert_eq!(swap_pair((1, 2)), (2, 1));
    assert_eq!(swap_pair((0, 0)), (0, 0));
    assert_eq!(swap_pair((-1, 1)), (1, -1));
}

// ============================================================================
// SHADOWING / PARSE TESTS
// ============================================================================

#[test]
fn test_parse_and_double_valid() {
    assert_eq!(parse_and_double("5"), Some(10));
    assert_eq!(parse_and_double("0"), Some(0));
    assert_eq!(parse_and_double("-3"), Some(-6));
}

#[test]
fn test_parse_and_double_invalid() {
    assert_eq!(parse_and_double("abc"), None);
    assert_eq!(parse_and_double(""), None);
    assert_eq!(parse_and_double("3.14"), None);
}
