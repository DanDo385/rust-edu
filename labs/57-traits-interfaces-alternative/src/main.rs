//! # Traits & Interfaces Demo

use traits_interfaces_alternative::solution::{get_description, Car, Describe, Person};

fn main() {
    println!("=== Traits & Interfaces Demo ===");

    let person = Person::new("Alice", 30);
    let car = Car::new("Toyota", 2020);

    println!("{} [{}]", person.describe(), person.label());
    println!("{} [{}]", car.describe(), car.label());
    println!("generic describe: {}", get_description(&person));
}
