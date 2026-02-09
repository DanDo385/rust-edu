//! Integration tests for Lab 58: Error Handling (Alternative)
//!
//! Tests verify Option/Result usage, custom error types, error propagation
//! with ?, parse error handling, and combinator patterns.

use error_handling_alternative::solution::*;

// ============================================================================
// OPTION TESTS (divide, safe_get, first_even)
// ============================================================================

#[test]
fn test_divide_normal() {
    assert_eq!(divide(10.0, 2.0), Some(5.0));
}

#[test]
fn test_divide_by_zero() {
    assert_eq!(divide(10.0, 0.0), None);
}

#[test]
fn test_divide_zero_numerator() {
    assert_eq!(divide(0.0, 5.0), Some(0.0));
}

#[test]
fn test_divide_negative_numbers() {
    let result = divide(-10.0, 2.0);
    assert_eq!(result, Some(-5.0));
}

#[test]
fn test_divide_fractional_result() {
    let result = divide(1.0, 3.0).unwrap();
    assert!((result - 0.333333).abs() < 0.001);
}

#[test]
fn test_safe_get_valid_index() {
    let data = [10, 20, 30];
    assert_eq!(safe_get(&data, 0), Some(10));
    assert_eq!(safe_get(&data, 2), Some(30));
}

#[test]
fn test_safe_get_out_of_bounds() {
    let data = [10, 20, 30];
    assert_eq!(safe_get(&data, 3), None);
    assert_eq!(safe_get(&data, 100), None);
}

#[test]
fn test_safe_get_empty_slice() {
    let data: [i32; 0] = [];
    assert_eq!(safe_get(&data, 0), None);
}

#[test]
fn test_first_even_found() {
    assert_eq!(first_even(&[1, 3, 4, 5]), Some(4));
}

#[test]
fn test_first_even_first_element() {
    assert_eq!(first_even(&[2, 3, 5, 7]), Some(2));
}

#[test]
fn test_first_even_none() {
    assert_eq!(first_even(&[1, 3, 5, 7]), None);
}

#[test]
fn test_first_even_empty() {
    assert_eq!(first_even(&[]), None);
}

#[test]
fn test_first_even_negative() {
    assert_eq!(first_even(&[-3, -2, -1]), Some(-2));
}

// ============================================================================
// RESULT TESTS (safe_divide, safe_sqrt, safe_add, safe_multiply)
// ============================================================================

#[test]
fn test_safe_divide_ok() {
    assert_eq!(safe_divide(10.0, 2.0), Ok(5.0));
}

#[test]
fn test_safe_divide_division_by_zero() {
    assert_eq!(safe_divide(10.0, 0.0), Err(MathError::DivisionByZero));
}

#[test]
fn test_safe_sqrt_positive() {
    let result = safe_sqrt(9.0).unwrap();
    assert!((result - 3.0).abs() < 1e-10);
}

#[test]
fn test_safe_sqrt_zero() {
    assert_eq!(safe_sqrt(0.0), Ok(0.0));
}

#[test]
fn test_safe_sqrt_negative() {
    assert_eq!(safe_sqrt(-1.0), Err(MathError::NegativeSquareRoot));
}

#[test]
fn test_safe_add_normal() {
    assert_eq!(safe_add(10, 20), Ok(30));
}

#[test]
fn test_safe_add_overflow() {
    assert_eq!(safe_add(i32::MAX, 1), Err(MathError::Overflow));
}

#[test]
fn test_safe_add_negative() {
    assert_eq!(safe_add(-10, -20), Ok(-30));
}

#[test]
fn test_safe_multiply_normal() {
    assert_eq!(safe_multiply(10, 20), Ok(200));
}

#[test]
fn test_safe_multiply_overflow() {
    assert_eq!(safe_multiply(i32::MAX, 2), Err(MathError::Overflow));
}

#[test]
fn test_safe_multiply_by_zero() {
    assert_eq!(safe_multiply(i32::MAX, 0), Ok(0));
}

// ============================================================================
// ERROR PROPAGATION TESTS (complex_calculation)
// ============================================================================

#[test]
fn test_complex_calculation_ok() {
    // (10 / 2) + sqrt(9) = 5 + 3 = 8
    let result = complex_calculation(10.0, 2.0, 9.0).unwrap();
    assert!((result - 8.0).abs() < 1e-10);
}

#[test]
fn test_complex_calculation_division_by_zero() {
    let result = complex_calculation(10.0, 0.0, 9.0);
    assert_eq!(result, Err(MathError::DivisionByZero));
}

