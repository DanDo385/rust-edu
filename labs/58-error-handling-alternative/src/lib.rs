//! # Lab 58: Error Handling (Alternative)
//!
//! Alternative implementation demonstrating Rust's error handling with
//! Result<T, E>, Option<T>, custom error types, the ? operator, and
//! error propagation patterns.
//!
//! ## Ownership Commentary
//! - `Result<T, E>` is an enum that either owns a success value (Ok(T)) or an error (Err(E))
//! - The ? operator propagates errors by returning early with Err
//! - Custom error enums let you define domain-specific error types
//! - `Option<T>` handles the absence of a value without null pointers

use std::fmt;
use std::num::ParseIntError;

// ============================================================================
// OPTION<T>: Handling optional values
// ============================================================================

/// Divides two f64 numbers, returning None if the divisor is zero.
///
/// # Memory Model
/// Option<f64> is 16 bytes on the stack: 8 bytes for the f64 + 8 bytes for
/// the discriminant (whether it's Some or None). No heap allocation.
pub fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

/// Returns the element at the given index, or None if out of bounds.
pub fn safe_get(slice: &[i32], index: usize) -> Option<i32> {
    if index < slice.len() {
        Some(slice[index])
    } else {
        None
    }
}

/// Finds the first even number in a slice, or None if there are no even numbers.
pub fn first_even(numbers: &[i32]) -> Option<i32> {
    numbers.iter().find(|&&n| n % 2 == 0).copied()
}

// ============================================================================
// CUSTOM ERROR TYPE
// ============================================================================

/// A custom error type for mathematical operations.
///
/// # Teaching Note
/// Custom error enums are the idiomatic way to handle domain errors in Rust.
/// Each variant describes a specific failure mode. Implementing Display and
/// std::error::Error makes them composable with other error types.
#[derive(Debug, Clone, PartialEq)]
pub enum MathError {
    DivisionByZero,
    NegativeSquareRoot,
    Overflow,
}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MathError::DivisionByZero => write!(f, "division by zero"),
            MathError::NegativeSquareRoot => write!(f, "cannot take square root of negative number"),
            MathError::Overflow => write!(f, "arithmetic overflow"),
        }
    }
}

impl std::error::Error for MathError {}

// ============================================================================
// RESULT<T, E>: Recoverable errors
// ============================================================================

/// Divides two f64 numbers, returning an error if the divisor is zero.
///
/// # Memory Model
/// Result<f64, MathError> is stored on the stack. The discriminant tells
/// whether it's Ok or Err. No heap allocation for either variant.
pub fn safe_divide(a: f64, b: f64) -> Result<f64, MathError> {
    if b == 0.0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

/// Computes the square root, returning an error for negative inputs.
pub fn safe_sqrt(x: f64) -> Result<f64, MathError> {
    if x < 0.0 {
        Err(MathError::NegativeSquareRoot)
    } else {
        Ok(x.sqrt())
    }
}

/// Adds two i32 values, returning an error on overflow.
pub fn safe_add(a: i32, b: i32) -> Result<i32, MathError> {
    a.checked_add(b).ok_or(MathError::Overflow)
}

/// Multiplies two i32 values, returning an error on overflow.
pub fn safe_multiply(a: i32, b: i32) -> Result<i32, MathError> {
    a.checked_mul(b).ok_or(MathError::Overflow)
}

// ============================================================================
// ERROR PROPAGATION WITH ?
// ============================================================================

/// Computes (a / b) + sqrt(c) using the ? operator for error propagation.
///
/// # Teaching Note
/// The ? operator is syntactic sugar for early return on Err.
/// `safe_divide(a, b)?` is equivalent to:
/// ```ignore
/// match safe_divide(a, b) {
///     Ok(val) => val,
///     Err(e) => return Err(e),
/// }
/// ```
pub fn complex_calculation(a: f64, b: f64, c: f64) -> Result<f64, MathError> {
    let div_result = safe_divide(a, b)?;
    let sqrt_result = safe_sqrt(c)?;
    Ok(div_result + sqrt_result)
}

// ============================================================================
// PARSE ERROR HANDLING
// ============================================================================

/// A parse error that wraps std's ParseIntError.
#[derive(Debug)]
pub enum ParseError {
    InvalidNumber(ParseIntError),
    NumberTooLarge,
    NegativeNumber,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidNumber(e) => write!(f, "invalid number: {}", e),
            ParseError::NumberTooLarge => write!(f, "number too large (max 1000)"),
            ParseError::NegativeNumber => write!(f, "negative numbers not allowed"),
        }
    }
}

impl From<ParseIntError> for ParseError {
    fn from(e: ParseIntError) -> Self {
        ParseError::InvalidNumber(e)
    }
}

/// Parses a string into an i32 and validates it is a positive number <= 1000.
///
/// # Teaching Note
/// The `From` trait implementation above allows ? to automatically convert
/// ParseIntError into our ParseError type.
pub fn parse_positive_bounded(s: &str) -> Result<i32, ParseError> {
    let n: i32 = s.parse()?; // ? converts ParseIntError to ParseError via From
    if n < 0 {
        return Err(ParseError::NegativeNumber);
    }
    if n > 1000 {
        return Err(ParseError::NumberTooLarge);
    }
    Ok(n)
}

/// Parses a string to i32 and doubles it. Returns Err if parsing fails.
pub fn parse_and_double(s: &str) -> Result<i32, ParseIntError> {
    let n: i32 = s.parse()?;
    Ok(n * 2)
}

// ============================================================================
// OPTION/RESULT COMBINATORS
// ============================================================================

/// Demonstrates using `unwrap_or` to provide a default value.
pub fn divide_or_default(a: f64, b: f64, default: f64) -> f64 {
    divide(a, b).unwrap_or(default)
}

/// Demonstrates using `map` to transform a Result value.
pub fn divide_and_round(a: f64, b: f64) -> Result<i64, MathError> {
    safe_divide(a, b).map(|result| result.round() as i64)
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide_ok() {
        assert_eq!(divide(10.0, 2.0), Some(5.0));
    }

    #[test]
    fn test_divide_by_zero() {
        assert_eq!(divide(10.0, 0.0), None);
    }

    #[test]
    fn test_safe_divide_ok() {
        assert_eq!(safe_divide(10.0, 2.0), Ok(5.0));
    }

    #[test]
    fn test_safe_divide_error() {
        assert_eq!(safe_divide(10.0, 0.0), Err(MathError::DivisionByZero));
    }
}
