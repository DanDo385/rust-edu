//! # Operator Overloading - Demo

use operator_overloading::solution::{Complex, Rational};

fn main() {
    println!("=== Operator Overloading Demo ===");

    let a = Rational::new(1, 2);
    let b = Rational::new(1, 3);
    println!("{} + {} = {}", a, b, a + b);
    println!("{} - {} = {}", a, b, a - b);
    println!("{} * {} = {}", a, b, a * b);
    println!("{} / {} = {}", a, b, a / b);

    let c1 = Complex::new(3.0, 4.0);
    let c2 = Complex::new(1.0, -2.0);
    println!("{:?} + {:?} = {:?}", c1, c2, c1 + c2);
    println!("{:?} * {:?} = {:?}", c1, c2, c1 * c2);
    println!("|{:?}| = {}", c1, c1.magnitude());
}
