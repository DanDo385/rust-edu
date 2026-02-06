//! Integration tests for variables-types project
//!
//! These tests verify that the solution works correctly.
//! Run with: cargo test -p variables-types

// Import the solution module from our library
use variables_types::solution::*;

// ============================================================================
// TESTS FOR: make_greeting
// ============================================================================

#[test]
fn test_make_greeting_basic() {
    // Test basic greeting creation
    let result = make_greeting("Alice", "Smith");
    assert_eq!(result, "Hello, Alice Smith!");
}

#[test]
fn test_make_greeting_different_names() {
    // Test with different names
    assert_eq!(make_greeting("Bob", "Jones"), "Hello, Bob Jones!");
    assert_eq!(make_greeting("Charlie", "Brown"), "Hello, Charlie Brown!");
}

#[test]
fn test_make_greeting_single_char_names() {
    // Edge case: single character names
    assert_eq!(make_greeting("A", "B"), "Hello, A B!");
}

#[test]
fn test_make_greeting_empty_strings() {
    // Edge case: empty strings (unusual but should work)
    assert_eq!(make_greeting("", ""), "Hello,  !");
    assert_eq!(make_greeting("Alice", ""), "Hello, Alice !");
}

#[test]
fn test_make_greeting_with_spaces() {
    // Edge case: names with spaces
    assert_eq!(make_greeting("Mary Jane", "Watson"), "Hello, Mary Jane Watson!");
}

#[test]
fn test_make_greeting_unicode() {
    // Edge case: Unicode names
    assert_eq!(make_greeting("José", "García"), "Hello, José García!");
    assert_eq!(make_greeting("李", "明"), "Hello, 李 明!");
}

// ============================================================================
// TESTS FOR: celsius_to_fahrenheit
// ============================================================================

#[test]
fn test_celsius_to_fahrenheit_freezing() {
    // Water freezes at 0°C = 32°F
    assert_eq!(celsius_to_fahrenheit(0.0), 32.0);
}

#[test]
fn test_celsius_to_fahrenheit_boiling() {
    // Water boils at 100°C = 212°F
    assert_eq!(celsius_to_fahrenheit(100.0), 212.0);
}

#[test]
fn test_celsius_to_fahrenheit_negative() {
    // Test negative temperatures
    assert_eq!(celsius_to_fahrenheit(-40.0), -40.0);  // -40 is same in both scales!
}

#[test]
fn test_celsius_to_fahrenheit_room_temp() {
    // Room temperature: ~25°C = 77°F
    assert_eq!(celsius_to_fahrenheit(25.0), 77.0);
}

#[test]
fn test_celsius_to_fahrenheit_body_temp() {
    // Body temperature: 37°C = 98.6°F
    let result = celsius_to_fahrenheit(37.0);
    // Use approximate equality for floating point
    assert!((result - 98.6).abs() < 0.01, "Expected ~98.6, got {}", result);
}

#[test]
fn test_celsius_to_fahrenheit_decimals() {
    // Test with decimal values
    assert_eq!(celsius_to_fahrenheit(0.5), 32.9);
    assert_eq!(celsius_to_fahrenheit(10.5), 50.9);
}

// ============================================================================
// TESTS FOR: find_largest
// ============================================================================

#[test]
fn test_find_largest_basic() {
    // Test basic case
    let numbers = vec![3, 7, 2, 9, 1];
    assert_eq!(find_largest(&numbers), Some(9));
}

#[test]
fn test_find_largest_empty() {
    // Test empty slice
    let numbers: Vec<i32> = vec![];
    assert_eq!(find_largest(&numbers), None);
}

#[test]
fn test_find_largest_single_element() {
    // Test single element
    let numbers = vec![42];
    assert_eq!(find_largest(&numbers), Some(42));
}

#[test]
fn test_find_largest_all_same() {
    // Test when all elements are the same
    let numbers = vec![5, 5, 5, 5];
    assert_eq!(find_largest(&numbers), Some(5));
}

