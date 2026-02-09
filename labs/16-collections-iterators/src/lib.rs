//! # Lab 16: Collections and Iterators
//!
//! Student-facing API. Implement these with iterators, then compare with `solution.rs`.

/// Sum all even numbers from a borrowed slice.
pub fn sum_evens(numbers: &[i32]) -> i32 {
    // TODO: Filter for even values and sum them.
    let _ = numbers;
    todo!("Sum even numbers")
}

/// Count elements that satisfy a predicate closure.
pub fn count_matching<F>(items: &[i32], predicate: F) -> usize
where
    F: Fn(i32) -> bool,
{
    // TODO: Apply predicate to each element and count matches.
    let _ = (items, predicate);
    todo!("Count values matching predicate")
}

/// Return true only when every number is strictly positive.
pub fn all_positive(numbers: &[i32]) -> bool {
    // TODO: Use an iterator method that short-circuits on false.
    let _ = numbers;
    todo!("Check positivity for all elements")
}

/// Group consecutive equal numbers into sub-vectors.
pub fn group_consecutive(numbers: &[i32]) -> Vec<Vec<i32>> {
    // TODO: Build groups preserving order and boundaries.
    let _ = numbers;
    todo!("Group consecutive equal values")
}

/// Return the first even value, if one exists.
pub fn find_first_even(numbers: &[i32]) -> Option<i32> {
    // TODO: Find first even and return owned i32.
    let _ = numbers;
    todo!("Find first even element")
}

/// Convert each number to an owned String.
pub fn to_strings(numbers: &[i32]) -> Vec<String> {
    // TODO: Map each i32 into String and collect.
    let _ = numbers;
    todo!("Convert numbers to strings")
}

/// Return maximum element, or None for empty input.
pub fn find_max(numbers: &[i32]) -> Option<i32> {
    // TODO: Return max as owned value.
    let _ = numbers;
    todo!("Find max value")
}

#[doc(hidden)]
pub mod solution;
