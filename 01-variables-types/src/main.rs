//! # Variables and Types - Interactive Demo
//!
//! This binary demonstrates the functions from our library.
//! Run with: cargo run -p variables-types

// Import all public items from our library
// `use` = keyword to bring items into scope
// `variables_types` = the name of our crate (from Cargo.toml)
// `::` = path separator (like . in other languages for modules)
// `*` = glob import (all public items)
use variables_types::*;

/// Main function - entry point of the program
///
/// `fn` = function keyword
/// `main` = special function name - program starts here
/// `()` = no parameters
/// No return type = returns () (unit type, like void)
fn main() {
    // Print a header
    // `println!` = macro to print with newline
    // The `!` means it's a macro, not a function
    println!("=== Variables and Types Demo ===\n");

    // ============================================================================
    // DEMO 1: make_greeting
    // ============================================================================

    println!("1. Greeting Function:");
    println!("   -------------------");

    // Create some example greetings
    // `let` = declare variable
    // Type is inferred as String (from function return type)
    let greeting1 = solution::make_greeting("Alice", "Smith");
    let greeting2 = solution::make_greeting("Bob", "Jones");
    let greeting3 = solution::make_greeting("Charlie", "Brown");

    // Print them out
    // `{}` = placeholder in format string
    // Gets replaced with the value
    println!("   {}", greeting1);
    println!("   {}", greeting2);
    println!("   {}", greeting3);
    println!();  // Empty line

    // ============================================================================
    // DEMO 2: celsius_to_fahrenheit
    // ============================================================================

    println!("2. Temperature Conversion:");
    println!("   -----------------------");

    // Define some temperatures to convert
    // `let` declarations with explicit type annotations for clarity
    let temps_c: [f64; 5] = [0.0, 25.0, 37.0, 100.0, -40.0];

    // `for` loop to iterate over array elements
    // `&temp` = borrow each element (don't move it)
    for &temp in &temps_c {
        // Convert to Fahrenheit
        let temp_f = solution::celsius_to_fahrenheit(temp);

        // Print with 1 decimal place
        // `:.1` = format specifier for 1 decimal place
        println!("   {:>6.1}°C = {:>6.1}°F", temp, temp_f);
    }
    println!();

    // ============================================================================
    // DEMO 3: find_largest
    // ============================================================================

    println!("3. Find Largest Number:");
    println!("   --------------------");

    // Create some example arrays
    // `vec!` = macro to create a Vec (growable array)
    let numbers1 = vec![3, 7, 2, 9, 1];
    let numbers2 = vec![-5, -1, -10];
    let numbers3: Vec<i32> = vec![];  // Empty vector

    // Helper function to print result
    // Takes a slice and a description
    fn demo_largest(numbers: &[i32], description: &str) {
        // Call our function
        // `&numbers[..]` = convert Vec to slice
        // or just `numbers` works too (Vec coerces to slice)
        let result = solution::find_largest(numbers);

        // `print!` = print without newline
        print!("   {} {:?}: ", description, numbers);

        // `match` = pattern matching on the Option
        // Handles both Some and None cases
        match result {
            Some(largest) => println!("Largest = {}", largest),
            None => println!("Empty (no largest)"),
        }
    }

    demo_largest(&numbers1, "Numbers");
    demo_largest(&numbers2, "Negatives");
    demo_largest(&numbers3, "Empty");
    println!();

    // ============================================================================
    // DEMO 4: count_vowels
    // ============================================================================

    println!("4. Count Vowels:");
    println!("   -------------");

    // Example strings
    // Array of string slices
    let texts = [
        "hello world",
        "RUST",
        "The quick brown fox",
        "bcdfg",  // No vowels
        "",       // Empty
        "AEIOU",  // All vowels
    ];

    // Iterate and count vowels
    for text in &texts {
        let vowel_count = solution::count_vowels(text);

        // Print with padding for alignment
        // `:<20` = left-align in 20 character field
        println!("   {:.<20} {} vowels", text, vowel_count);
    }
    println!();

    // ============================================================================
    // CLOSING MESSAGE
    // ============================================================================

    println!("=== Demo Complete! ===");
    println!("\nNow try:");
    println!("  1. Look at src/solution.rs for detailed explanations");
    println!("  2. Implement your own version in src/lib.rs");
    println!("  3. Run 'cargo test -p variables-types' to check your work");
}