#[test]
fn test_find_largest_negative_numbers() {
    // Test with negative numbers
    let numbers = vec![-5, -1, -10, -3];
    assert_eq!(find_largest(&numbers), Some(-1));
}

#[test]
fn test_find_largest_mixed_pos_neg() {
    // Test with mixed positive and negative
    let numbers = vec![-5, 10, -3, 7, -1];
    assert_eq!(find_largest(&numbers), Some(10));
}

#[test]
fn test_find_largest_at_beginning() {
    // Largest element is first
    let numbers = vec![100, 1, 2, 3];
    assert_eq!(find_largest(&numbers), Some(100));
}

#[test]
fn test_find_largest_at_end() {
    // Largest element is last
    let numbers = vec![1, 2, 3, 100];
    assert_eq!(find_largest(&numbers), Some(100));
}

#[test]
fn test_find_largest_duplicates() {
    // Multiple instances of largest
    let numbers = vec![5, 9, 3, 9, 1];
    assert_eq!(find_largest(&numbers), Some(9));
}

#[test]
fn test_find_largest_max_int() {
    // Test with i32::MAX
    let numbers = vec![1, i32::MAX, 3];
    assert_eq!(find_largest(&numbers), Some(i32::MAX));
}

#[test]
fn test_find_largest_min_int() {
    // Test with i32::MIN
    let numbers = vec![i32::MIN, -1, 0];
    assert_eq!(find_largest(&numbers), Some(0));
}

// ============================================================================
// TESTS FOR: count_vowels
// ============================================================================

#[test]
fn test_count_vowels_basic() {
    // Basic test
    assert_eq!(count_vowels("hello"), 2);  // e, o
}

#[test]
fn test_count_vowels_all_vowels() {
    // String with all vowels
    assert_eq!(count_vowels("aeiou"), 5);
    assert_eq!(count_vowels("AEIOU"), 5);
}

#[test]
fn test_count_vowels_no_vowels() {
    // String with no vowels
    assert_eq!(count_vowels("xyz"), 0);
    assert_eq!(count_vowels("bcdfg"), 0);
}

#[test]
fn test_count_vowels_empty() {
    // Empty string
    assert_eq!(count_vowels(""), 0);
}

#[test]
fn test_count_vowels_mixed_case() {
    // Mixed uppercase and lowercase
    assert_eq!(count_vowels("HeLLo WoRLd"), 3);  // e, o, o
}

#[test]
fn test_count_vowels_only_vowels_uppercase() {
    // All uppercase vowels
    assert_eq!(count_vowels("AEIOU"), 5);
}

#[test]
fn test_count_vowels_with_spaces() {
    // String with spaces
    assert_eq!(count_vowels("the quick brown fox"), 5);  // e, u, i, o, o
}

#[test]
fn test_count_vowels_with_punctuation() {
    // String with punctuation
    assert_eq!(count_vowels("Hello, World!"), 3);  // e, o, o
}

#[test]
fn test_count_vowels_repeated_vowels() {
    // Repeated vowels
    assert_eq!(count_vowels("aaa"), 3);
    assert_eq!(count_vowels("aaaeeeiii"), 9);
}

#[test]
fn test_count_vowels_y_not_counted() {
    // 'y' should not be counted as a vowel
    assert_eq!(count_vowels("rhythm"), 0);
    assert_eq!(count_vowels("happy"), 1);  // only 'a'
}

#[test]
fn test_count_vowels_numbers_and_symbols() {
    // Numbers and symbols (no vowels)
    assert_eq!(count_vowels("12345"), 0);
    assert_eq!(count_vowels("!@#$%"), 0);
}

#[test]
fn test_count_vowels_unicode() {
    // Unicode text (accented vowels like é, ï are different characters and don't match a,e,i,o,u)
    assert_eq!(count_vowels("café"), 1);  // Only 'a' counts (é is not plain 'e')
    assert_eq!(count_vowels("naïve"), 2);  // a, e count (but ï is different from i)
}

