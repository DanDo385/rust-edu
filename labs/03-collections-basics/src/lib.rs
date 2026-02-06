//! # Collections Basics - Vec and HashMap
//!
//! Learn Rust's collection types and iteration patterns.

use std::collections::HashMap;

/// Sum all even numbers in a vector.
pub fn sum_of_evens(numbers: &[i32]) -> i32 {
    todo!("Sum all even numbers")
}

/// Count frequency of each word in the text.
pub fn word_frequency(text: &str) -> HashMap<String, usize> {
    todo!("Count word frequencies")
}

/// Filter numbers in range [min, max] and return sorted.
pub fn filter_and_sort(numbers: &[i32], min: i32, max: i32) -> Vec<i32> {
    todo!("Filter and sort numbers")
}

/// Find the most common word, or None if empty.
pub fn most_common_word(text: &str) -> Option<String> {
    todo!("Find most common word")
}

#[doc(hidden)]
pub mod solution;
