// Integration tests for Lab 46: Declarative Macros
//
// Tests macro invocations, edge cases, and helper functions.
// Macros must be #[macro_export] to be used in integration tests.

use declarative_macros::*;
use declarative_macros::solution::{calculate_add, calculate_multiply, calculate_power};

// ============================================================================
// GREET MACRO TESTS
// ============================================================================

#[test]
fn test_greet_basic() {
    assert_eq!(greet!("World"), "Hello, World!");
}

#[test]
fn test_greet_with_variable() {
    let name = "Rust";
    assert_eq!(greet!(name), "Hello, Rust!");
}

#[test]
fn test_greet_with_expression() {
    assert_eq!(greet!(format!("{} {}", "Alice", "Smith")), "Hello, Alice Smith!");
}

#[test]
fn test_greet_empty_string() {
    assert_eq!(greet!(""), "Hello, !");
}

// ============================================================================
// CALCULATE MACRO TESTS
// ============================================================================

#[test]
fn test_calculate_add() {
    assert_eq!(calculate!(add 5, 3), 8);
}

#[test]
fn test_calculate_add_negative() {
    assert_eq!(calculate!(add -5, 3), -2);
}

#[test]
fn test_calculate_multiply() {
    assert_eq!(calculate!(multiply 5, 3), 15);
}

#[test]
fn test_calculate_multiply_zero() {
    assert_eq!(calculate!(multiply 5, 0), 0);
}

#[test]
fn test_calculate_power() {
    let result = calculate!(power 2, 10);
    assert!((result - 1024.0).abs() < f64::EPSILON);
}

#[test]
fn test_calculate_power_zero_exponent() {
    let result = calculate!(power 5, 0);
    assert!((result - 1.0).abs() < f64::EPSILON);
}

#[test]
fn test_calculate_helper_add() {
    assert_eq!(calculate_add(10, 20), 30);
}

#[test]
fn test_calculate_helper_multiply() {
    assert_eq!(calculate_multiply(6, 7), 42);
}

#[test]
fn test_calculate_helper_power() {
    assert!((calculate_power(3.0, 3.0) - 27.0).abs() < f64::EPSILON);
}

// ============================================================================
// SUM MACRO TESTS
// ============================================================================

#[test]
fn test_sum_single() {
    assert_eq!(sum!(42), 42);
}

#[test]
fn test_sum_two() {
    assert_eq!(sum!(10, 20), 30);
}

#[test]
fn test_sum_five() {
    assert_eq!(sum!(1, 2, 3, 4, 5), 15);
}

#[test]
fn test_sum_negative() {
    assert_eq!(sum!(-1, -2, -3), -6);
}

#[test]
fn test_sum_mixed() {
    assert_eq!(sum!(100, -50, 25, -10), 65);
}

// ============================================================================
// MAKE_VEC MACRO TESTS
// ============================================================================

