// Project 08: Error Handling
//
// Rust handles errors with Result and Option, not exceptions.
// This makes error handling explicit and type-safe.

use std::fs;
use std::io;
use std::num::ParseIntError;

fn main() {
    println!("=== Error Handling ===\n");

    // ============================================================================
    // RESULT<T, E> - FOR RECOVERABLE ERRORS
    // ============================================================================

    // Reading a file returns Result<String, io::Error>
    let filename = "test.txt";

    match fs::read_to_string(filename) {
        Ok(contents) => println!("File contents: {}", contents),
        Err(e) => println!("Error reading file: {}", e),
    }

    println!();

    // ============================================================================
    // THE ? OPERATOR - ERROR PROPAGATION
    // ============================================================================

    fn read_username_from_file() -> Result<String, io::Error> {
        // ? propagates the error if it occurs
        let contents = fs::read_to_string("username.txt")?;
        Ok(contents.trim().to_string())
    }

    // Equivalent without ?:
    fn read_username_verbose() -> Result<String, io::Error> {
        let contents = match fs::read_to_string("username.txt") {
            Ok(c) => c,
            Err(e) => return Err(e),  // Early return on error
        };
        Ok(contents.trim().to_string())
    }

    match read_username_from_file() {
        Ok(name) => println!("Username: {}", name),
        Err(e) => println!("Could not read username: {}", e),
    }

    println!();

    // ============================================================================
    // UNWRAP AND EXPECT - FOR PROTOTYPING
    // ============================================================================

    // unwrap() returns the value or panics if error
    // Use only when you're SURE it won't fail, or for prototyping

    let config = "DEBUG";
    let _level = config.parse::<String>().unwrap();

    // expect() is like unwrap() but with a custom message
    // let contents = fs::read_to_string("missing.txt").expect("File not found!");

    // Better: Handle the error properly
    let contents = fs::read_to_string("missing.txt")
        .unwrap_or_else(|_| String::from("default content"));

    println!("Contents (with default): {}", contents);

    println!();

    // ============================================================================
    // OPTION<T> - FOR OPTIONAL VALUES
    // ============================================================================

    fn divide(a: f64, b: f64) -> Option<f64> {
        if b == 0.0 {
            None
        } else {
            Some(a / b)
        }
    }

    match divide(10.0, 2.0) {
        Some(result) => println!("10 / 2 = {}", result),
        None => println!("Cannot divide by zero"),
    }

    // Option has many helper methods
    let result = divide(10.0, 2.0)
        .unwrap_or(0.0);  // Default value if None
    println!("Result with default: {}", result);

    let doubled = divide(10.0, 2.0)
        .map(|x| x * 2);  // Transform Some value
    println!("Doubled: {:?}", doubled);

    println!();

    // ============================================================================
    // CUSTOM ERROR TYPES
    // ============================================================================

    #[derive(Debug)]
    enum MathError {
        DivisionByZero,
        NegativeSquareRoot,
    }

    fn safe_divide(a: f64, b: f64) -> Result<f64, MathError> {
        if b == 0.0 {
            Err(MathError::DivisionByZero)
        } else {
            Ok(a / b)
        }
    }

    fn safe_sqrt(x: f64) -> Result<f64, MathError> {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }

    match safe_divide(10.0, 0.0) {
        Ok(result) => println!("Result: {}", result),
        Err(MathError::DivisionByZero) => println!("Error: Cannot divide by zero"),
        Err(MathError::NegativeSquareRoot) => println!("Error: Negative square root"),
    }

    println!();

    // ============================================================================
    // COMBINING MULTIPLE ERROR TYPES
    // ============================================================================

    // Using Box<dyn std::error::Error> for multiple error types
    fn process_number(s: &str) -> Result<i32, Box<dyn std::error::Error>> {
        let num: i32 = s.parse()?;  // ParseIntError
        if num < 0 {
            return Err("Number must be positive".into());
        }
        Ok(num * 2)
    }

    match process_number("42") {
        Ok(n) => println!("Processed: {}", n),
        Err(e) => println!("Error: {}", e),
    }

    match process_number("not a number") {
        Ok(n) => println!("Processed: {}", n),
        Err(e) => println!("Error: {}", e),
    }

    println!();

    // ============================================================================
    // PRACTICAL EXAMPLE: FILE READER CLI
    // ============================================================================

    #[derive(Debug)]
    enum FileError {
        NotFound,
        PermissionDenied,
        Other(String),
    }

    fn read_file_safe(path: &str) -> Result<String, FileError> {
        match fs::read_to_string(path) {
            Ok(contents) => Ok(contents),
            Err(e) => match e.kind() {
                io::ErrorKind::NotFound => Err(FileError::NotFound),
                io::ErrorKind::PermissionDenied => Err(FileError::PermissionDenied),
                _ => Err(FileError::Other(e.to_string())),
            },
        }
    }

    fn process_file(path: &str) {
        match read_file_safe(path) {
            Ok(contents) => {
                println!("File read successfully!");
                println!("Lines: {}", contents.lines().count());
                println!("Chars: {}", contents.len());
            }
            Err(FileError::NotFound) => {
                println!("File '{}' not found", path);
            }
            Err(FileError::PermissionDenied) => {
                println!("Permission denied for '{}'", path);
            }
            Err(FileError::Other(msg)) => {
                println!("Error reading '{}': {}", path, msg);
            }
        }
    }

    process_file("test.txt");
    process_file("missing.txt");

    println!();

    // ============================================================================
    // WHEN TO PANIC
    // ============================================================================

    // panic! for unrecoverable errors (program will crash)
    // Use when:
    // 1. Your program is in a bad state
    // 2. Continuing would be dangerous
    // 3. You're in test code

    fn get_item(index: usize, items: &[i32]) -> i32 {
        if index >= items.len() {
            panic!("Index out of bounds: {}", index);
        }
        items[index]
    }

    let items = vec![1, 2, 3];
    println!("Item 0: {}", get_item(0, &items));

    // This would panic:
    // println!("Item 10: {}", get_item(10, &items));

    println!();
}

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. Result<T, E> for recoverable errors
// 2. Option<T> for optional values
// 3. ? operator propagates errors
// 4. unwrap() panics if error (use sparingly)
// 5. expect() like unwrap with custom message
// 6. Custom error enums for domain errors
// 7. Box<dyn Error> for mixing error types
// 8. panic! for unrecoverable errors
// 9. Errors must be explicitly handled
// 10. No hidden control flow (no exceptions)
