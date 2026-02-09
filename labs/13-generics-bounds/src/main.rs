//! # Generics and Trait Bounds Demo

use generics_bounds::solution::{self, Pair, Point};

fn main() {
    println!("=== Generics and Trait Bounds Demo ===\n");

    let ints = [34, 50, 25, 100, 65];
    let chars = ['y', 'm', 'a', 'q'];

    println!("largest(ints): {}", solution::largest(&ints));
    println!("largest(chars): {}\n", solution::largest(&chars));

    let p1 = Point::new(3.0, 4.0);
    let p2 = Point::new(6.0, 8.0);
    println!("p1: ({}, {})", p1.x(), p1.y());
    println!("compare_distance: {}\n", p1.compare_distance(&p2));

    let pair = Pair::new(42, String::from("rust"));
    let swapped = pair.swap();
    println!("swapped.first: {}", swapped.first);
    println!("swapped.second: {}", swapped.second);
}
