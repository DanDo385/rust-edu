# Project 20: Testing and Benchmarking

## Overview
Learn Rust's built-in testing framework and benchmarking tools. Testing is a first-class citizen in Rust - the compiler and cargo have native support for unit tests, integration tests, documentation tests, and benchmarks. This project demonstrates comprehensive testing strategies.

## Concepts Taught
- **#[test]** attribute for unit tests
- **#[cfg(test)]** for test-only code
- **assert!**, **assert_eq!**, **assert_ne!** macros
- **#[should_panic]** for testing panics
- **Result<T, E>** in tests for error handling
- **Integration tests** in tests/ directory
- **Documentation tests** in doc comments
- **Test organization** and modules
- **cargo test** command and filters
- **Benchmarking** with criterion (external crate)
- **Property-based testing** concepts

## Why Rust Behaves This Way

### Testing as a First-Class Feature
Unlike many languages where testing requires external frameworks, Rust includes:
- Test runner built into cargo
- Test attributes in the language (#[test])
- Assert macros in the standard library
- Separate compilation for tests (zero runtime cost)

**Comparison:**
- **Python**: unittest, pytest (external)
- **JavaScript**: Jest, Mocha (external)
- **Go**: Built-in testing (similar to Rust)
- **Java**: JUnit (external)

### Three Types of Tests

**1. Unit Tests**
- In the same file as code
- Test individual functions/methods
- Can access private functions
- Located in #[cfg(test)] modules

**2. Integration Tests**
- In tests/ directory
- Test the public API
- Each file is a separate crate
- Simulate external usage

**3. Documentation Tests**
- In doc comments (///)
- Ensure examples in docs work
- Automatically tested with cargo test
- Rust's killer feature!

### Benchmarking
Rust's standard library used to include benchmarking but it's now unstable.
The community standard is **criterion**:
- Statistical analysis of performance
- Protects against measurement noise
- Beautiful HTML reports
- Regression detection

## Beginner Pitfalls & Borrow Checker Notes

### Pitfall 1: Tests Not Running
```rust
fn test_something() {  // ❌ Missing #[test] attribute
    assert_eq!(2 + 2, 4);
}
```
**Fix**: Add #[test] attribute:
```rust
#[test]
fn test_something() {  // ✅ Will run with cargo test
    assert_eq!(2 + 2, 4);
}
```

### Pitfall 2: Comparing Floats with assert_eq!
```rust
#[test]
fn test_math() {
    assert_eq!(0.1 + 0.2, 0.3);  // ❌ FAILS due to floating point precision!
}
```
**Fix**: Use approximate comparison:
```rust
#[test]
fn test_math() {
    let result = 0.1 + 0.2;
    assert!((result - 0.3).abs() < 1e-10);  // ✅ Approximate equality
}
```

### Pitfall 3: Not Testing Error Cases
```rust
fn divide(a: i32, b: i32) -> i32 {
    a / b  // What about b == 0?
}

#[test]
fn test_divide() {
    assert_eq!(divide(10, 2), 5);  // ✅ Happy path
    // ❌ What about divide(10, 0)?
}
```
**Fix**: Test error cases:
```rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

#[test]
fn test_divide_by_zero() {
    assert!(divide(10, 0).is_err());  // ✅ Test error case
}
```

### Pitfall 4: Integration Tests Importing Each Other
```rust
// tests/test1.rs
mod common;  // ❌ Each file in tests/ is a separate crate!

// tests/test2.rs
mod common;  // Trying to share code between integration tests
```
**Fix**: Put shared code in tests/common/mod.rs:
```rust
// tests/common/mod.rs
pub fn setup() { /* ... */ }

// tests/test1.rs
mod common;
use common::setup;
```

## Code Walkthrough

See `src/lib.rs` and `src/main.rs` for:
1. A library with comprehensive unit tests
2. Test module organization
3. Testing public and private functions
4. Error case testing
5. Testing with should_panic
6. Integration tests examples
7. Documentation tests
8. Benchmarking examples (with criterion)

## Performance Considerations

**Test Compilation:**
- Tests are only compiled with `cargo test`
- #[cfg(test)] code is stripped in release builds
- Zero runtime cost for shipping code

**Test Execution:**
- Tests run in parallel by default (fast)
- Use `cargo test -- --test-threads=1` for sequential
- Each integration test is a separate binary

**Benchmarking:**
- Criterion uses statistical analysis (slower but accurate)
- Warms up, measures many iterations
- Detects and reports performance regressions
- Use `cargo bench` to run benchmarks

## Comparison: Rust vs Go vs Python

| Feature | Rust | Go | Python |
|---------|------|----|----|
| Built-in testing | Yes (#[test]) | Yes (func TestXxx) | No (unittest stdlib) |
| Test runner | cargo test | go test | pytest (external) |
| Benchmarking | criterion (external) | Built-in | pytest-benchmark |
| Doc tests | Yes (in ///) | Yes (in examples) | Yes (doctest) |
| Parallel execution | Yes (default) | Yes (default) | Yes (pytest-xdist) |
| Mocking | Manual or crates | Manual or crates | unittest.mock |

## Additional Challenges

1. **TDD Exercise**: Write tests first, then implement a calculator.

2. **Property-Based Testing**: Use quickcheck or proptest for generative testing.

3. **Mock Objects**: Create test doubles for external dependencies.

4. **Coverage Report**: Use tarpaulin or llvm-cov to measure code coverage.

5. **Benchmark Suite**: Compare different implementations of the same algorithm.

6. **Fuzz Testing**: Use cargo-fuzz to find edge cases.

## Key Takeaways

1. **#[test]** marks functions as tests
2. **cargo test** runs all tests (unit + integration + doc)
3. Tests are only compiled when running tests (zero cost)
4. Unit tests go in #[cfg(test)] modules in the same file
5. Integration tests go in tests/ directory
6. Documentation tests are in /// comments
7. Use **assert!**, **assert_eq!**, **assert_ne!** for assertions
8. Tests run in parallel by default
9. **#[should_panic]** tests that code panics
10. Use **criterion** for reliable benchmarks

## Common Mistakes

❌ Forgetting #[test] attribute (test won't run)
❌ Using assert_eq! for floats (precision issues)
❌ Not testing error cases (only happy path)
❌ Tests that depend on each other (bad for parallelism)
❌ Not using #[cfg(test)] (test code in release builds)
❌ Testing implementation details instead of behavior
❌ Slow tests that should be benchmarks
❌ Not organizing tests into modules
❌ Integration tests importing each other
❌ Benchmarking without statistical analysis

## Future Directions

- **Next**: Learn about cargo features (Project 21)
- **Advanced**: Build complex applications with confidence
- **Production**: CI/CD integration with automated testing

## Running This Project

### Run all tests:
```bash
cd 20-testing-benchmarking
cargo test
```

### Run specific test:
```bash
cargo test test_name
```

### Run tests with output:
```bash
cargo test -- --nocapture
```

### Run benchmarks (requires criterion):
```bash
cargo bench
```

### Run tests sequentially:
```bash
cargo test -- --test-threads=1
```

## Expected Output

```
running 8 tests
test tests::test_add ... ok
test tests::test_subtract ... ok
test tests::test_multiply ... ok
test tests::test_divide ... ok
test tests::test_divide_by_zero ... ok
test tests::test_is_even ... ok
test tests::test_is_prime ... ok
test tests::test_fibonacci ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests my-library

running 2 tests
test src/lib.rs - add (line 7) ... ok
test src/lib.rs - Calculator::new (line 45) ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Benchmark output:
```
add_numbers            time:   [1.2345 ns 1.2456 ns 1.2567 ns]
fibonacci/10           time:   [234.56 ns 237.89 ns 241.23 ns]
```
