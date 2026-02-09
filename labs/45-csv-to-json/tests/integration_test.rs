// Integration tests for Lab 45: CSV to JSON Converter
//
// Tests CSV parsing, JSON output, field mapping, nested structures,
// error handling, and edge cases.

use csv_to_json::solution::*;

// ============================================================================
// CSV TEST DATA
// ============================================================================

const PERSON_CSV: &str = "\
name,age,city
Alice,30,New York
Bob,25,San Francisco
Charlie,35,Boston
Diana,28,Seattle";

const EMPLOYEE_CSV: &str = "\
employee_id,first_name,last_name,email,active,phone,salary,department
101,Alice,Smith,alice@example.com,true,555-1234,75000.00,Engineering
102,Bob,Jones,bob@example.com,true,,65000.00,Marketing
103,Charlie,Brown,,false,555-5678,55000.00,Sales";

const PRODUCT_CSV_GOOD: &str = "\
id,name,price,quantity
1,Widget,19.99,100
2,Gadget,29.99,50
3,Doohickey,39.99,25";

const PRODUCT_CSV_BAD: &str = "\
id,name,price,quantity
1,Widget,19.99,100
2,Gadget,INVALID,50
3,Doohickey,39.99,INVALID";

const STUDENT_CSV: &str = "\
id,name,contact.email,contact.phone,grades.math,grades.english,grades.science
1,Alice,alice@school.edu,555-0001,95,88,92
2,Bob,bob@school.edu,555-0002,78,85,90
3,Charlie,charlie@school.edu,555-0003,92,94,88";

// ============================================================================
// PERSON CSV PARSING TESTS
// ============================================================================

#[test]
fn test_parse_csv_to_persons() {
    let people = parse_csv_to_persons(PERSON_CSV).unwrap();
    assert_eq!(people.len(), 4);
}

#[test]
fn test_parse_csv_person_fields() {
    let people = parse_csv_to_persons(PERSON_CSV).unwrap();
    assert_eq!(people[0].name, "Alice");
    assert_eq!(people[0].age, 30);
    assert_eq!(people[0].city, "New York");
}

#[test]
fn test_parse_csv_all_persons() {
    let people = parse_csv_to_persons(PERSON_CSV).unwrap();
    assert_eq!(people[1].name, "Bob");
    assert_eq!(people[1].age, 25);
    assert_eq!(people[2].name, "Charlie");
    assert_eq!(people[2].age, 35);
    assert_eq!(people[3].name, "Diana");
    assert_eq!(people[3].city, "Seattle");
}

#[test]
fn test_parse_csv_empty_data() {
    let csv = "name,age,city\n";
    let people = parse_csv_to_persons(csv).unwrap();
    assert!(people.is_empty());
}

#[test]
fn test_parse_csv_single_record() {
    let csv = "name,age,city\nZoe,22,Portland";
    let people = parse_csv_to_persons(csv).unwrap();
    assert_eq!(people.len(), 1);
    assert_eq!(people[0].name, "Zoe");
}

// ============================================================================
// JSON OUTPUT TESTS
// ============================================================================

#[test]
fn test_persons_to_json_pretty() {
    let people = parse_csv_to_persons(PERSON_CSV).unwrap();
    let json = persons_to_json(&people).unwrap();

    // Should contain all names
    assert!(json.contains("Alice"));
    assert!(json.contains("Bob"));
    assert!(json.contains("Charlie"));
    assert!(json.contains("Diana"));

    // Pretty-printed JSON should contain indentation
    assert!(json.contains("  "));
}

#[test]
fn test_persons_to_json_compact() {
    let people = parse_csv_to_persons(PERSON_CSV).unwrap();
    let json = persons_to_json_compact(&people).unwrap();

    // Compact JSON should not contain newlines (within the JSON structure)
    assert!(!json.contains('\n'));
    assert!(json.contains("Alice"));
}

#[test]
fn test_json_roundtrip() {
    let people = parse_csv_to_persons(PERSON_CSV).unwrap();
    let json = persons_to_json(&people).unwrap();

    // Parse JSON back into Person records
    let parsed: Vec<Person> = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed, people);
}

#[test]
fn test_csv_to_json_one_step() {
    let json = csv_to_json(PERSON_CSV).unwrap();
    assert!(json.contains("Alice"));
    assert!(json.contains("30"));
    assert!(json.contains("New York"));

    // Validate it's valid JSON
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert!(parsed.is_array());
    assert_eq!(parsed.as_array().unwrap().len(), 4);
}

#[test]
fn test_empty_persons_to_json() {
    let people: Vec<Person> = Vec::new();
    let json = persons_to_json(&people).unwrap();
    assert_eq!(json, "[]");
}

// ============================================================================
// EMPLOYEE CSV TESTS (SERDE ATTRIBUTES)
// ============================================================================

#[test]
fn test_parse_csv_to_employees() {
    let employees = parse_csv_to_employees(EMPLOYEE_CSV).unwrap();
    assert_eq!(employees.len(), 3);
}

#[test]
fn test_employee_field_rename() {
    let employees = parse_csv_to_employees(EMPLOYEE_CSV).unwrap();
    // CSV has "employee_id" but struct field is "id"
    assert_eq!(employees[0].id, 101);
    // CSV has "first_name" but struct field is "first"
    assert_eq!(employees[0].first, "Alice");
    assert_eq!(employees[0].last, "Smith");
}

#[test]
fn test_employee_optional_fields() {
    let employees = parse_csv_to_employees(EMPLOYEE_CSV).unwrap();
    // Alice has email
    assert_eq!(employees[0].email, Some("alice@example.com".to_string()));
    // Charlie has no email
    assert_eq!(employees[2].email, None);
}

