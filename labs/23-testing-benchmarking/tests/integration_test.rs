use testing_benchmarking::*;

// ============================================================================
// TESTS: BASIC MATH FUNCTIONS
// ============================================================================

#[test]
fn test_add_positive_numbers() {
    assert_eq!(add(2, 3), 5);
    assert_eq!(add(100, 200), 300);
}

#[test]
fn test_add_negative_numbers() {
    assert_eq!(add(-1, -1), -2);
    assert_eq!(add(-5, -10), -15);
}

#[test]
fn test_add_mixed_signs() {
    assert_eq!(add(-1, 1), 0);
    assert_eq!(add(5, -3), 2);
}

#[test]
fn test_add_zero() {
    assert_eq!(add(0, 0), 0);
    assert_eq!(add(5, 0), 5);
    assert_eq!(add(0, 5), 5);
}

#[test]
fn test_subtract_basic() {
    assert_eq!(subtract(10, 3), 7);
    assert_eq!(subtract(5, 10), -5);
    assert_eq!(subtract(0, 0), 0);
}

#[test]
fn test_multiply_basic() {
    assert_eq!(multiply(3, 4), 12);
    assert_eq!(multiply(-2, 5), -10);
    assert_eq!(multiply(0, 100), 0);
}

#[test]
fn test_multiply_negative_by_negative() {
    assert_eq!(multiply(-3, -4), 12);
}

// ============================================================================
// TESTS: DIVIDE WITH RESULT
// ============================================================================

#[test]
fn test_divide_success() {
    assert_eq!(divide(10, 2).unwrap(), 5);
    assert_eq!(divide(100, 4).unwrap(), 25);
    assert_eq!(divide(-10, 2).unwrap(), -5);
}

#[test]
fn test_divide_by_zero_returns_error() {
    assert!(divide(10, 0).is_err());
}

#[test]
fn test_divide_error_message() {
    match divide(10, 0) {
        Err(msg) => assert!(msg.contains("zero")),
        Ok(_) => panic!("Expected error for division by zero"),
    }
}

#[test]
fn test_divide_integer_truncation() {
    assert_eq!(divide(7, 2).unwrap(), 3);
    assert_eq!(divide(1, 3).unwrap(), 0);
}

// ============================================================================
// TESTS: BOOLEAN FUNCTIONS
// ============================================================================

#[test]
fn test_is_even_true_cases() {
    assert!(is_even(0));
    assert!(is_even(2));
    assert!(is_even(100));
    assert!(is_even(-4));
    assert!(is_even(-100));
}

#[test]
fn test_is_even_false_cases() {
    assert!(!is_even(1));
    assert!(!is_even(3));
    assert!(!is_even(-7));
    assert!(!is_even(99));
}

#[test]
fn test_is_prime_small_primes() {
    assert!(is_prime(2));
    assert!(is_prime(3));
    assert!(is_prime(5));
    assert!(is_prime(7));
    assert!(is_prime(11));
    assert!(is_prime(13));
}

#[test]
fn test_is_prime_non_primes() {
    assert!(!is_prime(0));
    assert!(!is_prime(1));
    assert!(!is_prime(4));
    assert!(!is_prime(6));
    assert!(!is_prime(8));
    assert!(!is_prime(9));
    assert!(!is_prime(100));
}

#[test]
fn test_is_prime_larger_primes() {
    assert!(is_prime(97));
    assert!(is_prime(101));
    assert!(is_prime(1009));
}

// ============================================================================
// TESTS: FIBONACCI
// ============================================================================

#[test]
fn test_fibonacci_base_cases() {
    assert_eq!(fibonacci(0), 0);
    assert_eq!(fibonacci(1), 1);
}

#[test]
fn test_fibonacci_sequence() {
    assert_eq!(fibonacci(2), 1);
    assert_eq!(fibonacci(3), 2);
    assert_eq!(fibonacci(4), 3);
    assert_eq!(fibonacci(5), 5);
    assert_eq!(fibonacci(6), 8);
    assert_eq!(fibonacci(7), 13);
}

#[test]
fn test_fibonacci_larger_values() {
    assert_eq!(fibonacci(10), 55);
    assert_eq!(fibonacci(20), 6765);
    assert_eq!(fibonacci(30), 832040);
}

// ============================================================================
// TESTS: CALCULATOR STRUCT
// ============================================================================

#[test]
fn test_calculator_creation() {
    let calc = Calculator::new(10);
    assert_eq!(calc.value, 10);
}

#[test]
fn test_calculator_creation_negative() {
    let calc = Calculator::new(-5);
    assert_eq!(calc.value, -5);
}

#[test]
fn test_calculator_add() {
    let mut calc = Calculator::new(10);
    calc.add(5);
    assert_eq!(calc.value, 15);
}

#[test]
fn test_calculator_subtract() {
    let mut calc = Calculator::new(20);
    calc.subtract(5);
    assert_eq!(calc.value, 15);
}

#[test]
fn test_calculator_multiply() {
    let mut calc = Calculator::new(5);
    calc.multiply(3);
    assert_eq!(calc.value, 15);
}

#[test]
fn test_calculator_multiply_by_zero() {
    let mut calc = Calculator::new(100);
    calc.multiply(0);
    assert_eq!(calc.value, 0);
}

#[test]
fn test_calculator_reset() {
    let mut calc = Calculator::new(100);
    calc.add(50);
    calc.reset();
    assert_eq!(calc.value, 0);
}

#[test]
fn test_calculator_chained_operations() {
    let mut calc = Calculator::new(10);
    calc.add(5);       // 15
    calc.multiply(2);  // 30
    calc.subtract(10); // 20
    assert_eq!(calc.value, 20);
}

#[test]
fn test_calculator_negative_result() {
    let mut calc = Calculator::new(5);
    calc.subtract(10);
    assert_eq!(calc.value, -5);
}

// ============================================================================
// PROPERTY-BASED TESTS
// ============================================================================

#[test]
fn test_add_commutative() {
    // a + b == b + a
    assert_eq!(add(5, 3), add(3, 5));
    assert_eq!(add(-2, 7), add(7, -2));
}

#[test]
fn test_add_associative() {
    // (a + b) + c == a + (b + c)
    assert_eq!(add(add(1, 2), 3), add(1, add(2, 3)));
}

#[test]
fn test_multiply_commutative() {
    assert_eq!(multiply(3, 7), multiply(7, 3));
    assert_eq!(multiply(-2, 5), multiply(5, -2));
}

#[test]
fn test_add_identity() {
    // a + 0 == a
    assert_eq!(add(42, 0), 42);
    assert_eq!(add(0, 42), 42);
}

#[test]
fn test_multiply_identity() {
    // a * 1 == a
    assert_eq!(multiply(42, 1), 42);
    assert_eq!(multiply(1, 42), 42);
}
