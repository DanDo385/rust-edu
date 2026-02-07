//! # Traits Basics - Complete Solution

/// Trait defining behavior for geometric shapes.
///
/// Traits are like interfaces in other languages.
/// Any type implementing Shape must provide these methods.
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

/// Implement Shape trait for Circle.
///
/// `impl Shape for Circle` = implement the Shape trait for Circle type
/// Circle must provide implementations for all trait methods.
impl Shape for Circle {
    fn area(&self) -> f64 {
        // Area of circle: π × r²
        // `std::f64::consts::PI` = mathematical constant π
        // `self.radius` = access radius field
        // `self.radius * self.radius` = r²

        std::f64::consts::PI * self.radius * self.radius
    }

    fn perimeter(&self) -> f64 {
        // Perimeter of circle (circumference): 2 × π × r
        2.0 * std::f64::consts::PI * self.radius
    }
}

/// Implement Shape trait for Rectangle.
impl Shape for Rectangle {
    fn area(&self) -> f64 {
        // Area of rectangle: width × height
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        // Perimeter of rectangle: 2 × (width + height)
        2.0 * (self.width + self.height)
    }
}

/// Calculate total area of all shapes.
///
/// ## Parameters
/// - `shapes: &[&dyn Shape]` = slice of trait objects
///   - `&[...]` = borrowed slice
///   - `&dyn Shape` = trait object (dynamic dispatch)
///   - Can hold any type that implements Shape
///   - `dyn` = dynamic, resolved at runtime
///
/// ## How trait objects work
/// - Trait object = pointer to data + pointer to vtable
/// - vtable = virtual function table (method pointers)
/// - Allows polymorphism (different types, same interface)
/// - Runtime cost (dynamic dispatch vs static dispatch)
pub fn total_area(shapes: &[&dyn Shape]) -> f64 {
    // Iterate over all shapes and sum their areas
    // `.iter()` = create iterator
    // `.map(|shape| shape.area())` = calculate area for each shape
    //   - Calls the area() method (different implementation per type)
    //   - Runtime polymorphism via trait objects
    // `.sum()` = add up all areas

    shapes.iter().map(|shape| shape.area()).sum()
}

/// Find the shape with the largest area.
///
/// ## Parameters
/// - `shapes: &[&'a dyn Shape]` = slice of trait objects
///   - `'a` = lifetime annotation
///   - Returned reference has same lifetime as input
///
/// ## Returns
/// - `Option<&'a dyn Shape>` = reference to largest shape, or None if empty
///   - Same lifetime 'a as input (borrowed from input slice)
pub fn largest_shape<'a>(shapes: &[&'a dyn Shape]) -> Option<&'a dyn Shape> {
    // Find shape with maximum area
    // `.iter()` = iterate over trait objects
    // `.max_by()` = find maximum using custom comparator
    //   - `|a, b|` = closure taking two shapes
    //   - `a.area().partial_cmp(&b.area()).unwrap()` = compare areas
    //     - `partial_cmp` for f64 (handles NaN)
    //     - `unwrap()` = assume no NaN values
    // `.copied()` = copy the reference (&&dyn Shape -> &dyn Shape)

    shapes
        .iter()
        .max_by(|a, b| {
            // Handle potential NaN values in f64 comparisons
            // In practice, our shapes should produce finite areas, but we handle NaN gracefully
            a.area().partial_cmp(&b.area()).unwrap_or(std::cmp::Ordering::Equal)
        })
        .copied()
}
