//! Integration tests for Lab 36: A Tree-Walking Interpreter
//!
//! These tests verify the full `interpret` pipeline, from string input
//! to final `f64` result or error.

use interpreter::solution::{interpret, InterpreterError};
use interpreter::solution::lexer::LexerError;
use interpreter::solution::parser::ParseError;
use interpreter::solution::evaluator::EvalError;

/// Helper to assert that an expression evaluates to the correct number.
/// Uses a small epsilon for float comparison.
fn assert_evals_to(expr: &str, expected: f64) {
    let result = interpret(expr).unwrap();
    assert!((result - expected).abs() < 1e-9, "Expected {}, got {}", expected, result);
}

/// Helper to assert that an expression returns a specific error.
fn assert_evals_to_err(expr: &str, expected_err: InterpreterError) {
    let err = interpret(expr).unwrap_err();
    assert_eq!(err, expected_err);
}

// ============================================================================
// BASIC ARITHMETIC
// ============================================================================

#[test]
fn test_addition() {
    assert_evals_to("1 + 2", 3.0);
}

#[test]
fn test_subtraction() {
    assert_evals_to("10 - 3", 7.0);
}

#[test]
fn test_multiplication() {
    assert_evals_to("4 * 5", 20.0);
}

#[test]
fn test_division() {
    assert_evals_to("20 / 4", 5.0);
}

#[test]
fn test_decimal_numbers() {
    assert_evals_to("1.5 + 2.5", 4.0);
    assert_evals_to("0.5 * 3.0", 1.5);
}

// ============================================================================
// OPERATOR PRECEDENCE
// ============================================================================

#[test]
fn test_multiplication_before_addition() {
    assert_evals_to("2 + 3 * 4", 14.0);
}

#[test]
fn test_multiplication_before_subtraction() {
    assert_evals_to("10 - 2 * 3", 4.0);
}

#[test]
fn test_division_before_addition() {
    assert_evals_to("10 + 20 / 2", 20.0);
}

#[test]
fn test_left_associativity() {
    assert_evals_to("10 - 3 - 2", 5.0); // (10 - 3) - 2
    assert_evals_to("20 / 4 / 2", 2.5); // (20 / 4) / 2
}

// ============================================================================
// PARENTHESES / GROUPING
// ============================================================================

#[test]
fn test_parentheses_override_precedence() {
    assert_evals_to("(2 + 3) * 4", 20.0);
}

#[test]
fn test_nested_parentheses() {
    assert_evals_to("10 * (2 + (3 - 1))", 40.0);
}

#[test]
fn test_complex_expression() {
    assert_evals_to(" ( 1 + 1 ) * ( 10 / ( 2 + 3 ) ) ", 4.0); // 2 * (10 / 5) = 2 * 2
}

// ============================================================================
// UNARY OPERATORS
// ============================================================================

#[test]
fn test_unary_negation() {
    assert_evals_to("-5", -5.0);
    assert_evals_to("- (10 + 5)", -15.0);
    assert_evals_to("3 + -5", -2.0);
}

// ============================================================================
// ERROR HANDLING
// ============================================================================

#[test]
fn test_lexer_error_unexpected_character() {
    assert_evals_to_err("1 + #", InterpreterError::Lexer(LexerError::UnexpectedCharacter('#')));
}

#[test]
fn test_parser_error_unexpected_end() {
    assert_evals_to_err("1 +", InterpreterError::Parser(ParseError::UnexpectedEndOfInput));
}

#[test]
fn test_parser_error_missing_right_paren() {
    assert_evals_to_err("(1 + 2", InterpreterError::Parser(ParseError::ExpectedRightParen));
}

#[test]
fn test_evaluator_error_division_by_zero() {
    assert_evals_to_err("1 / 0", InterpreterError::Evaluator(EvalError::DivisionByZero));
}

#[test]
fn test_evaluator_error_division_by_zero_in_subexpression() {
    assert_evals_to_err("10 * (1 / (2 - 2))", InterpreterError::Evaluator(EvalError::DivisionByZero));
}