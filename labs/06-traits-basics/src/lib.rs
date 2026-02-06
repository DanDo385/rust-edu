//! # Traits Basics

pub trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

#[derive(Debug)]
pub struct Circle {
    pub radius: f64,
}

#[derive(Debug)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        todo!()
    }

    fn perimeter(&self) -> f64 {
        todo!()
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        todo!()
    }

    fn perimeter(&self) -> f64 {
        todo!()
    }
}

pub fn total_area(shapes: &[&dyn Shape]) -> f64 {
    todo!()
}

pub fn largest_shape<'a>(shapes: &[&'a dyn Shape]) -> Option<&'a dyn Shape> {
    todo!()
}

#[doc(hidden)]
pub mod solution;
