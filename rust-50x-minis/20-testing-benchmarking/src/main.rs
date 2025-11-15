// Project 20: Testing and Benchmarking - Main Program
//
// This is a simple driver program that demonstrates the library functions.
// The real value of this project is in the tests (see lib.rs).
//
// To see the tests in action:
// - Run tests: cargo test
// - Run specific test: cargo test test_fibonacci
// - Run with output: cargo test -- --nocapture
// - Run ignored tests: cargo test -- --ignored
// - Run benchmarks: cargo bench (requires criterion setup)

// Import from our library
use testing_benchmarking::*;

fn main() {
    println!("=== Rust Testing and Benchmarking ===\n");

    demonstrate_basic_functions();
    demonstrate_calculator();
    demonstrate_fibonacci();
    demonstrate_prime_numbers();

    println!("\n=== Testing Instructions ===");
    println!("Run 'cargo test' to execute all tests");
    println!("Run 'cargo test --lib' for library tests only");
    println!("Run 'cargo test test_fibonacci' to run a specific test");
    println!("Run 'cargo test -- --nocapture' to see println! output");
    println!("Run 'cargo test -- --test-threads=1' to run tests sequentially");
    println!("Run 'cargo test -- --ignored' to run ignored tests");
    println!("\nRun 'cargo bench' to execute benchmarks (requires criterion)");
}

// ============================================================================
// DEMONSTRATING LIBRARY FUNCTIONS
// ============================================================================

fn demonstrate_basic_functions() {
    println!("--- Basic Math Functions ---");

    let a = 10;
    let b = 3;

    println!("add({}, {}) = {}", a, b, add(a, b));
    println!("subtract({}, {}) = {}", a, b, subtract(a, b));
    println!("multiply({}, {}) = {}", a, b, multiply(a, b));

    match divide(a, b) {
        Ok(result) => println!("divide({}, {}) = {}", a, b, result),
        Err(e) => println!("divide({}, {}) failed: {}", a, b, e),
    }

    match divide(a, 0) {
        Ok(result) => println!("divide({}, 0) = {}", a, result),
        Err(e) => println!("divide({}, 0) failed: {}", a, e),
    }

    println!();
}

fn demonstrate_calculator() {
    println!("--- Calculator Demo ---");

    let mut calc = Calculator::new(10);
    println!("Initial value: {}", calc.value);

    calc.add(5);
    println!("After adding 5: {}", calc.value);

    calc.multiply(2);
    println!("After multiplying by 2: {}", calc.value);

    calc.subtract(10);
    println!("After subtracting 10: {}", calc.value);

    calc.reset();
    println!("After reset: {}", calc.value);

    println!();
}

fn demonstrate_fibonacci() {
    println!("--- Fibonacci Sequence ---");

    print!("First 15 Fibonacci numbers: ");
    for i in 0..15 {
        print!("{} ", fibonacci(i));
    }
    println!("\n");
}

fn demonstrate_prime_numbers() {
    println!("--- Prime Numbers ---");

    print!("Prime numbers up to 50: ");
    for n in 0..=50 {
        if is_prime(n) {
            print!("{} ", n);
        }
    }
    println!("\n");
}

// ============================================================================
// EXAMPLE: WRITING A NEW FEATURE WITH TDD
// ============================================================================
// Test-Driven Development (TDD) workflow:
// 1. Write the test first (it will fail)
// 2. Write minimal code to make test pass
// 3. Refactor if needed
// 4. Repeat

// Let's say we want to add a power function. Start with tests in lib.rs:
// ```
// #[test]
// fn test_power() {
//     assert_eq!(power(2, 3), 8);
//     assert_eq!(power(5, 2), 25);
//     assert_eq!(power(10, 0), 1);
// }
// ```
//
// Run cargo test - it fails (function doesn't exist)
//
// Now implement in lib.rs:
// ```
// pub fn power(base: i32, exponent: u32) -> i32 {
//     base.pow(exponent)
// }
// ```
//
// Run cargo test - it passes!

// ============================================================================
// INTEGRATION TESTING EXAMPLE
// ============================================================================
// Create tests/integration_test.rs:
// ```
// use testing_benchmarking::*;
//
// #[test]
// fn test_calculator_workflow() {
//     let mut calc = Calculator::new(0);
//     calc.add(10);
//     calc.multiply(5);
//     assert_eq!(calc.value, 50);
// }
//
// #[test]
// fn test_math_operations() {
//     assert_eq!(add(2, 3), 5);
//     assert_eq!(multiply(4, 5), 20);
//     assert!(divide(10, 0).is_err());
// }
// ```

