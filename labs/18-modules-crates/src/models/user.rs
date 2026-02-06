// models/user.rs - User model
//
// This file contains the User struct and related functionality.
// It's a submodule of 'models', accessed as: crate::models::user::User

use super::{ValidationError, ValidationResult};

// ============================================================================
// USER MODEL
// ============================================================================

/// Represents a user in the system
///
/// # Examples
///
/// ```
/// use modules_crates::models::User;
///
/// let user = User::new("alice".to_string(), "alice@example.com".to_string());
/// assert_eq!(user.username(), "alice");
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct User {
    // Fields are PRIVATE by default
    // Users of this struct can't access them directly
    // This is encapsulation - we control how data is accessed/modified
    username: String,
    email: String,
    active: bool,
}

impl User {
    /// Creates a new User
    ///
    /// This is the public constructor. Since the fields are private,
    /// this is the only way for external code to create a User.
    ///
    /// # Examples
    ///
    /// ```
    /// use modules_crates::models::User;
    ///
    /// let user = User::new("bob".to_string(), "bob@example.com".to_string());
    /// ```
    pub fn new(username: String, email: String) -> Self {
        User {
            username,
            email,
            active: true,
        }
    }

    /// Creates a new User with validation
    ///
    /// This returns a Result, allowing for validation errors.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError` if username or email is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use modules_crates::models::User;
    ///
    /// let user = User::validated("alice".to_string(), "alice@example.com".to_string());
    /// assert!(user.is_ok());
    ///
    /// let invalid = User::validated("a".to_string(), "invalid-email".to_string());
    /// assert!(invalid.is_err());
    /// ```
    pub fn validated(username: String, email: String) -> ValidationResult<Self> {
        // Validate username
        if username.len() < 3 {
            return Err(ValidationError::TooShort(
                "Username must be at least 3 characters".to_string(),
            ));
        }
        if username.len() > 20 {
            return Err(ValidationError::TooLong(
                "Username must be at most 20 characters".to_string(),
            ));
        }

        // Validate email (simple check)
        if !email.contains('@') {
            return Err(ValidationError::InvalidEmail(
                "Email must contain @".to_string(),
            ));
        }

        Ok(User::new(username, email))
    }

    // ========================================================================
    // GETTER METHODS
    // ========================================================================
    // Since fields are private, we provide getter methods to access them.
    // This gives us control over how data is accessed.

    /// Get the username
    pub fn username(&self) -> &str {
        &self.username
    }

    /// Get the email
    pub fn email(&self) -> &str {
        &self.email
    }

    /// Check if user is active
    pub fn is_active(&self) -> bool {
        self.active
    }

    // ========================================================================
    // SETTER METHODS
    // ========================================================================
    // We can control mutations through setter methods

    /// Set the email (with validation)
    pub fn set_email(&mut self, email: String) -> ValidationResult<()> {
        if !email.contains('@') {
            return Err(ValidationError::InvalidEmail(
                "Email must contain @".to_string(),
            ));
        }
        self.email = email;
        Ok(())
    }

    /// Deactivate the user
    pub fn deactivate(&mut self) {
        self.active = false;
    }

    /// Activate the user
    pub fn activate(&mut self) {
        self.active = true;
    }

    // ========================================================================
    // BUSINESS LOGIC
    // ========================================================================

    /// Generate a display name
    pub fn display_name(&self) -> String {
        format!("{} <{}>", self.username, self.email)
    }

    /// Check if this user matches a username
    pub fn matches_username(&self, username: &str) -> bool {
        self.username == username
    }
}

// ============================================================================
// TRAIT IMPLEMENTATIONS
// ============================================================================

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "User {{ username: {}, email: {}, active: {} }}",
            self.username, self.email, self.active
        )
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let user = User::new("alice".to_string(), "alice@example.com".to_string());
        assert_eq!(user.username(), "alice");
        assert_eq!(user.email(), "alice@example.com");
        assert!(user.is_active());
    }

    #[test]
    fn test_user_validation() {
        // Valid user
        let valid = User::validated("alice".to_string(), "alice@example.com".to_string());
        assert!(valid.is_ok());

        // Username too short
        let short = User::validated("ab".to_string(), "ab@example.com".to_string());
        assert!(short.is_err());

        // Invalid email
        let bad_email = User::validated("alice".to_string(), "not-an-email".to_string());
        assert!(bad_email.is_err());
    }

    #[test]
    fn test_user_deactivation() {
        let mut user = User::new("alice".to_string(), "alice@example.com".to_string());
        assert!(user.is_active());

        user.deactivate();
        assert!(!user.is_active());

        user.activate();
        assert!(user.is_active());
    }

    #[test]
    fn test_email_update() {
        let mut user = User::new("alice".to_string(), "alice@example.com".to_string());

        // Valid email update
        assert!(user.set_email("newemail@example.com".to_string()).is_ok());
        assert_eq!(user.email(), "newemail@example.com");

        // Invalid email update
        assert!(user.set_email("invalid-email".to_string()).is_err());
        // Email should remain unchanged after failed update
        assert_eq!(user.email(), "newemail@example.com");
    }
}

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. Struct fields are private by default - explicit encapsulation
// 2. Provide public constructors (new, validated, etc.)
// 3. Use getter methods for read access to private fields
// 4. Use setter methods for controlled mutation
// 5. Validation belongs in the model
// 6. Use Result<T, E> for operations that can fail
// 7. Implement Display for user-friendly output
// 8. Tests can live in the same file with #[cfg(test)]
// 9. Private fields prevent invalid state
// 10. Public API is intentional and documented
