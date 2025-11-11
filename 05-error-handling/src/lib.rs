//! # Error Handling

use std::fmt;
use std::error::Error;

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

pub fn parse_number(s: &str) -> Result<i32, ParseError> {
    todo!()
}

pub fn divide(a: f64, b: f64) -> Result<f64, MathError> {
    todo!()
}

pub fn read_first_line(path: &str) -> Result<String, std::io::Error> {
    todo!()
}

pub fn validate_email(email: &str) -> bool {
    todo!()
}

#[doc(hidden)]
pub mod solution;
