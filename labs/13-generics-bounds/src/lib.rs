//! # Lab 13: Generics and Trait Bounds
//!
//! This module exposes student exercises for generic functions and types.
//! Implement the `todo!()` bodies, then compare your work with `solution.rs`.

use std::fmt::Display;

/// Return the largest value in a non-empty slice.
pub fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    // TODO: Track the current largest item and scan the slice.
    // Hint: This function assumes `list` is non-empty.
    let _ = list;
    todo!("Find largest element in a generic slice")
}

/// A 2D point with both coordinates of the same type.
#[derive(Debug, Clone, PartialEq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    /// Construct a new point.
    pub fn new(x: T, y: T) -> Point<T> {
        // TODO: Return `Point { x, y }`.
        let _ = (x, y);
        todo!("Construct a generic Point")
    }

    /// Borrow the x coordinate.
    pub fn x(&self) -> &T {
        // TODO: Return a shared reference to x.
        todo!("Return x coordinate by reference")
    }

    /// Borrow the y coordinate.
    pub fn y(&self) -> &T {
        // TODO: Return a shared reference to y.
        todo!("Return y coordinate by reference")
    }
}

impl<T: PartialOrd + Display> Point<T> {
    /// Compare two points in a teaching-friendly textual form.
    pub fn compare_distance(&self, other: &Point<T>) -> String {
        // TODO: Build a String using both x values.
        let _ = other;
        todo!("Format a comparison string")
    }
}

/// Pair holding two potentially different types.
#[derive(Debug, Clone)]
pub struct Pair<T, U> {
    pub first: T,
    pub second: U,
}

impl<T, U> Pair<T, U> {
    /// Construct a new pair.
    pub fn new(first: T, second: U) -> Pair<T, U> {
        // TODO: Return `Pair { first, second }`.
        let _ = (first, second);
        todo!("Construct a generic Pair")
    }

    /// Consume self and return a swapped pair.
    pub fn swap(self) -> Pair<U, T> {
        // TODO: Return a new pair with fields reversed.
        todo!("Swap pair element order")
    }
}

#[doc(hidden)]
pub mod solution;
