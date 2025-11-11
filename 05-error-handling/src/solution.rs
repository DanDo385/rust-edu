//! # Error Handling - Complete Solution

use std::fmt;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub enum ParseError {
    InvalidFormat(String),
    OutOfRange,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidFormat(s) => write!(f, "Invalid number format: {}", s),
            ParseError::OutOfRange => write!(f, "Number out of range"),
        }
    }
}

impl Error for ParseError {}

#[derive(Debug)]
pub enum MathError {
    DivisionByZero,
}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MathError::DivisionByZero => write!(f, "Division by zero"),
        }
    }
}

impl Error for MathError {}

/// Parse a string to i32, returning detailed errors.
///
/// Uses Rust's ? operator for error propagation.
pub fn parse_number(s: &str) -> Result<i32, ParseError> {
    // `s.trim()` = remove leading/trailing whitespace
    // `.parse::<i32>()` = try to parse as i32
    //   - Returns Result<i32, ParseIntError>
    // `.map_err()` = convert error type
    //   - Transforms ParseIntError to our ParseError
    //   - `|_|` = ignore the original error (underscore means unused)
    //   - Create InvalidFormat with the original string

    s.trim()
        .parse::<i32>()
        .map_err(|_| ParseError::InvalidFormat(s.to_string()))
}

/// Divides two numbers, returning error on division by zero.
pub fn divide(a: f64, b: f64) -> Result<f64, MathError> {
    // Check for division by zero
    // `if b == 0.0` = check if divisor is zero
    //   - For floating point, comparing to 0.0 is safe here
    //   - More robust: `if b.abs() < f64::EPSILON`

    if b == 0.0 {
        // Return error wrapped in Err
        Err(MathError::DivisionByZero)
    } else {
        // Return successful division wrapped in Ok
        Ok(a / b)
    }
}

/// Reads first line of a file.
///
/// Demonstrates ? operator for I/O errors.
pub fn read_first_line(path: &str) -> Result<String, std::io::Error> {
    // Open file - returns Result<File, io::Error>
    // `File::open(path)?` = open file, propagate error if fails
    //   - `?` operator unwraps Ok or returns Err early
    //   - If file doesn't exist, function returns error immediately

    let file = File::open(path)?;

    // Create buffered reader for efficient line reading
    // `BufReader::new(file)` = wrap File in buffered reader
    //   - Reads in chunks, more efficient than reading byte-by-byte

    let reader = BufReader::new(file);

    // Get an iterator over lines
    // `.lines()` = returns iterator yielding Result<String, io::Error>
    //   - Each line is a Result (reading can fail)
    // `.next()` = get first item from iterator
    //   - Returns Option<Result<String, io::Error>>
    //   - None if file is empty
    //   - Some(Ok(line)) if read successful
    //   - Some(Err(e)) if read failed

    let first_line = reader.lines().next();

    // Handle the nested Option<Result>
    match first_line {
        Some(result) => result, // Returns Result<String, io::Error>
        None => Ok(String::new()), // Empty file returns empty string
    }
}

/// Validates email format (simple check).
///
/// Returns bool (no error needed for validation).
pub fn validate_email(email: &str) -> bool {
    // Simple validation: contains @ and . after @
    // `email.contains('@')` = check for @ symbol
    // `&&` = logical AND
    // `.split('@').count() == 2` = exactly one @ symbol
    // `email.contains('.')` = has dot (domain)

    if !email.contains('@') {
        return false;
    }

    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return false;
    }

    // Check that both parts are non-empty
    if parts[0].is_empty() || parts[1].is_empty() {
        return false;
    }

    // Check that there's a dot after the @
    email.rfind('.').map_or(false, |pos| pos > email.find('@').unwrap())
}