#[test]
fn test_count_vowels_long_text() {
    // Longer text
    let text = "The quick brown fox jumps over the lazy dog";
    assert_eq!(count_vowels(text), 11);  // e,u,i,o,o,u,o,e,e,a,o
}

// ============================================================================
// INTEGRATION TESTS - Testing multiple functions together
// ============================================================================

#[test]
fn test_integration_greeting_and_vowels() {
    // Create a greeting and count its vowels
    // "Hello, Alice Smith!" has vowels: e, o, A->a, i, e, i = 6 vowels
    let greeting = make_greeting("Alice", "Smith");
    let vowel_count = count_vowels(&greeting);
    assert_eq!(vowel_count, 6);  // e, o, a, i, e, i
}

#[test]
fn test_integration_find_largest_with_temps() {
    // Find the largest temperature and convert it
    let temps_c = vec![0.0, 25.0, 37.0, 100.0, -40.0];
    let temps_i32: Vec<i32> = temps_c.iter().map(|&t| t as i32).collect();

    let largest_c = find_largest(&temps_i32).unwrap();
    let largest_f = celsius_to_fahrenheit(largest_c as f64);

    assert_eq!(largest_c, 100);
    assert_eq!(largest_f, 212.0);
}

// ============================================================================
// PROPERTY-BASED TESTS - More advanced testing
// ============================================================================

#[test]
fn test_property_celsius_fahrenheit_reversible() {
    // Test that conversion is reversible (approximately)
    for c in -100..100 {
        let c_float = c as f64;
        let f = celsius_to_fahrenheit(c_float);
        // Reverse formula: C = (F - 32) * 5/9
        let c_back = (f - 32.0) * 5.0 / 9.0;

        assert!(
            (c_float - c_back).abs() < 0.0001,
            "Conversion not reversible: {} -> {} -> {}",
            c_float,
            f,
            c_back
        );
    }
}

#[test]
fn test_property_find_largest_is_in_slice() {
    // Property: The largest number returned must be in the original slice
    let test_cases = vec![
        vec![1, 2, 3, 4, 5],
        vec![5, 4, 3, 2, 1],
        vec![-1, -2, -3],
        vec![0],
    ];

    for numbers in test_cases {
        if let Some(largest) = find_largest(&numbers) {
            assert!(
                numbers.contains(&largest),
                "Largest {} not found in {:?}",
                largest,
                numbers
            );
        }
    }
}

#[test]
fn test_property_find_largest_greater_equal_all() {
    // Property: The largest number must be >= all other numbers
    let numbers = vec![3, 7, 2, 9, 1, 9, 5];
    let largest = find_largest(&numbers).unwrap();

    for &num in &numbers {
        assert!(
            largest >= num,
            "Largest {} is not >= {}",
            largest,
            num
        );
    }
}

#[test]
fn test_property_count_vowels_bounds() {
    // Property: Vowel count must be between 0 and string length
    let test_strings = vec![
        "hello",
        "aeiou",
        "bcdfg",
        "",
        "The quick brown fox",
    ];

    for text in test_strings {
        let count = count_vowels(text);
        let len = text.chars().count();

        assert!(
            count <= len,
            "Vowel count {} exceeds string length {}",
            count,
            len
        );
    }
}

#[test]
fn test_property_count_vowels_case_insensitive() {
    // Property: Count should be same regardless of case
    let pairs = vec![
        ("hello", "HELLO"),
        ("aeiou", "AEIOU"),
        ("The Quick Brown Fox", "the quick brown fox"),
    ];

    for (lower, upper) in pairs {
        let count_lower = count_vowels(lower);
        let count_upper = count_vowels(upper);

        assert_eq!(
            count_lower, count_upper,
            "Case sensitivity issue: '{}' gave {}, '{}' gave {}",
            lower, count_lower, upper, count_upper
        );
    }
}
