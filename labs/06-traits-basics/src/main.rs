//! # Traits Basics Demo

use traits_basics::solution::*;

fn main() {
    println!("=== Traits Basics Demo ===\n");

    let circle = Circle { radius: 5.0 };
    let rect = Rectangle { width: 10.0, height: 20.0 };

    println!("Circle (r=5): area={:.2}, perimeter={:.2}",
        circle.area(), circle.perimeter());
    println!("Rectangle (10x20): area={:.2}, perimeter={:.2}",
        rect.area(), rect.perimeter());

    let shapes: Vec<&dyn Shape> = vec![&circle, &rect];
    println!("\nTotal area: {:.2}", total_area(&shapes));

    if let Some(largest) = largest_shape(&shapes) {
        println!("Largest shape area: {:.2}", largest.area());
    }

    println!("\n=== Demo Complete! ===");
}
