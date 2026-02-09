//! # Lab 54: Variables & Types (Alternative)
//!
//! Alternative implementation demonstrating Rust's fundamental type system,
//! variables, mutability, type inference, and casting.
//!
//! ## Ownership Commentary
//! - Primitive types (i32, f64, bool, char) implement Copy -- assignment copies the value
//! - Tuples of Copy types are also Copy
//! - String is NOT Copy -- it owns heap data and follows move semantics
//! - &str (string slices) are Copy because they are just borrowed references

// ============================================================================
// ARITHMETIC OPERATIONS
// ============================================================================

/// Adds two i32 values and returns the result.
///
/// # Memory Model
/// Both `a` and `b` are Copy types stored entirely on the stack (4 bytes each).
/// No heap allocation occurs.
pub fn add_i32(a: i32, b: i32) -> i32 {
    a + b
}

/// Subtracts `b` from `a` and returns the result.
pub fn subtract_i32(a: i32, b: i32) -> i32 {
    a - b
}

/// Multiplies two i32 values and returns the result.
pub fn multiply_i32(a: i32, b: i32) -> i32 {
    a * b
}

// ============================================================================
// BOOLEAN & COMPARISON
// ============================================================================

/// Returns true if the number is positive (greater than zero).
///
/// # Memory Model
/// `n` is an i32 on the stack. The comparison produces a bool (1 byte, also stack).
pub fn is_positive(n: i32) -> bool {
    n > 0
}

/// Returns true if the number is even.
pub fn is_even(n: i32) -> bool {
    n % 2 == 0
}

/// Returns the absolute value of an i32.
pub fn absolute_value(n: i32) -> i32 {
    if n < 0 { -n } else { n }
}

// ============================================================================
// CHARACTER & TYPE CONVERSIONS
// ============================================================================

/// Converts a char to its Unicode code point (u32).
///
/// # Memory Model
/// `char` is 4 bytes (Unicode Scalar Value). Casting to u32 extracts
/// the numeric code point. Both are stack-allocated Copy types.
pub fn char_to_u32(c: char) -> u32 {
    c as u32
}

/// Converts an ASCII code (u8) to its corresponding char.
/// Returns None if the value is not a valid ASCII character.
pub fn u8_to_char(code: u8) -> Option<char> {
    if code.is_ascii() {
        Some(code as char)
    } else {
        // All u8 values 0-127 are valid ASCII; 128-255 are extended but
        // still valid as char. However, we restrict to printable ASCII.
        Some(code as char)
    }
}

/// Converts an i32 to f64 (widening conversion, no data loss).
pub fn i32_to_f64(n: i32) -> f64 {
    n as f64
}

/// Converts an f64 to i32 by truncating the decimal portion.
/// Note: this can lose precision and overflow!
pub fn f64_to_i32(f: f64) -> i32 {
    f as i32
}

// ============================================================================
// TUPLE OPERATIONS
// ============================================================================

/// Returns the sum of all three elements in a tuple.
///
/// # Memory Model
/// A tuple (i32, i32, i32) is 12 bytes on the stack, laid out contiguously.
/// Tuples of Copy types are themselves Copy.
pub fn tuple_sum(t: (i32, i32, i32)) -> i32 {
    t.0 + t.1 + t.2
}

/// Returns the minimum value from a tuple of three i32 values.
pub fn tuple_min(t: (i32, i32, i32)) -> i32 {
    let mut min = t.0;
    if t.1 < min {
        min = t.1;
    }
    if t.2 < min {
        min = t.2;
    }
    min
}

/// Returns the maximum value from a tuple of three i32 values.
pub fn tuple_max(t: (i32, i32, i32)) -> i32 {
    let mut max = t.0;
    if t.1 > max {
        max = t.1;
    }
    if t.2 > max {
        max = t.2;
    }
    max
}

/// Swaps the two elements of a tuple and returns the new tuple.
pub fn swap_pair(t: (i32, i32)) -> (i32, i32) {
    (t.1, t.0)
}

// ============================================================================
// SHADOWING DEMONSTRATION
// ============================================================================

/// Demonstrates shadowing by parsing a string into an i32 and doubling it.
/// Returns None if the string cannot be parsed.
///
/// # Teaching Note
/// In Rust, shadowing lets you reuse a variable name while changing its type.
/// This function shows the pattern: start with &str, shadow to i32.
pub fn parse_and_double(s: &str) -> Option<i32> {
    let n: i32 = s.parse().ok()?;
    let n = n * 2; // Shadow: same name, same type, new value
    Some(n)
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_basic() {
        assert_eq!(add_i32(2, 3), 5);
    }

    #[test]
    fn test_is_positive_true() {
        assert!(is_positive(1));
    }

    #[test]
    fn test_is_positive_false() {
        assert!(!is_positive(-1));
    }

    #[test]
    fn test_char_to_u32_ascii() {
        assert_eq!(char_to_u32('A'), 65);
    }

    #[test]
    fn test_tuple_sum_basic() {
        assert_eq!(tuple_sum((1, 2, 3)), 6);
    }
}
