//! # Testing and Benchmarking Demo

use testing_benchmarking::solution::{self, Calculator};

fn main() {
    println!("=== Testing and Benchmarking Demo ===\n");

    println!("add(2, 3) = {}", solution::add(2, 3));
    println!("subtract(10, 4) = {}", solution::subtract(10, 4));
    println!("multiply(6, 7) = {}", solution::multiply(6, 7));
    println!("divide(20, 5) = {:?}", solution::divide(20, 5));
    println!("is_even(42) = {}", solution::is_even(42));
    println!("is_prime(97) = {}", solution::is_prime(97));
    println!("fibonacci(10) = {}", solution::fibonacci(10));

    let mut calc = Calculator::new(10);
    calc.add(5);
    calc.multiply(2);
    calc.subtract(4);
    println!("calculator value = {}", calc.value);
}
