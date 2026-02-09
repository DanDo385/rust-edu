//! # Lab 23: Testing and Benchmarking
//!
//! Student-facing API with arithmetic and utility exercises.

pub fn add(a: i32, b: i32) -> i32 {
    // TODO: Return sum.
    let _ = (a, b);
    todo!("Implement add")
}

pub fn subtract(a: i32, b: i32) -> i32 {
    // TODO: Return difference.
    let _ = (a, b);
    todo!("Implement subtract")
}

pub fn multiply(a: i32, b: i32) -> i32 {
    // TODO: Return product.
    let _ = (a, b);
    todo!("Implement multiply")
}

pub fn divide(a: i32, b: i32) -> Result<i32, String> {
    // TODO: Return Err when b == 0.
    let _ = (a, b);
    todo!("Implement divide")
}

pub fn is_even(n: i32) -> bool {
    // TODO: Return true when n is divisible by 2.
    let _ = n;
    todo!("Implement is_even")
}

pub fn is_prime(n: u32) -> bool {
    // TODO: Determine if n is prime.
    let _ = n;
    todo!("Implement is_prime")
}

pub fn fibonacci(n: u32) -> u64 {
    // TODO: Compute nth Fibonacci value (0-indexed).
    let _ = n;
    todo!("Implement fibonacci")
}

pub struct Calculator {
    pub value: i32,
}

impl Calculator {
    pub fn new(initial: i32) -> Self {
        // TODO: Construct calculator with initial value.
        let _ = initial;
        todo!("Create Calculator")
    }

    pub fn add(&mut self, n: i32) {
        // TODO: Increase current value.
        let _ = n;
        todo!("Calculator add")
    }

    pub fn subtract(&mut self, n: i32) {
        // TODO: Decrease current value.
        let _ = n;
        todo!("Calculator subtract")
    }

    pub fn multiply(&mut self, n: i32) {
        // TODO: Multiply current value.
        let _ = n;
        todo!("Calculator multiply")
    }

    pub fn reset(&mut self) {
        // TODO: Reset value to zero.
        todo!("Calculator reset")
    }
}

#[doc(hidden)]
pub mod solution;
