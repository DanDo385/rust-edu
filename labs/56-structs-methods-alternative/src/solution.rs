//! # Lab 56: Structs & Methods (Alternative)
//!
//! Alternative implementation demonstrating structs, methods, associated functions,
//! the three forms of self (&self, &mut self, self), and method chaining.
//!
//! ## Ownership Commentary
//! - `&self` borrows the struct immutably (read-only access)
//! - `&mut self` borrows the struct mutably (read-write access, exclusive)
//! - `self` takes ownership (consumes the struct)
//! - Associated functions (no self) are like static methods

// ============================================================================
// RECTANGLE: Core struct with methods
// ============================================================================

/// A rectangle defined by width and height.
///
/// # Memory Model
/// Rectangle is 8 bytes on the stack (two u32 fields, 4 bytes each).
/// It derives Debug for printing and Clone/Copy for value semantics.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    /// Creates a new Rectangle (associated function / constructor).
    ///
    /// # Teaching Note
    /// Associated functions don't take `self` -- they are called on the type
    /// itself, not an instance: `Rectangle::new(10, 20)`.
    pub fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    /// Creates a square (width == height).
    pub fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    /// Calculates the area of the rectangle.
    ///
    /// # Memory Model
    /// `&self` borrows the Rectangle immutably. The caller retains ownership.
    /// Multiple &self calls can happen concurrently (shared reference).
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    /// Calculates the perimeter of the rectangle.
    pub fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }

    /// Returns true if this rectangle can hold (fully contain) another rectangle.
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    /// Returns true if the rectangle is a square.
    pub fn is_square(&self) -> bool {
        self.width == self.height
    }

    /// Scales the rectangle by a factor, modifying it in place.
    ///
    /// # Memory Model
    /// `&mut self` borrows the Rectangle mutably. Only ONE mutable reference
    /// can exist at a time. The borrow checker enforces this at compile time.
    pub fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }

    /// Converts the rectangle into a tuple, consuming the struct.
    ///
    /// # Memory Model
    /// `self` (not &self) takes ownership. After calling this method,
    /// the original Rectangle is moved and can no longer be used.
    pub fn into_tuple(self) -> (u32, u32) {
        (self.width, self.height)
    }
}

// ============================================================================
// POINT: Demonstrates floating-point struct
// ============================================================================

/// A 2D point with floating-point coordinates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    /// Creates a new Point.
    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }

    /// Returns the origin point (0, 0).
    pub fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    /// Calculates the distance from this point to the origin.
    pub fn distance_from_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Calculates the distance between this point and another point.
    pub fn distance_to(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    /// Translates the point by (dx, dy), modifying it in place.
    pub fn translate(&mut self, dx: f64, dy: f64) {
        self.x += dx;
        self.y += dy;
    }
}

// ============================================================================
// COUNTER: Demonstrates method chaining
// ============================================================================

/// A simple counter that demonstrates method chaining with &mut self.
#[derive(Debug, Clone, PartialEq)]
pub struct Counter {
    count: i32,
}

impl Counter {
    /// Creates a new counter starting at 0.
    pub fn new() -> Counter {
        Counter { count: 0 }
    }

    /// Increments the counter by 1 and returns &mut Self for chaining.
    pub fn increment(&mut self) -> &mut Self {
        self.count += 1;
        self
    }

    /// Decrements the counter by 1 and returns &mut Self for chaining.
    pub fn decrement(&mut self) -> &mut Self {
        self.count -= 1;
        self
    }

    /// Returns the current count.
    pub fn get(&self) -> i32 {
        self.count
    }

    /// Resets the counter to 0.
    pub fn reset(&mut self) -> &mut Self {
        self.count = 0;
        self
    }
}

impl Default for Counter {
    fn default() -> Self {
        Counter::new()
    }
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangle_area() {
        let r = Rectangle::new(10, 20);
        assert_eq!(r.area(), 200);
    }

    #[test]
    fn test_rectangle_can_hold() {
        let big = Rectangle::new(30, 50);
        let small = Rectangle::new(10, 20);
        assert!(big.can_hold(&small));
        assert!(!small.can_hold(&big));
    }

    #[test]
    fn test_counter_chaining() {
        let mut c = Counter::new();
        c.increment().increment().increment();
        assert_eq!(c.get(), 3);
    }
}
