// Project 20: Testing and Benchmarking
//
// This library demonstrates Rust's comprehensive testing capabilities.
// Tests are first-class citizens in Rust - they're built into the language
// and cargo build system.
//
// Run tests with: cargo test
// Run benchmarks with: cargo bench (requires criterion in Cargo.toml)

// ============================================================================
// LIBRARY CODE
// ============================================================================
// This is the actual code we want to test. In a real project, this would be
// your application logic.

/// Adds two numbers together.
///
/// # Examples
///
/// ```
/// use testing_benchmarking::add;
/// assert_eq!(add(2, 3), 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Subtracts the second number from the first.
///
/// # Examples
///
/// ```
/// use testing_benchmarking::subtract;
/// assert_eq!(subtract(10, 3), 7);
/// ```
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

/// Multiplies two numbers.
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

/// Divides two numbers, returning an error if dividing by zero.
///
/// # Examples
///
/// ```
/// use testing_benchmarking::divide;
/// assert_eq!(divide(10, 2).unwrap(), 5);
/// assert!(divide(10, 0).is_err());
/// ```
pub fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}

/// Checks if a number is even.
pub fn is_even(n: i32) -> bool {
    n % 2 == 0
}

/// Checks if a number is prime.
pub fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    let limit = (n as f64).sqrt() as u32;
    for i in (3..=limit).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

/// Calculates the nth Fibonacci number (0-indexed).
///
/// # Examples
///
/// ```
/// use testing_benchmarking::fibonacci;
/// assert_eq!(fibonacci(0), 0);
/// assert_eq!(fibonacci(1), 1);
/// assert_eq!(fibonacci(10), 55);
/// ```
pub fn fibonacci(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut a = 0u64;
            let mut b = 1u64;
            for _ in 2..=n {
                let temp = a + b;
                a = b;
                b = temp;
            }
            b
        }
    }
}

/// A simple calculator struct to demonstrate testing methods.
pub struct Calculator {
    pub value: i32,
}

impl Calculator {
    /// Creates a new calculator with an initial value.
    ///
    /// # Examples
    ///
    /// ```
    /// use testing_benchmarking::Calculator;
    /// let calc = Calculator::new(10);
    /// assert_eq!(calc.value, 10);
    /// ```
    pub fn new(initial: i32) -> Self {
        Calculator { value: initial }
    }

    /// Adds to the current value.
    pub fn add(&mut self, n: i32) {
        self.value += n;
    }

    /// Subtracts from the current value.
    pub fn subtract(&mut self, n: i32) {
        self.value -= n;
    }

    /// Multiplies the current value.
    pub fn multiply(&mut self, n: i32) {
        self.value *= n;
    }

    /// Resets the calculator to zero.
    pub fn reset(&mut self) {
        self.value = 0;
    }
}

// ============================================================================
// UNIT TESTS
// ============================================================================
// Unit tests are placed in the same file as the code they test, inside a
// module marked with #[cfg(test)]. This module is only compiled when running
// tests, so it has zero cost in release builds.

#[cfg(test)]
mod tests {
    // Import everything from the parent module
    use super::*;

