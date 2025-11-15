// utils.rs - Utility functions
//
// This module is PRIVATE (not marked pub in lib.rs)
// It can only be used within this crate, not by external users
//
// This is useful for:
// - Internal helper functions
// - Implementation details
// - Shared utilities that shouldn't be part of the public API

use std::time::{SystemTime, UNIX_EPOCH};

// ============================================================================
// STRING UTILITIES
// ============================================================================

/// Generate a random string (simplified)
///
/// This is NOT pub - only visible within this crate
/// External users of the library cannot call this function
///
/// In a real system, this would use a cryptographically secure RNG
pub(crate) fn generate_random_string(length: usize) -> String {
    // Simplified: use timestamp and repeat to get desired length
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    let base = format!("{:x}", timestamp);
    base.chars()
        .cycle()
        .take(length)
        .collect()
}

/// Convert string to title case
pub(crate) fn title_case(s: &str) -> String {
    s.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase()
                }
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Truncate string to max length
pub(crate) fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}

// ============================================================================
// VALIDATION UTILITIES
// ============================================================================

/// Check if string is a valid email (simplified)
pub(crate) fn is_valid_email(email: &str) -> bool {
    // Simplified validation - just check for @ and .
    email.contains('@') && email.contains('.')
}

/// Check if string is alphanumeric
pub(crate) fn is_alphanumeric(s: &str) -> bool {
    s.chars().all(|c| c.is_alphanumeric())
}

// ============================================================================
// MATH UTILITIES
// ============================================================================

/// Calculate percentage
pub(crate) fn percentage(part: f64, total: f64) -> f64 {
    if total == 0.0 {
        0.0
    } else {
        (part / total) * 100.0
    }
}

/// Clamp a value between min and max
pub(crate) fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

// ============================================================================
// PRIVATE HELPER FUNCTIONS
// ============================================================================
// These are private even within the crate (no pub(crate))

fn _internal_helper() {
    // Only visible within this utils module
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_random_string() {
        let s1 = generate_random_string(16);
        let s2 = generate_random_string(16);

        assert_eq!(s1.len(), 16);
        assert_eq!(s2.len(), 16);
        // Strings should be different (based on timestamp)
        // Note: This test might rarely fail if called in same nanosecond
    }

    #[test]
    fn test_title_case() {
        assert_eq!(title_case("hello world"), "Hello World");
        assert_eq!(title_case("RUST programming"), "Rust Programming");
    }

    #[test]
    fn test_truncate() {
        assert_eq!(truncate("hello", 10), "hello");
        assert_eq!(truncate("hello world", 8), "hello...");
    }

    #[test]
    fn test_is_valid_email() {
        assert!(is_valid_email("test@example.com"));
        assert!(!is_valid_email("invalid-email"));
        assert!(!is_valid_email("no-at-sign.com"));
    }

    #[test]
    fn test_is_alphanumeric() {
        assert!(is_alphanumeric("abc123"));
        assert!(!is_alphanumeric("abc-123"));
        assert!(!is_alphanumeric("hello world"));
    }

    #[test]
    fn test_percentage() {
        assert_eq!(percentage(50.0, 100.0), 50.0);
        assert_eq!(percentage(0.0, 100.0), 0.0);
        assert_eq!(percentage(25.0, 0.0), 0.0);  // Edge case: divide by zero
    }

    #[test]
    fn test_clamp() {
        assert_eq!(clamp(5, 0, 10), 5);
        assert_eq!(clamp(-5, 0, 10), 0);
        assert_eq!(clamp(15, 0, 10), 10);
    }
}

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. Private modules (not pub in lib.rs) are internal only
// 2. pub(crate) makes items visible within the crate but not externally
// 3. Regular pub would make items public to library users (not desired here)
// 4. Utility modules keep helper functions organized
// 5. Keep implementation details private
// 6. External users never see this module's existence
// 7. Tests can still access pub(crate) items
// 8. Private modules are for internal code reuse
