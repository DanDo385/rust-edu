//! # Lab 55: Ownership & Borrowing (Alternative)
//!
//! Alternative implementation demonstrating Rust's ownership system, borrowing,
//! references, and string operations. Focuses on practical functions that show
//! how borrowing enables safe, zero-cost abstractions.
//!
//! ## Ownership Commentary
//! - `&str` borrows string data without taking ownership (immutable reference)
//! - `&mut String` borrows a String mutably, allowing modification
//! - Returning `String` transfers ownership to the caller
//! - `.to_string()` and `String::from()` allocate new heap memory

// ============================================================================
// BORROWING: IMMUTABLE REFERENCES
// ============================================================================

/// Returns the length of a string slice without taking ownership.
///
/// # Memory Model
/// `s: &str` is a fat pointer (pointer + length) on the stack -- 16 bytes.
/// It borrows the string data, so the caller retains ownership.
/// No heap allocation occurs in this function.
pub fn string_length(s: &str) -> usize {
    s.len()
}

/// Returns the first word in a string (up to the first space).
/// If there are no spaces, returns the entire string.
///
/// # Memory Model
/// The returned `&str` borrows from the input -- it is a slice into
/// the same underlying data. No new allocation occurs.
/// The lifetime of the return value is tied to the input `s`.
pub fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }
    s
}

/// Returns the last word in a string (after the last space).
/// If there are no spaces, returns the entire string.
pub fn last_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate().rev() {
        if byte == b' ' {
            return &s[i + 1..];
        }
    }
    s
}

/// Counts the number of words in a string (split by whitespace).
pub fn word_count(s: &str) -> usize {
    if s.trim().is_empty() {
        return 0;
    }
    s.split_whitespace().count()
}

// ============================================================================
// OWNERSHIP: CREATING NEW STRINGS
// ============================================================================

/// Concatenates two string slices into a new owned String.
///
/// # Memory Model
/// This function allocates a NEW String on the heap that contains the
/// combined content of `a` and `b`. The caller takes ownership of the result.
/// Neither `a` nor `b` is consumed -- they are borrowed via &str.
pub fn concat_strings(a: &str, b: &str) -> String {
    let mut result = String::with_capacity(a.len() + b.len());
    result.push_str(a);
    result.push_str(b);
    result
}

/// Clones a string slice and converts it to uppercase.
///
/// # Memory Model
/// `s` is borrowed (not consumed). A new String is allocated on the heap
/// with the uppercase version. The caller owns the returned String.
pub fn clone_and_modify(s: &str) -> String {
    s.to_uppercase()
}

/// Reverses a string and returns a new owned String.
///
/// # Teaching Note
/// This creates a new allocation because reversing in-place on a &str
/// is not possible (it's immutable). The reversed chars are collected
/// into a new heap-allocated String.
pub fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

/// Repeats a string `n` times with a separator between repetitions.
pub fn repeat_with_separator(s: &str, n: usize, sep: &str) -> String {
    if n == 0 {
        return String::new();
    }
    let parts: Vec<&str> = std::iter::repeat(s).take(n).collect();
    parts.join(sep)
}

// ============================================================================
// MUTABLE BORROWING
// ============================================================================

/// Appends a suffix to a mutable String reference.
///
/// # Memory Model
/// `s: &mut String` is a mutable reference to a String.
/// The String may reallocate its internal buffer if capacity is exceeded.
/// Only ONE mutable reference can exist at a time (borrow checker enforces this).
pub fn append_suffix(s: &mut String, suffix: &str) {
    s.push_str(suffix);
}

/// Truncates a string to the given maximum length (by character count).
/// Returns a new String (does not modify the original).
pub fn truncate_string(s: &str, max_len: usize) -> String {
    s.chars().take(max_len).collect()
}

// ============================================================================
// COPY vs MOVE DEMONSTRATION
// ============================================================================

/// Takes ownership of a String and returns its length.
/// After calling this, the original String is consumed (moved).
///
/// # Teaching Note
/// This function demonstrates MOVE semantics. The caller cannot use
/// the String after passing it here because ownership transfers.
pub fn consume_and_measure(s: String) -> usize {
    s.len()
    // `s` is dropped here -- heap memory is freed
}

/// Demonstrates that Copy types (like i32) can be used after assignment.
/// Returns a tuple of (original + 1, original + 2).
pub fn copy_type_demo(n: i32) -> (i32, i32) {
    let a = n + 1; // `n` is copied, not moved
    let b = n + 2; // `n` is still valid because i32 is Copy
    (a, b)
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_length() {
        assert_eq!(string_length("hello"), 5);
    }

    #[test]
    fn test_first_word() {
        assert_eq!(first_word("hello world"), "hello");
    }

    #[test]
    fn test_concat() {
        assert_eq!(concat_strings("hello", " world"), "hello world");
    }

    #[test]
    fn test_clone_and_modify() {
        assert_eq!(clone_and_modify("hello"), "HELLO");
    }
}