    // ========================================================================
    // BASIC TEST EXAMPLES
    // ========================================================================

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(10, 3), 7);
        assert_eq!(subtract(5, 10), -5);
        assert_eq!(subtract(0, 0), 0);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(3, 4), 12);
        assert_eq!(multiply(-2, 5), -10);
        assert_eq!(multiply(0, 100), 0);
    }

    // ========================================================================
    // TESTING FUNCTIONS THAT RETURN RESULT
    // ========================================================================

    #[test]
    fn test_divide_success() {
        assert_eq!(divide(10, 2).unwrap(), 5);
        assert_eq!(divide(100, 4).unwrap(), 25);
        assert_eq!(divide(-10, 2).unwrap(), -5);
    }

    #[test]
    fn test_divide_by_zero() {
        // Test that dividing by zero returns an error
        assert!(divide(10, 0).is_err());

        // We can also check the error message
        match divide(10, 0) {
            Err(msg) => assert_eq!(msg, "Cannot divide by zero"),
            Ok(_) => panic!("Expected an error!"),
        }
    }

    // Alternative: Use Result<T, E> in test functions
    #[test]
    fn test_divide_with_result() -> Result<(), String> {
        assert_eq!(divide(20, 4)?, 5);
        Ok(())
    }

    // ========================================================================
    // TESTING BOOLEAN FUNCTIONS
    // ========================================================================

    #[test]
    fn test_is_even() {
        assert!(is_even(0));
        assert!(is_even(2));
        assert!(is_even(100));
        assert!(is_even(-4));

        assert!(!is_even(1));
        assert!(!is_even(3));
        assert!(!is_even(-7));
    }

    #[test]
    fn test_is_prime() {
        // Test prime numbers
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(is_prime(5));
        assert!(is_prime(7));
        assert!(is_prime(11));
        assert!(is_prime(97));

        // Test non-prime numbers
        assert!(!is_prime(0));
        assert!(!is_prime(1));
        assert!(!is_prime(4));
        assert!(!is_prime(6));
        assert!(!is_prime(8));
        assert!(!is_prime(9));
        assert!(!is_prime(100));
    }

    // ========================================================================
    // TESTING WITH DIFFERENT INPUTS
    // ========================================================================

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(3), 2);
        assert_eq!(fibonacci(4), 3);
        assert_eq!(fibonacci(5), 5);
        assert_eq!(fibonacci(6), 8);
        assert_eq!(fibonacci(10), 55);
        assert_eq!(fibonacci(20), 6765);
    }

    // ========================================================================
    // TESTING STRUCTS AND METHODS
    // ========================================================================

    #[test]
    fn test_calculator_new() {
        let calc = Calculator::new(10);
        assert_eq!(calc.value, 10);

        let calc2 = Calculator::new(-5);
        assert_eq!(calc2.value, -5);
    }

    #[test]
    fn test_calculator_add() {
        let mut calc = Calculator::new(10);
        calc.add(5);
        assert_eq!(calc.value, 15);

        calc.add(-3);
        assert_eq!(calc.value, 12);
    }

    #[test]
    fn test_calculator_subtract() {
        let mut calc = Calculator::new(20);
        calc.subtract(5);
        assert_eq!(calc.value, 15);

        calc.subtract(20);
        assert_eq!(calc.value, -5);
    }

    #[test]
    fn test_calculator_multiply() {
        let mut calc = Calculator::new(5);
        calc.multiply(3);
        assert_eq!(calc.value, 15);

        calc.multiply(0);
        assert_eq!(calc.value, 0);
    }

    #[test]
    fn test_calculator_reset() {
        let mut calc = Calculator::new(100);
        calc.add(50);
        calc.multiply(2);
        assert_eq!(calc.value, 300);

        calc.reset();
        assert_eq!(calc.value, 0);
    }

    #[test]
    fn test_calculator_chaining() {
        let mut calc = Calculator::new(10);
        calc.add(5);      // 15
        calc.multiply(2); // 30
        calc.subtract(10); // 20
        assert_eq!(calc.value, 20);
    }

    // ========================================================================
    // TESTING THAT CODE PANICS
    // ========================================================================
    // Use #[should_panic] when testing that code panics under certain conditions

    fn divide_panic(a: i32, b: i32) -> i32 {
        if b == 0 {
            panic!("Cannot divide by zero!");
        }
        a / b
    }

    #[test]
    #[should_panic]
    fn test_divide_panic() {
        divide_panic(10, 0);  // This should panic
    }

    #[test]
    #[should_panic(expected = "Cannot divide by zero")]
    fn test_divide_panic_with_message() {
        divide_panic(10, 0);  // Must panic with specific message
    }

    // ========================================================================
    // TESTING PRIVATE FUNCTIONS
    // ========================================================================
    // Unit tests can access private functions because they're in the same module!

    fn internal_helper(x: i32) -> i32 {
        x * 2 + 1
    }

    #[test]
    fn test_private_function() {
        assert_eq!(internal_helper(5), 11);
        assert_eq!(internal_helper(0), 1);
    }

    // ========================================================================
    // IGNORING TESTS
    // ========================================================================
    // Sometimes you want to skip expensive or flaky tests

    #[test]
    #[ignore]
    fn expensive_test() {
        // This test is ignored by default
        // Run with: cargo test -- --ignored
        for _ in 0..1_000_000 {
            let _ = fibonacci(20);
        }
    }
}

// ============================================================================
// INTEGRATION TESTS
// ============================================================================
// Integration tests go in the tests/ directory at the project root.
// Each file in tests/ is compiled as a separate crate.
// They can only test the public API (can't access private functions).
//
// Example: tests/integration_test.rs
// ```
// use testing_benchmarking::*;
//
// #[test]
// fn test_public_api() {
//     assert_eq!(add(2, 3), 5);
// }
// ```

// ============================================================================
// BENCHMARKING WITH CRITERION
// ============================================================================
// Criterion provides statistical benchmarking. Add to Cargo.toml:
//
// [dev-dependencies]
// criterion = "0.5"
//
// [[bench]]
// name = "my_benchmark"
// harness = false
//
// Then create benches/my_benchmark.rs:
// ```
// use criterion::{black_box, criterion_group, criterion_main, Criterion};
// use testing_benchmarking::*;
//
// fn benchmark_fibonacci(c: &mut Criterion) {
//     c.bench_function("fibonacci 20", |b| {
//         b.iter(|| fibonacci(black_box(20)))
//     });
// }
//
// criterion_group!(benches, benchmark_fibonacci);
// criterion_main!(benches);
// ```

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. Use #[test] to mark test functions
// 2. Use #[cfg(test)] for test-only code (zero cost in release)
// 3. assert!, assert_eq!, assert_ne! for assertions
// 4. Unit tests in the same file, integration tests in tests/
// 5. Doc tests in /// comments are automatically tested
// 6. Tests run in parallel by default (be careful with shared state)
// 7. Use Result<T, E> in tests for cleaner error handling
// 8. #[should_panic] for testing panics
// 9. #[ignore] for expensive tests
// 10. Use criterion for reliable benchmarks

// ============================================================================
// COMMON MISTAKES
// ============================================================================
// ❌ Forgetting #[test] attribute (test won't run)
// ❌ Using assert_eq! for floats (precision issues)
// ❌ Tests that depend on execution order (breaks parallelism)
// ❌ Not testing error cases (only happy path)
// ❌ Testing implementation details instead of behavior
// ❌ Slow tests mixed with fast tests (use #[ignore] or separate)
// ❌ Not using #[cfg(test)] (test code in release builds)
// ❌ Integration tests trying to import each other
// ❌ Benchmarking without proper statistics
// ❌ Not organizing tests into logical modules
