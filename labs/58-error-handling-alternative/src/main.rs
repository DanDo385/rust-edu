//! # Error Handling Demo

use error_handling_alternative::solution::{divide, safe_add, safe_divide, safe_sqrt};

fn main() {
    println!("=== Error Handling Demo ===");

    println!("divide(10,2)={:?}", divide(10.0, 2.0));
    println!("divide(10,0)={:?}", divide(10.0, 0.0));

    println!("safe_divide(10,2)={:?}", safe_divide(10.0, 2.0));
    println!("safe_divide(10,0)={:?}", safe_divide(10.0, 0.0));
    println!("safe_sqrt(9)={:?}", safe_sqrt(9.0));
    println!("safe_add(i32::MAX,1)={:?}", safe_add(i32::MAX, 1));
}
