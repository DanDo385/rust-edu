//! # Lab 13: Generics & Trait Bounds
//!
//! Write once, use many times. Generics let you write code that works with
//! multiple types while maintaining type safety.
//!
//! **Key insight:** Rust doesn't have runtime type information. Instead, the compiler
//! **monomorphizes** your code: it generates a separate version for each concrete type.
//! This means generics are ZERO-COST abstractions.

use std::fmt::Display;

/// Finds the largest item in a slice using generic types with trait bounds.
///
/// **Teaching focus:**
/// - Generic type parameter `<T>`
/// - Trait bounds: `T: PartialOrd + Copy`
/// - Why we need Copy (can't move out of slice when comparing)
///
/// **From the borrow checker's perspective:**
/// - `list: &[T]` borrows the slice (don't own the data)
/// - `item` is Copy (we can copy primitives out of the slice)
/// - Without Copy, we'd get borrow checker errors
/// - Return type `T` means we return a copy of the largest value
///
/// **Monomorphization (what the compiler does):**
/// When you call `largest(&[1, 2, 3])`, Rust generates:
/// ```ignore
/// fn largest_i32(list: &[i32]) -> i32 { ... }
/// ```
/// When you call `largest(&['a', 'b'])`, Rust generates:
/// ```ignore
/// fn largest_char(list: &[char]) -> char { ... }
/// ```
/// Each concrete type gets its own compiled version!
pub fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    // **Why T: Copy?**
    // - We copy `list[0]` and `item` out of the slice
    // - Copy types can be moved without ownership concerns
    // - Without Copy, Rust would prevent us from using the values multiple times
    let mut largest = list[0];
    for &item in list.iter() {
        // &item is needed because iter() gives &T
        // The & before item dereferences the reference
        if item > largest {
            largest = item;
        }
    }
    largest
}

/// A generic point in 2D space with both coordinates the same type.
///
/// **Teaching focus:**
/// - Generic struct definition
/// - Multiple generic parameters (later: different types)
/// - impl blocks for generic types
/// - Ownership: struct OWNS both x and y
#[derive(Debug, Clone, PartialEq)]
pub struct Point<T> {
    /// X coordinate (generic type T)
    pub x: T,
    /// Y coordinate (same type T)
    pub y: T,
}

impl<T> Point<T> {
    /// Creates a new point with both coordinates.
    pub fn new(x: T, y: T) -> Point<T> {
        // **From the borrow checker's perspective:**
        // - x and y are MOVED into the struct
        // - Caller no longer owns them after this call
        // - Point now owns both values
        Point { x, y }
    }

    /// Gets a reference to the x coordinate.
    ///
    /// **Why return &T?**
    /// - Caller borrows the x value
    /// - Caller doesn't take ownership
    /// - Point still owns x and y
    pub fn x(&self) -> &T {
        &self.x
    }

    /// Gets a reference to the y coordinate.
    pub fn y(&self) -> &T {
        &self.y
    }
}

/// Implementation for Point when T supports comparison and display.
impl<T: PartialOrd + Display> Point<T> {
    /// Compares this point with another by distance from origin.
    ///
    /// **Trait bound: PartialOrd + Display**
    /// - PartialOrd: Can compare with `<`, `>`, `==`
    /// - Display: Can convert to string with `{}`
    pub fn compare_distance(&self, other: &Point<T>) -> String {
        // This is a teaching example showing trait bounds
        // In real code, we'd use actual distance calculations
        format!("Comparing {} and {}", self.x, other.x)
    }
}

/// A pair that can hold two different types.
///
/// **Teaching focus:**
/// - Multiple distinct generic parameters (`T` and `U`)
/// - Each parameter can be different type
/// - Ownership: struct owns both values
#[derive(Debug, Clone)]
pub struct Pair<T, U> {
    /// First value (type T)
    pub first: T,
    /// Second value (type U, can be different from T!)
    pub second: U,
}

impl<T, U> Pair<T, U> {
    /// Creates a new pair.
    pub fn new(first: T, second: U) -> Pair<T, U> {
        // **From the borrow checker's perspective:**
        // - first (type T) is MOVED into struct
        // - second (type U) is MOVED into struct
        // - Caller loses ownership of both
        Pair { first, second }
    }

    /// Swaps the order (requires a temporary swap or returns swapped tuple).
    pub fn swap(self) -> Pair<U, T> {
        // **Ownership note:**
        // - Takes ownership of self (consumes it)
        // - Creates new Pair with swapped types
        // - Original Pair is dropped
        Pair {
            first: self.second,
            second: self.first,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_integers() {
        let nums = vec![34, 50, 25, 100, 65];
        assert_eq!(largest(&nums), 100);
    }

    #[test]
    fn test_largest_floats() {
        let nums = vec![1.5, 2.7, 0.3, 9.2];
        assert!(largest(&nums) > 9.0);
    }

    #[test]
    fn test_point_creation() {
        let p = Point::new(5, 10);
        assert_eq!(p.x, 5);
        assert_eq!(p.y, 10);
    }

    #[test]
    fn test_pair_creation() {
        let p = Pair::new(42, "hello");
        assert_eq!(p.first, 42);
        assert_eq!(p.second, "hello");
    }

    #[test]
    fn test_pair_swap() {
        let p = Pair::new(1, "world");
        let swapped = p.swap();
        assert_eq!(swapped.first, "world");
        assert_eq!(swapped.second, 1);
    }
}
