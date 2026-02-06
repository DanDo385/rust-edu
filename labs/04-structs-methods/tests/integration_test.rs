//! Integration tests for structs-methods

use structs_methods::solution::Rectangle;

#[test]
fn test_new_valid() {
    let rect = Rectangle::new(10, 20);
    assert!(rect.is_some());
    let rect = rect.unwrap();
    assert_eq!(rect.width, 10);
    assert_eq!(rect.height, 20);
}

#[test]
fn test_new_zero_width() {
    assert_eq!(Rectangle::new(0, 20), None);
}

#[test]
fn test_new_zero_height() {
    assert_eq!(Rectangle::new(10, 0), None);
}

#[test]
fn test_new_both_zero() {
    assert_eq!(Rectangle::new(0, 0), None);
}

#[test]
fn test_area() {
    let rect = Rectangle::new(10, 20).unwrap();
    assert_eq!(rect.area(), 200);
}

#[test]
fn test_area_square() {
    let rect = Rectangle::new(5, 5).unwrap();
    assert_eq!(rect.area(), 25);
}

#[test]
fn test_perimeter() {
    let rect = Rectangle::new(10, 20).unwrap();
    assert_eq!(rect.perimeter(), 60);
}

#[test]
fn test_perimeter_square() {
    let rect = Rectangle::new(5, 5).unwrap();
    assert_eq!(rect.perimeter(), 20);
}

#[test]
fn test_can_fit_yes() {
    let big = Rectangle::new(100, 100).unwrap();
    let small = Rectangle::new(50, 50).unwrap();
    assert!(big.can_fit(&small));
}

#[test]
fn test_can_fit_no() {
    let small = Rectangle::new(50, 50).unwrap();
    let big = Rectangle::new(100, 100).unwrap();
    assert!(!small.can_fit(&big));
}

#[test]
fn test_can_fit_equal() {
    let rect1 = Rectangle::new(50, 50).unwrap();
    let rect2 = Rectangle::new(50, 50).unwrap();
    assert!(rect1.can_fit(&rect2));
}

#[test]
fn test_scale_double() {
    let rect = Rectangle::new(10, 20).unwrap();
    let scaled = rect.scale(2);
    assert_eq!(scaled.width, 20);
    assert_eq!(scaled.height, 40);
    assert_eq!(rect.width, 10); // Original unchanged
}

#[test]
fn test_scale_triple() {
    let rect = Rectangle::new(5, 10).unwrap();
    let scaled = rect.scale(3);
    assert_eq!(scaled.width, 15);
    assert_eq!(scaled.height, 30);
}

#[test]
fn test_scale_one() {
    let rect = Rectangle::new(10, 20).unwrap();
    let scaled = rect.scale(1);
    assert_eq!(scaled.width, 10);
    assert_eq!(scaled.height, 20);
}
