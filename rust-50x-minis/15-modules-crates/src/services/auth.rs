// services/auth.rs - Authentication service
//
// This module provides authentication functionality.
// It demonstrates how services interact with models and utilities.

use crate::models::User;
use crate::utils;  // Private module - only accessible within this crate

// ============================================================================
// AUTHENTICATION TOKEN
// ============================================================================

/// Represents an authentication token
///
/// The token value is private - users can only create it through authenticate()
#[derive(Debug, Clone)]
pub struct AuthToken {
    value: String,
    user_id: String,
}

impl AuthToken {
    /// Create a new authentication token
    ///
    /// This is pub(crate) - only visible within this crate, not to external users
    pub(crate) fn new(value: String, user_id: String) -> Self {
        AuthToken { value, user_id }
    }

    /// Get the token value
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Get the user ID associated with this token
    pub fn user_id(&self) -> &str {
        &self.user_id
    }

    /// Check if token is valid (simplified - just checks if not empty)
    pub fn is_valid(&self) -> bool {
        !self.value.is_empty()
    }
}

impl std::fmt::Display for AuthToken {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "AuthToken({}...)", &self.value[..8.min(self.value.len())])
    }
}

// ============================================================================
// AUTHENTICATION FUNCTIONS
// ============================================================================

/// Authenticate a user and generate a token
///
/// This is a simplified authentication function for demonstration.
/// In a real system, this would verify credentials, check the database, etc.
///
/// # Examples
///
/// ```
/// use modules_crates::models::User;
/// use modules_crates::services::auth::authenticate;
///
/// let user = User::new("alice".to_string(), "alice@example.com".to_string());
/// let token = authenticate(&user);
/// assert!(token.is_valid());
/// ```
pub fn authenticate(user: &User) -> AuthToken {
    // In a real system, this would:
    // 1. Verify credentials (username/password)
    // 2. Check database
    // 3. Generate cryptographically secure token
    // 4. Store token in session/database

    // For demo, we use our internal utility to generate a token
    let token_value = utils::generate_random_string(32);

    AuthToken::new(token_value, user.username().to_string())
}

/// Verify a token is valid for a user
///
/// This would typically check against a database or session store.
pub fn verify_token(token: &AuthToken, user: &User) -> bool {
    // Simplified check
    token.is_valid() && token.user_id() == user.username()
}

// ============================================================================
// AUTHENTICATION SERVICE
// ============================================================================

/// Authentication service with state
///
/// This demonstrates a stateful service that could maintain sessions,
/// rate limiting, etc.
pub struct AuthService {
    // In a real system, this might hold:
    // - Database connections
    // - Redis connections for sessions
    // - Rate limiting state
    // - etc.
    name: String,
}

impl AuthService {
    /// Create a new authentication service
    pub fn new(name: String) -> Self {
        AuthService { name }
    }

    /// Authenticate and generate a token
    pub fn authenticate(&self, user: &User) -> AuthToken {
        println!("[{}] Authenticating user: {}", self.name, user.username());
        authenticate(user)
    }

    /// Verify a token
    pub fn verify(&self, token: &AuthToken, user: &User) -> bool {
        println!("[{}] Verifying token for: {}", self.name, user.username());
        verify_token(token, user)
    }

    /// Logout (invalidate token)
    pub fn logout(&self, token: &AuthToken) {
        println!("[{}] Logging out token: {}", self.name, token);
        // In a real system, remove token from session store
    }
}

impl Default for AuthService {
    fn default() -> Self {
        AuthService::new("AuthService".to_string())
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authenticate() {
        let user = User::new("alice".to_string(), "alice@example.com".to_string());
        let token = authenticate(&user);

        assert!(token.is_valid());
        assert_eq!(token.user_id(), "alice");
    }

    #[test]
    fn test_verify_token() {
        let user = User::new("alice".to_string(), "alice@example.com".to_string());
        let token = authenticate(&user);

        assert!(verify_token(&token, &user));
    }

    #[test]
    fn test_auth_service() {
        let service = AuthService::default();
        let user = User::new("bob".to_string(), "bob@example.com".to_string());

        let token = service.authenticate(&user);
        assert!(service.verify(&token, &user));

        service.logout(&token);
    }
}

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. Services contain business logic
// 2. Services use models (User) and utilities
// 3. pub(crate) makes items visible only within the crate
// 4. Services can be stateless functions or stateful structs
// 5. Encapsulation: AuthToken value is private
// 6. Use super:: to access parent module items
// 7. Use crate:: to access crate root items
// 8. Services coordinate between different modules
