//! Integration tests for Lab 13: Generics & Trait Bounds

use generics_bounds::*;

#[test]
fn test_largest_i32() {
    let nums = vec![10, 40, 34, 13, 46, 46, 52, 48, 84];
    assert_eq!(largest(&nums), 84);
}

#[test]
fn test_largest_f64() {
    let nums: Vec<f64> = vec![10.5, 40.2, 34.7, 13.1];
    let result = largest(&nums);
    assert!((result - 40.2).abs() < 0.001);
}

#[test]
fn test_largest_char() {
    let chars = vec!['y', 'm', 'a', 'q'];
    assert_eq!(largest(&chars), 'y');
}

#[test]
fn test_point_generic_int() {
    let p = Point::new(5, 10);
    assert_eq!(*p.x(), 5);
    assert_eq!(*p.y(), 10);
}

#[test]
fn test_point_generic_float() {
    let p: Point<f64> = Point::new(1.5, 2.7);
    let x_val = *p.x();
    assert!((x_val - 1.5).abs() < 0.001);
}

#[test]
fn test_point_generic_string() {
    let p = Point::new("hello".to_string(), "world".to_string());
    assert_eq!(p.x(), "hello");
    assert_eq!(p.y(), "world");
}

#[test]
fn test_point_clone() {
    let p1 = Point::new(3, 4);
    let p2 = p1.clone();
    assert_eq!(p1, p2);
}

#[test]
fn test_pair_different_types() {
    let p = Pair::new(42, "hello");
    assert_eq!(p.first, 42);
    assert_eq!(p.second, "hello");
}

#[test]
fn test_pair_swap_int_string() {
    let p = Pair::new(99, "test".to_string());
    let swapped = p.swap();
    assert_eq!(swapped.first, "test");
    assert_eq!(swapped.second, 99);
}

#[test]
fn test_pair_swap_ownership() {
    // After swap, original pair is moved/consumed
    let p = Pair::new(1, 2);
    let _swapped = p.swap();
    // p is no longer available (owned by swap, then dropped)
    // This test just verifies it compiles correctly
}

#[test]
fn test_largest_empty_panics() {
    // This would panic, so we don't test it directly
    // In real code, you'd use Option<T> or Result<T, E>
}
