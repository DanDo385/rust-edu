// Project 42: CSV to JSON Converter
//
// Demonstrates serde serialization, CSV parsing, JSON output, and data transformation.
// This is the foundation for ETL pipelines, data integration, and file format conversion.

use csv;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::Write;

fn main() -> Result<(), Box<dyn Error>> {
    println!("=== CSV to JSON Converter ===\n");

    // Example 1: Simple CSV to JSON conversion
    println!("1. Simple CSV to JSON conversion");
    simple_conversion()?;

    println!("\n{}\n", "=".repeat(60));

    // Example 2: Advanced conversion with field mapping
    println!("2. Advanced conversion with field mapping");
    advanced_conversion()?;

    println!("\n{}\n", "=".repeat(60));

    // Example 3: Handling nested structures
    println!("3. Converting to nested JSON structures");
    nested_structures()?;

    println!("\n{}\n", "=".repeat(60));

    // Example 4: Error handling and data validation
    println!("4. Error handling and data validation");
    error_handling_demo()?;

    println!("\n{}\n", "=".repeat(60));

    // Example 5: Streaming large files
    println!("5. Streaming large CSV files");
    streaming_demo()?;

    println!("\n=== Conversion Complete ===");

    Ok(())
}

// ============================================================================
// EXAMPLE 1: SIMPLE CSV TO JSON CONVERSION
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u32,
    city: String,
}

fn simple_conversion() -> Result<(), Box<dyn Error>> {
    // Create sample CSV data
    let csv_data = "\
name,age,city
Alice,30,New York
Bob,25,San Francisco
Charlie,35,Boston
Diana,28,Seattle";

    println!("  Input CSV:");
    println!("  {}\n", csv_data.replace('\n', "\n  "));

    // Parse CSV
    let mut reader = csv::Reader::from_reader(csv_data.as_bytes());
    let mut people = Vec::new();

    for result in reader.deserialize() {
        let person: Person = result?;
        people.push(person);
    }

    println!("  Parsed {} records", people.len());

    // Convert to JSON (pretty-printed)
    let json_pretty = serde_json::to_string_pretty(&people)?;
    println!("\n  JSON output (pretty):");
    println!("  {}", json_pretty.replace('\n', "\n  "));

    // Convert to JSON (compact)
    let json_compact = serde_json::to_string(&people)?;
    println!("\n  JSON output (compact):");
    println!("  {}", json_compact);

    // Save to file
    let mut file = File::create("output_simple.json")?;
    file.write_all(json_pretty.as_bytes())?;
    println!("\n  ✓ Saved to output_simple.json");

    Ok(())
}

// ============================================================================
// EXAMPLE 2: ADVANCED CONVERSION WITH FIELD MAPPING
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
struct Employee {
    // Rename fields to match CSV headers
    #[serde(rename = "employee_id")]
    id: u64,

    #[serde(rename = "first_name")]
    first: String,

    #[serde(rename = "last_name")]
    last: String,

    // Optional field (might be missing in CSV)
    email: Option<String>,

    // Field with default value if missing
    #[serde(default)]
    active: bool,

    // Skip serializing if None
    #[serde(skip_serializing_if = "Option::is_none")]
    phone: Option<String>,

    // Flatten nested struct
    #[serde(flatten)]
    salary_info: SalaryInfo,
}

#[derive(Debug, Serialize, Deserialize)]
struct SalaryInfo {
    #[serde(default)]
    salary: f64,

    #[serde(default)]
    department: String,
}

fn advanced_conversion() -> Result<(), Box<dyn Error>> {
    let csv_data = "\
employee_id,first_name,last_name,email,active,phone,salary,department
101,Alice,Smith,alice@example.com,true,555-1234,75000.00,Engineering
102,Bob,Jones,bob@example.com,true,,65000.00,Marketing
103,Charlie,Brown,,false,555-5678,55000.00,Sales
104,Diana,Wilson,diana@example.com,true,555-9012,80000.00,Engineering";

    println!("  Input CSV:");
    println!("  {}\n", csv_data.lines().take(2).collect::<Vec<_>>().join("\n  "));
    println!("  ...");

    let mut reader = csv::Reader::from_reader(csv_data.as_bytes());
    let mut employees = Vec::new();

    for result in reader.deserialize() {
        let employee: Employee = result?;
        employees.push(employee);
    }

    println!("\n  Parsed {} employee records", employees.len());

    // Convert to JSON
    let json = serde_json::to_string_pretty(&employees)?;
    println!("\n  JSON output (first 500 chars):");
    let preview: String = json.chars().take(500).collect();
    println!("  {}", preview.replace('\n', "\n  "));

    if json.len() > 500 {
        println!("  ...");
    }

    // Save to file
    let mut file = File::create("output_advanced.json")?;
    file.write_all(json.as_bytes())?;
    println!("\n  ✓ Saved to output_advanced.json");

    Ok(())
}

// ============================================================================
// EXAMPLE 3: NESTED JSON STRUCTURES
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
struct Student {
    id: u32,
    name: String,
    contact: ContactInfo,
    grades: Grades,
}

