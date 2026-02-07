//! Integration tests for Lab 55: Ownership & Borrowing (Alternative)
//!
//! Tests verify borrowing behavior, string operations, move vs copy semantics,
//! and mutable borrowing patterns.

use ownership_borrowing_alternative::*;

// ============================================================================
// IMMUTABLE BORROWING TESTS
// ============================================================================

#[test]
fn test_string_length_basic() {
    assert_eq!(string_length("hello"), 5);
}

#[test]
fn test_string_length_empty() {
    assert_eq!(string_length(""), 0);
}

#[test]
fn test_string_length_unicode() {
    // Each Chinese character is 3 bytes in UTF-8
    assert_eq!(string_length("中文"), 6);
}

#[test]
fn test_string_length_preserves_original() {
    // Demonstrates that borrowing does NOT consume the original
    let s = String::from("hello");
    let len = string_length(&s);
    assert_eq!(len, 5);
    // `s` is still usable after borrowing
    assert_eq!(s, "hello");
}

#[test]
fn test_first_word_single_word() {
    assert_eq!(first_word("hello"), "hello");
}

#[test]
fn test_first_word_multiple_words() {
    assert_eq!(first_word("hello world foo"), "hello");
}

#[test]
fn test_first_word_empty() {
    assert_eq!(first_word(""), "");
}

#[test]
fn test_first_word_leading_space() {
    assert_eq!(first_word(" hello"), "");
}

#[test]
fn test_last_word_single() {
    assert_eq!(last_word("hello"), "hello");
}

#[test]
fn test_last_word_multiple() {
    assert_eq!(last_word("hello beautiful world"), "world");
}

#[test]
fn test_last_word_empty() {
    assert_eq!(last_word(""), "");
}

#[test]
fn test_word_count_multiple() {
    assert_eq!(word_count("hello world foo"), 3);
}

#[test]
fn test_word_count_single() {
    assert_eq!(word_count("hello"), 1);
}

#[test]
fn test_word_count_empty() {
    assert_eq!(word_count(""), 0);
}

#[test]
fn test_word_count_whitespace_only() {
    assert_eq!(word_count("   "), 0);
}

#[test]
fn test_word_count_extra_spaces() {
    // split_whitespace handles multiple spaces
    assert_eq!(word_count("hello   world"), 2);
}

// ============================================================================
// OWNERSHIP / NEW STRING TESTS
// ============================================================================

#[test]
fn test_concat_strings_basic() {
    assert_eq!(concat_strings("hello", " world"), "hello world");
}

#[test]
fn test_concat_strings_empty_left() {
    assert_eq!(concat_strings("", "world"), "world");
}

#[test]
fn test_concat_strings_empty_right() {
    assert_eq!(concat_strings("hello", ""), "hello");
}

#[test]
fn test_concat_strings_both_empty() {
    assert_eq!(concat_strings("", ""), "");
}

#[test]
fn test_concat_preserves_originals() {
    let a = "hello";
    let b = " world";
    let result = concat_strings(a, b);
    // Original slices are still valid (they were borrowed, not moved)
    assert_eq!(result, "hello world");
    assert_eq!(a, "hello");
    assert_eq!(b, " world");
}

#[test]
fn test_clone_and_modify_basic() {
    assert_eq!(clone_and_modify("hello"), "HELLO");
}

#[test]
fn test_clone_and_modify_mixed_case() {
    assert_eq!(clone_and_modify("Hello World"), "HELLO WORLD");
}

#[test]
fn test_clone_and_modify_already_upper() {
    assert_eq!(clone_and_modify("RUST"), "RUST");
}

#[test]
fn test_clone_and_modify_empty() {
    assert_eq!(clone_and_modify(""), "");
}

#[test]
fn test_reverse_string_basic() {
    assert_eq!(reverse_string("hello"), "olleh");
}

#[test]
fn test_reverse_string_palindrome() {
    assert_eq!(reverse_string("racecar"), "racecar");
}

#[test]
fn test_reverse_string_empty() {
    assert_eq!(reverse_string(""), "");
}

#[test]
fn test_reverse_string_single_char() {
    assert_eq!(reverse_string("a"), "a");
}

#[test]
fn test_repeat_with_separator_basic() {
    assert_eq!(repeat_with_separator("ha", 3, "-"), "ha-ha-ha");
}

#[test]
fn test_repeat_with_separator_once() {
    assert_eq!(repeat_with_separator("hello", 1, ","), "hello");
}

#[test]
fn test_repeat_with_separator_zero() {
    assert_eq!(repeat_with_separator("hello", 0, ","), "");
}

#[test]
fn test_repeat_with_separator_empty_sep() {
    assert_eq!(repeat_with_separator("ab", 3, ""), "ababab");
}

// ============================================================================
// MUTABLE BORROWING TESTS
// ============================================================================

#[test]
fn test_append_suffix_basic() {
    let mut s = String::from("hello");
    append_suffix(&mut s, " world");
    assert_eq!(s, "hello world");
}

#[test]
fn test_append_suffix_empty() {
    let mut s = String::from("hello");
    append_suffix(&mut s, "");
    assert_eq!(s, "hello");
}

#[test]
fn test_append_suffix_multiple() {
    let mut s = String::from("a");
    append_suffix(&mut s, "b");
    append_suffix(&mut s, "c");
    assert_eq!(s, "abc");
}

#[test]
fn test_truncate_string_shorter() {
    assert_eq!(truncate_string("hello world", 5), "hello");
}

#[test]
fn test_truncate_string_exact() {
    assert_eq!(truncate_string("hello", 5), "hello");
}

#[test]
fn test_truncate_string_longer() {
    assert_eq!(truncate_string("hi", 10), "hi");
}

#[test]
fn test_truncate_string_zero() {
    assert_eq!(truncate_string("hello", 0), "");
}

// ============================================================================
// COPY vs MOVE TESTS
// ============================================================================

#[test]
fn test_consume_and_measure() {
    let s = String::from("hello");
    let len = consume_and_measure(s);
    assert_eq!(len, 5);
    // `s` is no longer usable here -- it was moved
}

#[test]
fn test_consume_and_measure_empty() {
    let s = String::from("");
    assert_eq!(consume_and_measure(s), 0);
}

#[test]
fn test_copy_type_demo() {
    let n = 10;
    let (a, b) = copy_type_demo(n);
    assert_eq!(a, 11);
    assert_eq!(b, 12);
    // `n` is still usable because i32 is Copy
    assert_eq!(n, 10);
}

#[test]
fn test_copy_type_demo_negative() {
    let (a, b) = copy_type_demo(-5);
    assert_eq!(a, -4);
    assert_eq!(b, -3);
}
