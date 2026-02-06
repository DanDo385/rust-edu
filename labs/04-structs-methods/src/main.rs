//! # Structs and Methods - Interactive Demo

use structs_methods::solution::Rectangle;

fn main() {
    println!("=== Structs and Methods Demo ===\n");

    // Demo 1: Creating rectangles
    println!("1. Creating Rectangles:");
    let rect1 = Rectangle::new(30, 50).unwrap();
    let rect2 = Rectangle::new(10, 20).unwrap();
    println!("   Rectangle 1: {:?}", rect1);
    println!("   Rectangle 2: {:?}\n", rect2);

    // Demo 2: Area and perimeter
    println!("2. Calculations:");
    println!("   Rect1 area: {}", rect1.area());
    println!("   Rect1 perimeter: {}", rect1.perimeter());
    println!("   Rect2 area: {}", rect2.area());
    println!("   Rect2 perimeter: {}\n", rect2.perimeter());

    // Demo 3: Fitting rectangles
    println!("3. Can Fit:");
    println!("   Can rect1 fit rect2? {}", rect1.can_fit(&rect2));
    println!("   Can rect2 fit rect1? {}\n", rect2.can_fit(&rect1));

    // Demo 4: Scaling
    println!("4. Scaling:");
    let scaled = rect2.scale(3);
    println!("   Original rect2: {:?}", rect2);
    println!("   Scaled 3x: {:?}", scaled);
    println!("   Scaled area: {}\n", scaled.area());

    // Demo 5: Invalid rectangle
    println!("5. Validation:");
    match Rectangle::new(0, 10) {
        Some(r) => println!("   Created: {:?}", r),
        None => println!("   Cannot create rectangle with zero dimension"),
    }

    println!("\n=== Demo Complete! ===");
}