#[derive(Debug, Serialize, Deserialize)]
struct ContactInfo {
    email: String,
    phone: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Grades {
    math: u32,
    english: u32,
    science: u32,
}

fn nested_structures() -> Result<(), Box<dyn Error>> {
    // CSV with dot notation for nested fields
    let csv_data = "\
id,name,contact.email,contact.phone,grades.math,grades.english,grades.science
1,Alice,alice@school.edu,555-0001,95,88,92
2,Bob,bob@school.edu,555-0002,78,85,90
3,Charlie,charlie@school.edu,555-0003,92,94,88";

    println!("  Input CSV (with dot notation):");
    println!("  {}\n", csv_data.lines().next().unwrap());
    println!("  ...");

    // For this example, we'll manually parse since csv crate doesn't
    // automatically handle dot notation
    let students = parse_nested_csv(csv_data)?;

    println!("\n  Parsed {} student records", students.len());

    // Convert to nested JSON
    let json = serde_json::to_string_pretty(&students)?;
    println!("\n  Nested JSON output:");
    println!("  {}", json.replace('\n', "\n  "));

    // Save to file
    let mut file = File::create("output_nested.json")?;
    file.write_all(json.as_bytes())?;
    println!("\n  ✓ Saved to output_nested.json");

    Ok(())
}

// Helper function to parse CSV with dot notation
fn parse_nested_csv(csv_data: &str) -> Result<Vec<Student>, Box<dyn Error>> {
    let mut reader = csv::Reader::from_reader(csv_data.as_bytes());
    let headers = reader.headers()?.clone();
    let mut students = Vec::new();

    for result in reader.records() {
        let record = result?;

        // Helper to get field value
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

// ============================================================================
// EXAMPLE 4: ERROR HANDLING AND DATA VALIDATION
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
struct Product {
    id: u64,
    name: String,
    price: f64,
    quantity: u32,
}

fn error_handling_demo() -> Result<(), Box<dyn Error>> {
    // CSV with some malformed data
    let good_csv = "\
id,name,price,quantity
1,Widget,19.99,100
2,Gadget,29.99,50";

    let bad_csv = "\
id,name,price,quantity
1,Widget,19.99,100
2,Gadget,INVALID,50
3,Doohickey,39.99,INVALID";

    // Process good CSV
    println!("  Processing valid CSV...");
    let mut reader = csv::Reader::from_reader(good_csv.as_bytes());
    let mut count = 0;

    for result in reader.deserialize() {
        match result {
            Ok(product) => {
                let product: Product = product;
                count += 1;
                println!("    ✓ Parsed: {} (${:.2})", product.name, product.price);
            }
            Err(e) => {
                println!("    ✗ Error: {}", e);
            }
        }
    }
    println!("  Successfully parsed {} products\n", count);

    // Process bad CSV (with errors)
    println!("  Processing CSV with errors...");
    let mut reader = csv::Reader::from_reader(bad_csv.as_bytes());
    let mut valid_products = Vec::new();
    let mut error_count = 0;

    for (line_num, result) in reader.deserialize().enumerate() {
        match result {
            Ok(product) => {
                let product: Product = product;
                println!("    ✓ Line {}: Parsed {}", line_num + 2, product.name);
                valid_products.push(product);
            }
            Err(e) => {
                error_count += 1;
                println!("    ✗ Line {}: Error - {}", line_num + 2, e);
            }
        }
    }

    println!("\n  Summary: {} valid, {} errors", valid_products.len(), error_count);

    // Save only valid products
    if !valid_products.is_empty() {
        let json = serde_json::to_string_pretty(&valid_products)?;
        let mut file = File::create("output_valid_only.json")?;
        file.write_all(json.as_bytes())?;
        println!("  ✓ Saved {} valid products to output_valid_only.json", valid_products.len());
    }

    Ok(())
}

// ============================================================================
// EXAMPLE 5: STREAMING LARGE FILES
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
struct LogEntry {
    timestamp: String,
    level: String,
    message: String,
}

fn streaming_demo() -> Result<(), Box<dyn Error>> {
    // Simulate a large CSV file (in practice, this would be from disk)
    let mut csv_data = String::from("timestamp,level,message\n");
    for i in 1..=1000 {
        csv_data.push_str(&format!(
            "2024-01-01T12:00:{:02},INFO,Log message {}\n",
            i % 60,
            i
        ));
    }

    println!("  Processing 1000 log entries (streaming mode)...");

    let mut reader = csv::Reader::from_reader(csv_data.as_bytes());
    let mut file = File::create("output_streaming.json")?;

    // Write JSON array start
    file.write_all(b"[\n")?;

    let mut count = 0;
    let mut first = true;

    // Process records one at a time (streaming)
    for result in reader.deserialize() {
        let entry: LogEntry = result?;

        // Write comma between entries
        if !first {
            file.write_all(b",\n")?;
        }
        first = false;

        // Serialize and write this entry
        let json = serde_json::to_string(&entry)?;
        file.write_all(json.as_bytes())?;

        count += 1;

        // Progress indicator
        if count % 100 == 0 {
            println!("    Processed {} entries...", count);
        }
    }

    // Write JSON array end
    file.write_all(b"\n]")?;

    println!("  ✓ Processed {} log entries", count);
    println!("  ✓ Saved to output_streaming.json");
    println!("  Memory usage: Low (streaming, not loading all into memory)");

    Ok(())
}

// ============================================================================
// WHAT RUST DOES UNDER THE HOOD
// ============================================================================
// 1. SERDE DERIVE MACROS
//    - #[derive(Serialize, Deserialize)] generates code at compile-time
//    - This code implements the Serialize and Deserialize traits
//    - No runtime reflection! All type info is known at compile-time
//    - This is why serde is so fast (zero-cost abstraction)
//
// 2. CSV PARSING
//    - csv::Reader uses buffered I/O for efficiency
//    - Zero-copy parsing when possible (borrows from buffer)
//    - Validates CSV structure while parsing
//    - Handles quoted fields, escaped characters, different delimiters
//
// 3. JSON SERIALIZATION
//    - serde_json builds a JSON string incrementally
//    - Escapes special characters as needed
//    - Pretty-printing adds indentation (slower but readable)
//    - Compact format has no whitespace (faster, smaller)
//
// 4. MEMORY MANAGEMENT
//    - Streaming mode: only one record in memory at a time
//    - Collecting to Vec: all records in memory (use for small datasets)
//    - Strings are heap-allocated, but owned (no GC!)
//    - When Reader is dropped, all buffers are freed automatically
//
// 5. ERROR HANDLING
//    - csv::Reader returns Result<Record, csv::Error>
//    - deserialize() returns Result<T, csv::Error>
//    - Errors include: invalid UTF-8, type mismatches, I/O errors
//    - Using ? operator propagates errors up the call stack

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. Serde is Rust's standard serialization framework
// 2. Derive macros (#[derive(Serialize, Deserialize)]) generate code automatically
// 3. Type safety: compiler ensures data matches your struct schema
// 4. csv crate handles all CSV edge cases (quotes, escapes, delimiters)
// 5. Streaming is crucial for large files (don't load everything into memory)
// 6. Error handling is explicit (no silent failures)
// 7. Zero-cost abstractions: same performance as hand-written code
// 8. Field attributes control serialization behavior (rename, default, skip, etc.)

// ============================================================================
// SERDE ATTRIBUTES REFERENCE
// ============================================================================
// #[serde(rename = "old_name")]          - Map to different field name
// #[serde(default)]                      - Use Default::default() if missing
// #[serde(skip)]                         - Don't serialize/deserialize
// #[serde(skip_serializing)]             - Don't serialize (but do deserialize)
// #[serde(skip_deserializing)]           - Don't deserialize (but do serialize)
// #[serde(skip_serializing_if = "path")] - Skip if condition is true
// #[serde(flatten)]                      - Flatten nested struct into parent
// #[serde(with = "module")]              - Use custom serialize/deserialize
// #[serde(deny_unknown_fields)]          - Error on unexpected fields

// ============================================================================
// COMMON BEGINNER MISTAKES
// ============================================================================
// ❌ Forgetting #[derive(Serialize, Deserialize)]
// ❌ Field name mismatch between CSV headers and struct fields
// ❌ Not handling Option<T> for nullable fields
// ❌ Loading huge files entirely into memory
// ❌ Not validating data types (expecting u32 but CSV has text)
// ❌ Ignoring errors (using unwrap() instead of ?)
// ❌ Using wrong delimiter (comma vs semicolon vs tab)
// ❌ Not handling BOM (Byte Order Mark) in UTF-8 files

// ============================================================================
// PERFORMANCE TIPS
// ============================================================================
// 1. Use streaming for large files (>100MB)
// 2. Disable pretty-printing for production (use to_string not to_string_pretty)
// 3. Use BufReader for file I/O
// 4. Consider bincode for binary serialization (10x faster than JSON)
// 5. Use #[serde(flatten)] carefully (can hurt performance)
// 6. Preallocate Vec capacity if you know size: Vec::with_capacity(n)
// 7. Use &str instead of String when possible (but csv crate owns data)

// ============================================================================
// REAL-WORLD USE CASES
// ============================================================================
// - ETL pipelines: Extract data from CSV, Transform, Load to database
// - API integration: Convert CSV exports to JSON for REST APIs
// - Data migration: Import legacy CSV data into modern systems
// - Report generation: Export database queries to CSV or JSON
// - Configuration files: Convert between formats (CSV ↔ JSON ↔ TOML)
// - Log processing: Parse CSV logs and convert to structured JSON
// - Data analysis: Prepare CSV data for analysis tools
// - Backup systems: Convert application data to portable formats

// ============================================================================
// ADVANCED TOPICS
// ============================================================================
// - Custom deserializers for complex validation
// - Serde with generic types
// - Handling multiple CSV dialects in one program
// - Building CLI tools with clap for file conversion
// - Parallel processing with rayon for huge files
// - Database integration (CSV → SQL inserts)
// - Schema validation with JSON Schema
