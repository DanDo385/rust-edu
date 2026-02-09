//! # Structs & Methods Demo

use structs_methods_alternative::solution::{Counter, Point, Rectangle};

fn main() {
    println!("=== Structs & Methods Demo ===");

    let mut rect = Rectangle::new(10, 20);
    println!("area={} perimeter={}", rect.area(), rect.perimeter());
    rect.scale(2);
    println!("scaled=({},{})", rect.width, rect.height);

    let p = Point::new(3.0, 4.0);
    println!("distance_from_origin={}", p.distance_from_origin());

    let mut counter = Counter::new();
    counter.increment().increment().decrement();
    println!("counter={}", counter.get());
    counter.reset();
    println!("counter_after_reset={}", counter.get());
}