#[test]
fn test_complex_calculation_negative_sqrt() {
    let result = complex_calculation(10.0, 2.0, -1.0);
    assert_eq!(result, Err(MathError::NegativeSquareRoot));
}

#[test]
fn test_complex_calculation_both_errors() {
    // Division by zero happens first, so that error is returned
    let result = complex_calculation(10.0, 0.0, -1.0);
    assert_eq!(result, Err(MathError::DivisionByZero));
}

// ============================================================================
// PARSE ERROR TESTS
// ============================================================================

#[test]
fn test_parse_positive_bounded_valid() {
    assert_eq!(parse_positive_bounded("42").unwrap(), 42);
}

#[test]
fn test_parse_positive_bounded_zero() {
    assert_eq!(parse_positive_bounded("0").unwrap(), 0);
}

#[test]
fn test_parse_positive_bounded_max() {
    assert_eq!(parse_positive_bounded("1000").unwrap(), 1000);
}

#[test]
fn test_parse_positive_bounded_too_large() {
    let result = parse_positive_bounded("1001");
    assert!(result.is_err());
    assert!(matches!(result, Err(ParseError::NumberTooLarge)));
}

#[test]
fn test_parse_positive_bounded_negative() {
    let result = parse_positive_bounded("-5");
    assert!(result.is_err());
    assert!(matches!(result, Err(ParseError::NegativeNumber)));
}

#[test]
fn test_parse_positive_bounded_invalid() {
    let result = parse_positive_bounded("abc");
    assert!(result.is_err());
    assert!(matches!(result, Err(ParseError::InvalidNumber(_))));
}

#[test]
fn test_parse_positive_bounded_empty() {
    let result = parse_positive_bounded("");
    assert!(result.is_err());
}

#[test]
fn test_parse_and_double_valid() {
    assert_eq!(parse_and_double("5"), Ok(10));
}

#[test]
fn test_parse_and_double_zero() {
    assert_eq!(parse_and_double("0"), Ok(0));
}

#[test]
fn test_parse_and_double_negative() {
    assert_eq!(parse_and_double("-3"), Ok(-6));
}

#[test]
fn test_parse_and_double_invalid() {
    assert!(parse_and_double("abc").is_err());
}

// ============================================================================
// COMBINATOR TESTS
// ============================================================================

#[test]
fn test_divide_or_default_normal() {
    assert_eq!(divide_or_default(10.0, 2.0, 0.0), 5.0);
}

#[test]
fn test_divide_or_default_fallback() {
    assert_eq!(divide_or_default(10.0, 0.0, -1.0), -1.0);
}

#[test]
fn test_divide_and_round_ok() {
    assert_eq!(divide_and_round(10.0, 3.0), Ok(3));
}

#[test]
fn test_divide_and_round_exact() {
    assert_eq!(divide_and_round(10.0, 2.0), Ok(5));
}

#[test]
fn test_divide_and_round_error() {
    assert_eq!(divide_and_round(10.0, 0.0), Err(MathError::DivisionByZero));
}

// ============================================================================
// ERROR DISPLAY TESTS
// ============================================================================

#[test]
fn test_math_error_display_division_by_zero() {
    let err = MathError::DivisionByZero;
    assert_eq!(format!("{}", err), "division by zero");
}

#[test]
fn test_math_error_display_negative_sqrt() {
    let err = MathError::NegativeSquareRoot;
    assert_eq!(
        format!("{}", err),
        "cannot take square root of negative number"
    );
}

#[test]
fn test_math_error_display_overflow() {
    let err = MathError::Overflow;
    assert_eq!(format!("{}", err), "arithmetic overflow");
}

#[test]
fn test_parse_error_display_invalid() {
    let result = parse_positive_bounded("abc");
    if let Err(e) = result {
        let msg = format!("{}", e);
        assert!(msg.starts_with("invalid number:"));
    } else {
        panic!("expected error");
    }
}

#[test]
fn test_parse_error_display_too_large() {
    let result = parse_positive_bounded("9999");
    if let Err(e) = result {
        assert_eq!(format!("{}", e), "number too large (max 1000)");
    } else {
        panic!("expected error");
    }
}

#[test]
fn test_parse_error_display_negative() {
    let result = parse_positive_bounded("-1");
    if let Err(e) = result {
        assert_eq!(format!("{}", e), "negative numbers not allowed");
    } else {
        panic!("expected error");
    }
}
