//! # Structs and Methods

#[derive(Debug, Clone, PartialEq)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Option<Rectangle> {
        todo!()
    }

    pub fn area(&self) -> u32 {
        todo!()
    }

    pub fn perimeter(&self) -> u32 {
        todo!()
    }

    pub fn can_fit(&self, other: &Rectangle) -> bool {
        todo!()
    }

    pub fn scale(&self, factor: u32) -> Rectangle {
        todo!()
    }
}

#[doc(hidden)]
pub mod solution;
