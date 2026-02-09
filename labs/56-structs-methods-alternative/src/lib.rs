//! # Lab 56: Structs & Methods (Alternative) - Student API
//!
//! Implement structs and methods below. See `src/solution.rs`.

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn new(_width: u32, _height: u32) -> Rectangle {
        todo!("Construct rectangle")
    }

    pub fn square(_size: u32) -> Rectangle {
        todo!("Construct square")
    }

    pub fn area(&self) -> u32 {
        let _ = self;
        todo!("Compute area")
    }

    pub fn perimeter(&self) -> u32 {
        let _ = self;
        todo!("Compute perimeter")
    }

    pub fn can_hold(&self, _other: &Rectangle) -> bool {
        let _ = self;
        todo!("Check strict containment")
    }

    pub fn is_square(&self) -> bool {
        let _ = self;
        todo!("Check equal width and height")
    }

    pub fn scale(&mut self, _factor: u32) {
        let _ = self;
        todo!("Scale dimensions in place")
    }

    pub fn into_tuple(self) -> (u32, u32) {
        todo!("Consume rectangle into tuple")
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(_x: f64, _y: f64) -> Point {
        todo!("Construct point")
    }

    pub fn origin() -> Point {
        todo!("Return (0,0)")
    }

    pub fn distance_from_origin(&self) -> f64 {
        let _ = self;
        todo!("Compute Euclidean distance from origin")
    }

    pub fn distance_to(&self, _other: &Point) -> f64 {
        let _ = self;
        todo!("Compute Euclidean distance to other point")
    }

    pub fn translate(&mut self, _dx: f64, _dy: f64) {
        let _ = self;
        todo!("Translate point by delta")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Counter {
    count: i32,
}

impl Counter {
    pub fn new() -> Counter {
        todo!("Create counter at zero")
    }

    pub fn increment(&mut self) -> &mut Self {
        let _ = self;
        todo!("Increment counter")
    }

    pub fn decrement(&mut self) -> &mut Self {
        let _ = self;
        todo!("Decrement counter")
    }

    pub fn get(&self) -> i32 {
        let _ = self;
        todo!("Read current count")
    }

    pub fn reset(&mut self) -> &mut Self {
        let _ = self;
        todo!("Reset counter to zero")
    }
}

impl Default for Counter {
    fn default() -> Self {
        todo!("Default counter")
    }
}

#[doc(hidden)]
pub mod solution;
