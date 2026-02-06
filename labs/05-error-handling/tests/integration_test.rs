//! Integration tests for error-handling

use error_handling::solution::*;

#[test]
fn test_parse_number_valid() {
    assert_eq!(parse_number("42").unwrap(), 42);
    assert_eq!(parse_number(" 123 ").unwrap(), 123);
    assert_eq!(parse_number("-50").unwrap(), -50);
}

#[test]
fn test_parse_number_invalid() {
    assert!(parse_number("not a number").is_err());
    assert!(parse_number("12.34").is_err());
}

#[test]
fn test_divide_valid() {
    assert_eq!(divide(10.0, 2.0).unwrap(), 5.0);
    assert_eq!(divide(7.0, 2.0).unwrap(), 3.5);
}

#[test]
fn test_divide_by_zero() {
    assert!(divide(10.0, 0.0).is_err());
}

#[test]
fn test_validate_email_valid() {
    assert!(validate_email("user@example.com"));
    assert!(validate_email("test.user@domain.co.uk"));
}

#[test]
fn test_validate_email_invalid() {
    assert!(!validate_email("notemail"));
    assert!(!validate_email("@example.com"));
    assert!(!validate_email("user@"));
}
