//! # CSV to JSON - Demo

use std::error::Error;

use csv_to_json::solution::{
    csv_to_json, parse_csv_to_employees, parse_nested_csv, persons_to_json_compact,
    students_to_json,
};

fn main() -> Result<(), Box<dyn Error>> {
    println!("=== CSV to JSON Demo ===\n");

    let people_csv = "name,age,city\nAlice,30,New York\nBob,25,San Francisco";
    let people_json = csv_to_json(people_csv)?;
    println!("People JSON:\n{}\n", people_json);

    let compact = persons_to_json_compact(&csv_to_json::solution::parse_csv_to_persons(people_csv)?)?;
    println!("Compact People JSON: {}\n", compact);

    let employee_csv = "employee_id,first_name,last_name,email,active,phone,salary,department\n101,Alice,Smith,alice@example.com,true,555-1234,75000.00,Engineering";
    let employees = parse_csv_to_employees(employee_csv)?;
    println!("Parsed employees: {}", employees.len());

    let student_csv = "id,name,contact.email,contact.phone,grades.math,grades.english,grades.science\n1,Alice,alice@school.edu,555-0001,95,88,92";
    let students = parse_nested_csv(student_csv)?;
    println!("Students JSON:\n{}", students_to_json(&students)?);

    Ok(())
}
