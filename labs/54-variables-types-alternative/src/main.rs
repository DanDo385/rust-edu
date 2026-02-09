//! # Variables & Types Demo

use variables_types_alternative::solution::{
    add_i32, char_to_u32, f64_to_i32, i32_to_f64, is_even, tuple_sum,
};

fn main() {
    println!("=== Variables & Types Demo ===");
    println!("2 + 3 = {}", add_i32(2, 3));
    println!("is_even(42) = {}", is_even(42));
    println!("tuple_sum((1,2,3)) = {}", tuple_sum((1, 2, 3)));
    println!("'A' -> {}", char_to_u32('A'));
    println!("7 -> {}", i32_to_f64(7));
    println!("3.99 -> {}", f64_to_i32(3.99));
}
