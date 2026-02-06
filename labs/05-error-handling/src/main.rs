//! # Error Handling Demo

use error_handling::solution::*;

fn main() {
    println!("=== Error Handling Demo ===\n");

    // Demo 1: Parsing numbers
    println!("1. Parsing Numbers:");
    match parse_number("42") {
        Ok(n) => println!("   Parsed: {}", n),
        Err(e) => println!("   Error: {}", e),
    }
    match parse_number("not a number") {
        Ok(n) => println!("   Parsed: {}", n),
        Err(e) => println!("   Error: {}\n", e),
    }

    // Demo 2: Division
    println!("2. Division:");
    match divide(10.0, 2.0) {
        Ok(result) => println!("   10 / 2 = {}", result),
        Err(e) => println!("   Error: {}", e),
    }
    match divide(10.0, 0.0) {
        Ok(result) => println!("   10 / 0 = {}", result),
        Err(e) => println!("   Error: {}\n", e),
    }

    // Demo 3: Email validation
    println!("3. Email Validation:");
    let emails = vec!["user@example.com", "invalid.email", "no@domain"];
    for email in emails {
        println!("   {}: {}", email, validate_email(email));
    }

    println!("\n=== Demo Complete! ===");
}
