//! Integration tests for Lab 56: Structs & Methods (Alternative)
//!
//! Tests verify Rectangle methods, Point operations, and Counter chaining.

use structs_methods_alternative::solution::*;

// ============================================================================
// RECTANGLE TESTS
// ============================================================================

#[test]
fn test_rectangle_new() {
    let r = Rectangle::new(10, 20);
    assert_eq!(r.width, 10);
    assert_eq!(r.height, 20);
}

#[test]
fn test_rectangle_square() {
    let sq = Rectangle::square(5);
    assert_eq!(sq.width, 5);
    assert_eq!(sq.height, 5);
}

#[test]
fn test_rectangle_area() {
    assert_eq!(Rectangle::new(10, 20).area(), 200);
    assert_eq!(Rectangle::new(1, 1).area(), 1);
    assert_eq!(Rectangle::new(0, 100).area(), 0);
}

#[test]
fn test_rectangle_perimeter() {
    assert_eq!(Rectangle::new(10, 20).perimeter(), 60);
    assert_eq!(Rectangle::new(5, 5).perimeter(), 20);
}

#[test]
fn test_rectangle_can_hold_true() {
    let big = Rectangle::new(30, 50);
    let small = Rectangle::new(10, 20);
    assert!(big.can_hold(&small));
}

#[test]
fn test_rectangle_can_hold_false() {
    let big = Rectangle::new(30, 50);
    let small = Rectangle::new(10, 20);
    assert!(!small.can_hold(&big));
}

#[test]
fn test_rectangle_can_hold_equal_size() {
    let r1 = Rectangle::new(10, 10);
    let r2 = Rectangle::new(10, 10);
    // Equal size cannot "hold" (requires strictly greater)
    assert!(!r1.can_hold(&r2));
}

#[test]
fn test_rectangle_can_hold_one_dimension_equal() {
    let r1 = Rectangle::new(10, 20);
    let r2 = Rectangle::new(10, 15);
    // Width is equal, so r1 cannot hold r2
    assert!(!r1.can_hold(&r2));
}

#[test]
fn test_rectangle_is_square() {
    assert!(Rectangle::square(5).is_square());
    assert!(Rectangle::new(7, 7).is_square());
    assert!(!Rectangle::new(3, 5).is_square());
}

#[test]
fn test_rectangle_scale() {
    let mut r = Rectangle::new(10, 20);
    r.scale(3);
    assert_eq!(r.width, 30);
    assert_eq!(r.height, 60);
}

#[test]
fn test_rectangle_scale_by_one() {
    let mut r = Rectangle::new(5, 10);
    r.scale(1);
    assert_eq!(r, Rectangle::new(5, 10));
}

#[test]
fn test_rectangle_scale_by_zero() {
    let mut r = Rectangle::new(5, 10);
    r.scale(0);
    assert_eq!(r.area(), 0);
}

#[test]
fn test_rectangle_into_tuple() {
    let r = Rectangle::new(10, 20);
    let t = r.into_tuple();
    assert_eq!(t, (10, 20));
}

#[test]
fn test_rectangle_debug() {
    let r = Rectangle::new(10, 20);
    let debug_str = format!("{:?}", r);
    assert!(debug_str.contains("10"));
    assert!(debug_str.contains("20"));
}

#[test]
fn test_rectangle_copy() {
    let r1 = Rectangle::new(10, 20);
    let r2 = r1; // Copy, not move
    assert_eq!(r1.area(), 200); // r1 is still valid
    assert_eq!(r2.area(), 200);
}

#[test]
fn test_rectangle_equality() {
    let r1 = Rectangle::new(10, 20);
    let r2 = Rectangle::new(10, 20);
    let r3 = Rectangle::new(20, 10);
    assert_eq!(r1, r2);
    assert_ne!(r1, r3);
}

// ============================================================================
// POINT TESTS
// ============================================================================

#[test]
fn test_point_new() {
    let p = Point::new(3.0, 4.0);
    assert_eq!(p.x, 3.0);
    assert_eq!(p.y, 4.0);
}

#[test]
fn test_point_origin() {
    let p = Point::origin();
    assert_eq!(p.x, 0.0);
    assert_eq!(p.y, 0.0);
}

#[test]
fn test_point_distance_from_origin() {
    let p = Point::new(3.0, 4.0);
    let dist = p.distance_from_origin();
    assert!((dist - 5.0).abs() < 1e-10);
}

#[test]
fn test_point_distance_from_origin_zero() {
    let p = Point::origin();
    assert_eq!(p.distance_from_origin(), 0.0);
}

#[test]
fn test_point_distance_to() {
    let p1 = Point::new(0.0, 0.0);
    let p2 = Point::new(3.0, 4.0);
    let dist = p1.distance_to(&p2);
    assert!((dist - 5.0).abs() < 1e-10);
}

#[test]
fn test_point_distance_to_same() {
    let p = Point::new(5.0, 5.0);
    assert_eq!(p.distance_to(&p), 0.0);
}

#[test]
fn test_point_translate() {
    let mut p = Point::new(1.0, 2.0);
    p.translate(5.0, -3.0);
    assert!((p.x - 6.0).abs() < 1e-10);
    assert!((p.y - (-1.0)).abs() < 1e-10);
}

#[test]
fn test_point_translate_to_origin() {
    let mut p = Point::new(3.0, 4.0);
    p.translate(-3.0, -4.0);
    assert!((p.x).abs() < 1e-10);
    assert!((p.y).abs() < 1e-10);
}

#[test]
fn test_point_copy() {
    let p1 = Point::new(1.0, 2.0);
    let p2 = p1; // Copy
    assert_eq!(p1.x, 1.0); // p1 still valid
    assert_eq!(p2.x, 1.0);
}

// ============================================================================
// COUNTER TESTS
// ============================================================================

#[test]
fn test_counter_new() {
    let c = Counter::new();
    assert_eq!(c.get(), 0);
}

#[test]
fn test_counter_default() {
    let c = Counter::default();
    assert_eq!(c.get(), 0);
}

#[test]
fn test_counter_increment() {
    let mut c = Counter::new();
    c.increment();
    assert_eq!(c.get(), 1);
}

#[test]
fn test_counter_decrement() {
    let mut c = Counter::new();
    c.increment().increment().increment();
    c.decrement();
    assert_eq!(c.get(), 2);
}

#[test]
fn test_counter_chaining() {
    let mut c = Counter::new();
    c.increment().increment().increment().decrement();
    assert_eq!(c.get(), 2);
}

#[test]
fn test_counter_reset() {
    let mut c = Counter::new();
    c.increment().increment().increment();
    c.reset();
    assert_eq!(c.get(), 0);
}

#[test]
fn test_counter_reset_and_continue() {
    let mut c = Counter::new();
    c.increment().increment();
    c.reset().increment();
    assert_eq!(c.get(), 1);
}

#[test]
fn test_counter_negative() {
    let mut c = Counter::new();
    c.decrement().decrement();
    assert_eq!(c.get(), -2);
}