// ============================================================================
// BENCHMARKING EXAMPLE
// ============================================================================
// Create benches/fibonacci_benchmark.rs:
// ```
// use criterion::{black_box, criterion_group, criterion_main, Criterion};
// use testing_benchmarking::fibonacci;
//
// fn fibonacci_benchmark(c: &mut Criterion) {
//     c.bench_function("fibonacci 10", |b| {
//         b.iter(|| fibonacci(black_box(10)))
//     });
//
//     c.bench_function("fibonacci 20", |b| {
//         b.iter(|| fibonacci(black_box(20)))
//     });
//
//     c.bench_function("fibonacci 30", |b| {
//         b.iter(|| fibonacci(black_box(30)))
//     });
// }
//
// criterion_group!(benches, fibonacci_benchmark);
// criterion_main!(benches);
// ```
//
// And add to Cargo.toml:
// ```
// [dev-dependencies]
// criterion = "0.5"
//
// [[bench]]
// name = "fibonacci_benchmark"
// harness = false
// ```
//
// Run: cargo bench

// ============================================================================
// PROPERTY-BASED TESTING EXAMPLE
// ============================================================================
// Property-based testing generates random inputs to find edge cases.
// Use quickcheck or proptest:
//
// Add to Cargo.toml:
// ```
// [dev-dependencies]
// quickcheck = "1.0"
// quickcheck_macros = "1.0"
// ```
//
// In lib.rs tests:
// ```
// use quickcheck_macros::quickcheck;
//
// #[quickcheck]
// fn test_add_commutative(a: i32, b: i32) -> bool {
//     add(a, b) == add(b, a)  // Addition is commutative
// }
//
// #[quickcheck]
// fn test_add_associative(a: i32, b: i32, c: i32) -> bool {
//     add(add(a, b), c) == add(a, add(b, c))  // Addition is associative
// }
// ```

// ============================================================================
// KEY TESTING PRINCIPLES
// ============================================================================
// 1. Test behavior, not implementation
// 2. Test edge cases and error conditions
// 3. Keep tests simple and focused
// 4. Tests should be independent (no shared state)
// 5. Fast tests > slow tests (use #[ignore] for slow ones)
// 6. Test the public API (integration tests)
// 7. Use descriptive test names
// 8. One assertion per test (when possible)
// 9. Don't test the standard library
// 10. Refactor tests when they become hard to maintain

// ============================================================================
// COMMON TESTING PATTERNS
// ============================================================================

// Pattern 1: Setup and Teardown
// ```
// #[test]
// fn test_with_setup() {
//     // Setup
//     let mut calc = Calculator::new(0);
//
//     // Execute
//     calc.add(10);
//
//     // Assert
//     assert_eq!(calc.value, 10);
//
//     // Teardown (usually automatic in Rust via Drop)
// }
// ```

// Pattern 2: Table-Driven Tests
// ```
// #[test]
// fn test_is_even_multiple_cases() {
//     let test_cases = vec![
//         (0, true),
//         (1, false),
//         (2, true),
//         (3, false),
//         (100, true),
//         (101, false),
//     ];
//
//     for (input, expected) in test_cases {
//         assert_eq!(is_even(input), expected, "Failed for input: {}", input);
//     }
// }
// ```

// Pattern 3: Testing Error Messages
// ```
// #[test]
// fn test_error_message() {
//     match divide(10, 0) {
//         Err(msg) => assert!(msg.contains("zero")),
//         Ok(_) => panic!("Expected error"),
//     }
// }
// ```

// Pattern 4: Parameterized Tests (with a macro)
// ```
// macro_rules! test_fibonacci {
//     ($($name:ident: $value:expr,)*) => {
//     $(
//         #[test]
//         fn $name() {
//             let (input, expected) = $value;
//             assert_eq!(fibonacci(input), expected);
//         }
//     )*
//     }
// }
//
// test_fibonacci! {
//     fib_0: (0, 0),
//     fib_1: (1, 1),
//     fib_10: (10, 55),
// }
// ```

// ============================================================================
// TEST ORGANIZATION BEST PRACTICES
// ============================================================================
// 1. Unit tests: Same file as code, in #[cfg(test)] module
// 2. Integration tests: tests/ directory, each file is a crate
// 3. Common test utilities: tests/common/mod.rs
// 4. Benchmarks: benches/ directory
// 5. Examples: examples/ directory (also tested with cargo test)
// 6. Documentation tests: In /// comments
// 7. Use submodules for organizing many tests

// ============================================================================
// CONTINUOUS INTEGRATION
// ============================================================================
// GitHub Actions example (.github/workflows/test.yml):
// ```
// name: Tests
//
// on: [push, pull_request]
//
// jobs:
//   test:
//     runs-on: ubuntu-latest
//     steps:
//       - uses: actions/checkout@v2
//       - uses: actions-rs/toolchain@v1
//         with:
//           toolchain: stable
//       - run: cargo test --all-features
//       - run: cargo bench --no-run
// ```