#[test]
fn test_make_vec_integers() {
    let v = make_vec![1, 2, 3, 4, 5];
    assert_eq!(v, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_make_vec_strings() {
    let v = make_vec!["hello", "world"];
    assert_eq!(v, vec!["hello", "world"]);
}

#[test]
fn test_make_vec_empty() {
    let v: Vec<i32> = make_vec![];
    assert!(v.is_empty());
}

#[test]
fn test_make_vec_single() {
    let v = make_vec![42];
    assert_eq!(v, vec![42]);
}

#[test]
fn test_make_vec_trailing_comma() {
    let v = make_vec![1, 2, 3,];
    assert_eq!(v, vec![1, 2, 3]);
}

// ============================================================================
// HASHMAP MACRO TESTS
// ============================================================================

#[test]
fn test_hashmap_basic() {
    let map = hashmap! {
        "Alice" => 95,
        "Bob" => 87,
    };
    assert_eq!(map.get("Alice"), Some(&95));
    assert_eq!(map.get("Bob"), Some(&87));
}

#[test]
fn test_hashmap_empty() {
    let map: std::collections::HashMap<&str, i32> = hashmap! {};
    assert!(map.is_empty());
}

#[test]
fn test_hashmap_single_entry() {
    let map = hashmap! { "key" => "value" };
    assert_eq!(map.len(), 1);
    assert_eq!(map.get("key"), Some(&"value"));
}

#[test]
fn test_hashmap_trailing_comma() {
    let map = hashmap! {
        "a" => 1,
        "b" => 2,
    };
    assert_eq!(map.len(), 2);
}

#[test]
fn test_hashmap_integer_keys() {
    let map = hashmap! {
        1 => "one",
        2 => "two",
        3 => "three",
    };
    assert_eq!(map.get(&2), Some(&"two"));
}

// ============================================================================
// ASSERT_BETWEEN MACRO TESTS
// ============================================================================

#[test]
fn test_assert_between_within_range() {
    assert_between!(50, 0, 100);
}

#[test]
fn test_assert_between_at_minimum() {
    assert_between!(0, 0, 100);
}

#[test]
fn test_assert_between_at_maximum() {
    assert_between!(100, 0, 100);
}

#[test]
fn test_assert_between_equal_bounds() {
    assert_between!(5, 5, 5);
}

#[test]
#[should_panic(expected = "Assertion failed")]
fn test_assert_between_below_range() {
    assert_between!(-1, 0, 100);
}

#[test]
#[should_panic(expected = "Assertion failed")]
fn test_assert_between_above_range() {
    assert_between!(101, 0, 100);
}

// ============================================================================
// COUNT_TOKENS MACRO TESTS
// ============================================================================

#[test]
fn test_count_tokens_zero() {
    assert_eq!(count_tokens!(), 0);
}

#[test]
fn test_count_tokens_one() {
    assert_eq!(count_tokens!(x), 1);
}

#[test]
fn test_count_tokens_five() {
    assert_eq!(count_tokens!(a b c d e), 5);
}

#[test]
fn test_count_tokens_with_operators() {
    // Each token is counted: 1 + 2 + 3 = 5 tokens
    assert_eq!(count_tokens!(1 + 2 + 3), 5);
}

// ============================================================================
// FANCY_SUM MACRO TESTS
// ============================================================================

#[test]
fn test_fancy_sum_single() {
    assert_eq!(fancy_sum!(10), 10);
}

#[test]
fn test_fancy_sum_multiple() {
    assert_eq!(fancy_sum!(1, 2, 3, 4, 5), 15);
}

#[test]
fn test_fancy_sum_two() {
    assert_eq!(fancy_sum!(100, 200), 300);
}

// ============================================================================
// STRING ENUM MACRO TESTS
// ============================================================================

string_enum! {
    Color {
        Red,
        Green,
        Blue,
        Yellow,
    }
}

#[test]
fn test_string_enum_to_str() {
    assert_eq!(Color::Red.to_str(), "Red");
    assert_eq!(Color::Green.to_str(), "Green");
    assert_eq!(Color::Blue.to_str(), "Blue");
    assert_eq!(Color::Yellow.to_str(), "Yellow");
}

#[test]
fn test_string_enum_from_str() {
    assert_eq!(Color::from_str("Red"), Some(Color::Red));
    assert_eq!(Color::from_str("Green"), Some(Color::Green));
    assert_eq!(Color::from_str("Blue"), Some(Color::Blue));
}

#[test]
fn test_string_enum_from_str_invalid() {
    assert_eq!(Color::from_str("Purple"), None);
    assert_eq!(Color::from_str(""), None);
    assert_eq!(Color::from_str("red"), None); // case-sensitive
}

#[test]
fn test_string_enum_all_variants() {
    let variants = Color::all_variants();
    assert_eq!(variants.len(), 4);
    assert!(variants.contains(&"Red"));
    assert!(variants.contains(&"Green"));
    assert!(variants.contains(&"Blue"));
    assert!(variants.contains(&"Yellow"));
}

#[test]
fn test_string_enum_equality() {
    let a = Color::Red;
    let b = Color::Red;
    let c = Color::Blue;

    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn test_string_enum_clone() {
    let a = Color::Green;
    let b = a;
    assert_eq!(a, b);
}

// ============================================================================
// TIME_IT MACRO TESTS
// ============================================================================

#[test]
fn test_time_it_returns_result() {
    let (result, _duration) = time_it!({
        42
    });
    assert_eq!(result, 42);
}

#[test]
fn test_time_it_measures_duration() {
    let (_result, duration) = time_it!({
        std::thread::sleep(std::time::Duration::from_millis(10));
        "done"
    });
    assert!(duration.as_millis() >= 5); // Allow some tolerance
}

#[test]
fn test_time_it_with_computation() {
    let (result, duration) = time_it!({
        let sum: i32 = (1..=100).sum();
        sum
    });
    assert_eq!(result, 5050);
    // Should be very fast (sub-millisecond)
    assert!(duration.as_secs() == 0);
}

// ============================================================================
// CONFIG DSL MACRO TESTS
// ============================================================================

config! {
    TestConfig {
        host: String = "localhost".to_string(),
        port: u16 = 8080,
        max_connections: usize = 100,
    }
}

#[test]
fn test_config_defaults() {
    let cfg = TestConfig::new();
    assert_eq!(cfg.host, "localhost");
    assert_eq!(cfg.port, 8080);
    assert_eq!(cfg.max_connections, 100);
}

#[test]
fn test_config_default_trait() {
    let cfg = TestConfig::default();
    assert_eq!(cfg.host, "localhost");
    assert_eq!(cfg.port, 8080);
}

#[test]
fn test_config_debug() {
    let cfg = TestConfig::new();
    let debug = format!("{:?}", cfg);
    assert!(debug.contains("TestConfig"));
    assert!(debug.contains("localhost"));
    assert!(debug.contains("8080"));
}

#[test]
fn test_config_field_access() {
    let mut cfg = TestConfig::new();
    cfg.port = 3000;
    cfg.host = "0.0.0.0".to_string();
    assert_eq!(cfg.port, 3000);
    assert_eq!(cfg.host, "0.0.0.0");
}

config! {
    MinimalConfig {
        debug: bool = false,
    }
}

#[test]
fn test_config_single_field() {
    let cfg = MinimalConfig::new();
    assert!(!cfg.debug);
}