#[test]
fn test_employee_optional_phone() {
    let employees = parse_csv_to_employees(EMPLOYEE_CSV).unwrap();
    // Alice has phone
    assert_eq!(employees[0].phone, Some("555-1234".to_string()));
    // Bob has no phone
    assert_eq!(employees[1].phone, None);
}

#[test]
fn test_employee_flattened_salary_info() {
    let employees = parse_csv_to_employees(EMPLOYEE_CSV).unwrap();
    assert_eq!(employees[0].salary_info.salary, 75000.0);
    assert_eq!(employees[0].salary_info.department, "Engineering");
    assert_eq!(employees[2].salary_info.salary, 55000.0);
    assert_eq!(employees[2].salary_info.department, "Sales");
}

#[test]
fn test_employee_active_field() {
    let employees = parse_csv_to_employees(EMPLOYEE_CSV).unwrap();
    assert!(employees[0].active);
    assert!(!employees[2].active);
}

#[test]
fn test_employee_json_skip_serializing_none_phone() {
    let employees = parse_csv_to_employees(EMPLOYEE_CSV).unwrap();
    let json = serde_json::to_string(&employees[1]).unwrap();
    // Bob has no phone, so "phone" key should not appear
    assert!(!json.contains("phone"));
}

// ============================================================================
// PRODUCT CSV TESTS (ERROR HANDLING)
// ============================================================================

#[test]
fn test_parse_good_products() {
    let (products, errors) = parse_csv_to_products_tolerant(PRODUCT_CSV_GOOD);
    assert_eq!(products.len(), 3);
    assert_eq!(errors, 0);
}

#[test]
fn test_parse_products_with_errors() {
    let (products, errors) = parse_csv_to_products_tolerant(PRODUCT_CSV_BAD);
    // Only the first row is fully valid
    assert_eq!(products.len(), 1);
    assert_eq!(errors, 2);
}

#[test]
fn test_product_field_values() {
    let (products, _) = parse_csv_to_products_tolerant(PRODUCT_CSV_GOOD);
    assert_eq!(products[0].id, 1);
    assert_eq!(products[0].name, "Widget");
    assert_eq!(products[0].price, 19.99);
    assert_eq!(products[0].quantity, 100);
}

#[test]
fn test_products_to_json() {
    let (products, _) = parse_csv_to_products_tolerant(PRODUCT_CSV_GOOD);
    let json = products_to_json(&products).unwrap();
    assert!(json.contains("Widget"));
    assert!(json.contains("19.99"));
}

// ============================================================================
// NESTED STRUCTURE TESTS
// ============================================================================

#[test]
fn test_parse_nested_csv_student_count() {
    let students = parse_nested_csv(STUDENT_CSV).unwrap();
    assert_eq!(students.len(), 3);
}

#[test]
fn test_parse_nested_csv_student_fields() {
    let students = parse_nested_csv(STUDENT_CSV).unwrap();
    assert_eq!(students[0].id, 1);
    assert_eq!(students[0].name, "Alice");
    assert_eq!(students[0].contact.email, "alice@school.edu");
    assert_eq!(students[0].contact.phone, "555-0001");
    assert_eq!(students[0].grades.math, 95);
    assert_eq!(students[0].grades.english, 88);
    assert_eq!(students[0].grades.science, 92);
}

#[test]
fn test_parse_nested_csv_all_students() {
    let students = parse_nested_csv(STUDENT_CSV).unwrap();
    assert_eq!(students[1].name, "Bob");
    assert_eq!(students[1].grades.math, 78);
    assert_eq!(students[2].name, "Charlie");
    assert_eq!(students[2].contact.email, "charlie@school.edu");
}

#[test]
fn test_students_to_json_nested() {
    let students = parse_nested_csv(STUDENT_CSV).unwrap();
    let json = students_to_json(&students).unwrap();

    // Should contain nested fields
    assert!(json.contains("contact"));
    assert!(json.contains("grades"));
    assert!(json.contains("alice@school.edu"));
    assert!(json.contains("95"));

    // Validate it's valid JSON
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert!(parsed.is_array());
}

#[test]
fn test_student_json_roundtrip() {
    let students = parse_nested_csv(STUDENT_CSV).unwrap();
    let json = students_to_json(&students).unwrap();
    let parsed: Vec<Student> = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed, students);
}

// ============================================================================
// EDGE CASES
// ============================================================================

#[test]
fn test_csv_with_commas_in_quoted_fields() {
    let csv = "name,age,city\n\"Smith, John\",40,\"Portland, OR\"";
    let people = parse_csv_to_persons(csv).unwrap();
    assert_eq!(people.len(), 1);
    assert_eq!(people[0].name, "Smith, John");
    assert_eq!(people[0].city, "Portland, OR");
}

#[test]
fn test_csv_with_quotes_in_fields() {
    let csv = "name,age,city\n\"She said \"\"hello\"\"\",25,Boston";
    let people = parse_csv_to_persons(csv).unwrap();
    assert_eq!(people[0].name, "She said \"hello\"");
}

#[test]
fn test_invalid_csv_missing_field() {
    let csv = "name,age,city\nAlice,30";
    let result = parse_csv_to_persons(csv);
    assert!(result.is_err());
}

#[test]
fn test_invalid_csv_wrong_type() {
    let csv = "name,age,city\nAlice,not_a_number,Boston";
    let result = parse_csv_to_persons(csv);
    assert!(result.is_err());
}
