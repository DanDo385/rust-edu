//! Integration tests for traits-basics

use traits_basics::solution::*;

#[test]
fn test_circle_area() {
    let circle = Circle { radius: 5.0 };
    assert!((circle.area() - 78.54).abs() < 0.01);
}

#[test]
fn test_rectangle_area() {
    let rect = Rectangle { width: 10.0, height: 20.0 };
    assert_eq!(rect.area(), 200.0);
}

#[test]
fn test_total_area() {
    let circle = Circle { radius: 5.0 };
    let rect = Rectangle { width: 10.0, height: 20.0 };
    let shapes: Vec<&dyn Shape> = vec![&circle, &rect];
    let total = total_area(&shapes);
    assert!((total - 278.54).abs() < 0.01);
}

#[test]
fn test_largest_shape() {
    let circle = Circle { radius: 5.0 };
    let rect = Rectangle { width: 10.0, height: 20.0 };
    let shapes: Vec<&dyn Shape> = vec![&circle, &rect];
    let largest = largest_shape(&shapes).unwrap();
    assert!((largest.area() - 200.0).abs() < 0.01);
}
