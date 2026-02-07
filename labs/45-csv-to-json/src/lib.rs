// Lab 45: CSV to JSON Converter
//
// This module provides data structures and conversion functions for transforming
// CSV data to JSON. Demonstrates serde serialization, CSV parsing, and data
// transformation patterns.
//
// Key concepts:
// - Serde derive macros (#[derive(Serialize, Deserialize)])
// - CSV parsing with the `csv` crate
// - JSON serialization with `serde_json`
// - Field mapping with serde attributes
// - Streaming vs. in-memory conversion

use csv;
use serde::{Deserialize, Serialize};
use std::error::Error;

// ============================================================================
// DATA STRUCTURES
// ============================================================================

/// A simple person record for basic CSV-to-JSON conversion.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Person {
    pub name: String,
    pub age: u32,
    pub city: String,
}

/// An employee record demonstrating serde field attributes.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Employee {
    #[serde(rename = "employee_id")]
    pub id: u64,

    #[serde(rename = "first_name")]
    pub first: String,

    #[serde(rename = "last_name")]
    pub last: String,

    /// Optional field (might be missing in CSV)
    pub email: Option<String>,

    /// Field with default value if missing
    #[serde(default)]
    pub active: bool,

    /// Skip serializing if None
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    /// Flattened nested struct
    #[serde(flatten)]
    pub salary_info: SalaryInfo,
}

/// Salary information, flattened into the parent Employee struct.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SalaryInfo {
    #[serde(default)]
    pub salary: f64,

    #[serde(default)]
    pub department: String,
}

/// A product record for error-handling demonstrations.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Product {
    pub id: u64,
    pub name: String,
    pub price: f64,
    pub quantity: u32,
}

/// A log entry for streaming demonstrations.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: String,
    pub message: String,
}

/// Contact information for nested structure examples.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ContactInfo {
    pub email: String,
    pub phone: String,
}

/// Grades for nested structure examples.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Grades {
    pub math: u32,
    pub english: u32,
    pub science: u32,
}

/// A student record with nested fields.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Student {
    pub id: u32,
    pub name: String,
    pub contact: ContactInfo,
    pub grades: Grades,
}

// ============================================================================
// CONVERSION FUNCTIONS
// ============================================================================

/// Parse a CSV string into a vector of Person records.
///
/// The CSV must have headers: name, age, city
pub fn parse_csv_to_persons(csv_data: &str) -> Result<Vec<Person>, Box<dyn Error>> {
    let mut reader = csv::Reader::from_reader(csv_data.as_bytes());
    let mut people = Vec::new();

    for result in reader.deserialize() {
        let person: Person = result?;
        people.push(person);
    }

    Ok(people)
}

/// Convert a vector of Person records to a pretty-printed JSON string.
pub fn persons_to_json(people: &[Person]) -> Result<String, Box<dyn Error>> {
    let json = serde_json::to_string_pretty(people)?;
    Ok(json)
}

/// Convert a vector of Person records to a compact JSON string.
pub fn persons_to_json_compact(people: &[Person]) -> Result<String, Box<dyn Error>> {
    let json = serde_json::to_string(people)?;
    Ok(json)
}

/// Parse a CSV string into a vector of Employee records.
///
/// The CSV must have headers: employee_id, first_name, last_name, email, active, phone, salary, department
pub fn parse_csv_to_employees(csv_data: &str) -> Result<Vec<Employee>, Box<dyn Error>> {
    let mut reader = csv::Reader::from_reader(csv_data.as_bytes());
    let mut employees = Vec::new();

    for result in reader.deserialize() {
        let employee: Employee = result?;
        employees.push(employee);
    }

    Ok(employees)
}

/// Parse a CSV string into a vector of Product records, skipping invalid rows.
///
/// Returns a tuple of (valid_products, error_count).
pub fn parse_csv_to_products_tolerant(csv_data: &str) -> (Vec<Product>, usize) {
    let mut reader = csv::Reader::from_reader(csv_data.as_bytes());
    let mut products = Vec::new();
    let mut error_count = 0;

    for result in reader.deserialize() {
        match result {
            Ok(product) => products.push(product),
            Err(_) => error_count += 1,
        }
    }

    (products, error_count)
}

/// Parse CSV with dot-notation headers into Student records.
///
/// Expected headers: id, name, contact.email, contact.phone, grades.math, grades.english, grades.science
pub fn parse_nested_csv(csv_data: &str) -> Result<Vec<Student>, Box<dyn Error>> {
    let mut reader = csv::Reader::from_reader(csv_data.as_bytes());
    let headers = reader.headers()?.clone();
    let mut students = Vec::new();

    for result in reader.records() {
        let record = result?;

        let get_field = |name: &str| -> String {
            headers
                .iter()
                .position(|h| h == name)
                .and_then(|i| record.get(i))
                .unwrap_or("")
                .to_string()
        };

        let student = Student {
            id: get_field("id").parse().unwrap_or(0),
            name: get_field("name"),
            contact: ContactInfo {
                email: get_field("contact.email"),
                phone: get_field("contact.phone"),
            },
            grades: Grades {
                math: get_field("grades.math").parse().unwrap_or(0),
                english: get_field("grades.english").parse().unwrap_or(0),
                science: get_field("grades.science").parse().unwrap_or(0),
            },
        };

        students.push(student);
    }

    Ok(students)
}

/// One-step conversion: CSV string -> JSON string (pretty-printed).
///
/// Parses Person records from CSV and serializes them as JSON.
pub fn csv_to_json(csv_data: &str) -> Result<String, Box<dyn Error>> {
    let people = parse_csv_to_persons(csv_data)?;
    persons_to_json(&people)
}

/// Convert a vector of Products to a JSON string.
pub fn products_to_json(products: &[Product]) -> Result<String, Box<dyn Error>> {
    let json = serde_json::to_string_pretty(products)?;
    Ok(json)
}

/// Convert a vector of Students to a JSON string.
pub fn students_to_json(students: &[Student]) -> Result<String, Box<dyn Error>> {
    let json = serde_json::to_string_pretty(students)?;
    Ok(json)
}

// ============================================================================
// WHAT RUST DOES UNDER THE HOOD
// ============================================================================
// 1. SERDE DERIVE MACROS
//    - #[derive(Serialize, Deserialize)] generates code at compile-time
//    - No runtime reflection! All type info is known at compile-time
//    - This is why serde is so fast (zero-cost abstraction)
//
// 2. CSV PARSING
//    - csv::Reader uses buffered I/O for efficiency
//    - Zero-copy parsing when possible (borrows from buffer)
//    - Handles quoted fields, escaped characters, different delimiters
//
// 3. JSON SERIALIZATION
//    - serde_json builds a JSON string incrementally
//    - Pretty-printing adds indentation (slower but readable)
//    - Compact format has no whitespace (faster, smaller)
//
// 4. ERROR HANDLING
//    - Result<T, E> forces you to handle errors
//    - Box<dyn Error> allows any error type (trait object)
//    - The ? operator propagates errors up the call stack
