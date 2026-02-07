// Lab 28: Web Server with Axum - Data Model & Store
//
// This module contains the framework-independent business logic for a
// RESTful Todo API. By separating data models and storage from the Axum
// route handlers (in main.rs), we achieve:
//
// 1. Testable business logic without spinning up a server
// 2. No dependency on axum, tokio, or tower in this module
// 3. Clear separation: models + storage (lib.rs) vs. HTTP (main.rs)
//
// The TodoStore pattern mirrors real-world applications where the
// "repository" or "store" layer is independent of the web framework.
//
// # Memory Model
// ```text
// TodoStore (on stack or behind Arc<RwLock<>>):
// ┌─────────────────────────────────┐
// │ todos: HashMap<u64, Todo>       │──────> Heap: Hash table with Todos
// │ next_id: u64                    │
// └─────────────────────────────────┘
//
// Each Todo in the HashMap:
// ┌────────────────────────┐       ┌──────────────────┐
// │ id: u64 (8 bytes)      │       │ "Learn Rust"     │
// │ title: ptr+len+cap     │──────>│ (10 bytes + cap) │
// │ completed: bool (1 b.) │       └──────────────────┘
// └────────────────────────┘
// ```
// In main.rs, the store is wrapped in Arc<RwLock<TodoStore>> for
// thread-safe shared access across async request handlers.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// DATA MODELS
// ============================================================================

/// A single Todo item.
///
/// This struct represents a complete todo record with an assigned ID.
/// It derives Serialize and Deserialize for JSON conversion in the
/// HTTP handlers, plus Clone for extracting owned copies from the store.
///
/// # Serde Integration
/// Serialize: Todo -> JSON (for HTTP responses)
/// Deserialize: JSON -> Todo (for testing and deserialization)
/// In production, you'd typically only Serialize responses and
/// Deserialize requests, but having both is convenient for testing.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Todo {
    pub id: u64,
    pub title: String,
    pub completed: bool,
}

/// Request body for creating a new todo.
///
/// Separate from Todo because the client doesn't provide an ID --
/// the server assigns it. This pattern (separate Create/Update DTOs)
/// prevents clients from accidentally setting server-managed fields.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateTodo {
    pub title: String,
    pub completed: bool,
}

/// Request body for updating an existing todo.
///
/// All fields are Optional because PATCH-style updates should allow
/// partial modifications. The client only sends the fields they want
/// to change.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpdateTodo {
    pub title: Option<String>,
    pub completed: Option<bool>,
}

// ============================================================================
// ERROR TYPES
// ============================================================================

/// Application-specific error type for store operations.
///
/// In main.rs, this is converted to HTTP status codes via Axum's
/// IntoResponse trait. Here in the library, it's a plain enum that
/// can be matched on in tests.
#[derive(Debug, Clone, PartialEq)]
pub enum AppError {
    /// The requested resource was not found (maps to HTTP 404).
    NotFound,
    /// The request was invalid (maps to HTTP 400).
    BadRequest(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::NotFound => write!(f, "Resource not found"),
            AppError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

// ============================================================================
// TODO STORE
// ============================================================================

/// In-memory todo storage with auto-incrementing IDs.
///
/// In a real application, this would be backed by a database (PostgreSQL,
/// SQLite, etc.). The HashMap provides O(1) lookup by ID, which mirrors
/// database primary key lookups.
///
/// # Thread Safety
/// TodoStore itself is NOT thread-safe. In main.rs, it's wrapped in
/// `Arc<RwLock<TodoStore>>` which provides:
/// - Arc: shared ownership across async tasks
/// - RwLock: multiple concurrent readers OR one exclusive writer
///
/// This separation keeps the store simple and testable while allowing
/// the web framework to add concurrency as needed.
#[derive(Debug)]
pub struct TodoStore {
    todos: HashMap<u64, Todo>,
    next_id: u64,
}

impl TodoStore {
    /// Creates a new empty TodoStore.
    ///
    /// IDs start at 1 (not 0) to follow REST API conventions where
    /// ID 0 is often considered invalid or a sentinel value.
    pub fn new() -> Self {
        TodoStore {
            todos: HashMap::new(),
            next_id: 1,
        }
    }

    /// Adds a new todo and returns the created Todo with its assigned ID.
    ///
    /// The ID is auto-incremented from the store's internal counter.
    /// The returned Todo is a clone -- the store retains ownership of
    /// the original.
    ///
    /// # Arguments
    /// * `create_todo` - The title and initial completed status.
    pub fn add_todo(&mut self, create_todo: CreateTodo) -> Todo {
        let todo = Todo {
            id: self.next_id,
            title: create_todo.title,
            completed: create_todo.completed,
        };

        self.todos.insert(self.next_id, todo.clone());
        self.next_id += 1;

        todo
    }

    /// Retrieves a reference to a todo by ID.
    ///
    /// Returns None if no todo with the given ID exists.
    /// The returned reference borrows from the store -- the caller
    /// cannot hold it across a mutable store operation.
    pub fn get_todo(&self, id: u64) -> Option<&Todo> {
        self.todos.get(&id)
    }

    /// Returns cloned copies of all todos.
    ///
    /// The order is NOT guaranteed (HashMap iteration order is random).
    /// In a real API, you'd sort by ID, creation date, or other field.
    pub fn get_all_todos(&self) -> Vec<Todo> {
        self.todos.values().cloned().collect()
    }

    /// Returns all todos sorted by ID (ascending).
    ///
    /// Useful for deterministic output in tests and API responses.
    pub fn get_all_todos_sorted(&self) -> Vec<Todo> {
        let mut todos: Vec<Todo> = self.todos.values().cloned().collect();
        todos.sort_by_key(|t| t.id);
        todos
    }

    /// Updates an existing todo with the provided fields.
    ///
    /// Only the fields present in UpdateTodo (Some variants) are modified.
    /// Fields set to None are left unchanged. This implements PATCH semantics.
    ///
    /// Returns the updated Todo if found, or None if the ID doesn't exist.
    pub fn update_todo(&mut self, id: u64, update: UpdateTodo) -> Option<Todo> {
        if let Some(todo) = self.todos.get_mut(&id) {
            if let Some(title) = update.title {
                todo.title = title;
            }
            if let Some(completed) = update.completed {
                todo.completed = completed;
            }
            Some(todo.clone())
        } else {
            None
        }
    }

    /// Removes a todo by ID and returns it.
    ///
    /// Returns None if no todo with the given ID exists.
    /// The removed Todo is returned as an owned value (moved out of the HashMap).
    pub fn delete_todo(&mut self, id: u64) -> Option<Todo> {
        self.todos.remove(&id)
    }

    /// Returns the number of todos in the store.
    pub fn count(&self) -> usize {
        self.todos.len()
    }

    /// Returns true if the store contains no todos.
    pub fn is_empty(&self) -> bool {
        self.todos.is_empty()
    }

    /// Returns the number of completed todos.
    pub fn completed_count(&self) -> usize {
        self.todos.values().filter(|t| t.completed).count()
    }

    /// Returns the number of pending (incomplete) todos.
    pub fn pending_count(&self) -> usize {
        self.todos.values().filter(|t| !t.completed).count()
    }
}

impl Default for TodoStore {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// VALIDATION HELPERS
// ============================================================================

/// Validates a CreateTodo request.
///
/// Returns Ok(()) if valid, or Err(AppError::BadRequest) with a
/// description of the validation failure.
///
/// Rules:
/// - Title must not be empty (after trimming whitespace)
/// - Title must not exceed 200 characters
pub fn validate_create_todo(create: &CreateTodo) -> Result<(), AppError> {
    if create.title.trim().is_empty() {
        return Err(AppError::BadRequest("Title cannot be empty".to_string()));
    }
    if create.title.len() > 200 {
        return Err(AppError::BadRequest(
            "Title too long (max 200 chars)".to_string(),
        ));
    }
    Ok(())
}

/// Validates an UpdateTodo request.
///
/// Only validates fields that are present (Some). If title is None,
/// it's not checked. This supports PATCH semantics where only provided
/// fields are validated.
pub fn validate_update_todo(update: &UpdateTodo) -> Result<(), AppError> {
    if let Some(ref title) = update.title {
        if title.trim().is_empty() {
            return Err(AppError::BadRequest("Title cannot be empty".to_string()));
        }
        if title.len() > 200 {
            return Err(AppError::BadRequest(
                "Title too long (max 200 chars)".to_string(),
            ));
        }
    }
    Ok(())
}

// ============================================================================
// WHAT RUST DOES UNDER THE HOOD
// ============================================================================
// 1. HASHMAP INTERNALS
//    HashMap<u64, Todo> uses a hash table with Robin Hood hashing.
//    u64 keys hash to a bucket index. Lookup is O(1) amortized.
//    When the table is ~87.5% full, it resizes (doubles capacity).
//
// 2. CLONE SEMANTICS
//    add_todo() and update_todo() return clones because the store
//    retains ownership. Cloning a Todo allocates a new String for
//    the title. In a production system, you might use Arc<str> to
//    avoid cloning the title data.
//
// 3. OPTION AS RESULT
//    get_todo() returns Option<&Todo> -- the borrow checker ensures
//    you can't hold this reference while calling add_todo() or
//    delete_todo() (which take &mut self). This prevents iterator
//    invalidation at compile time.
//
// 4. SERDE DERIVE
//    #[derive(Serialize, Deserialize)] generates efficient conversion
//    code at compile time. No runtime reflection or type inspection.
//    The generated code directly reads/writes struct fields.
//
// 5. SEPARATION FROM FRAMEWORK
//    By keeping serde as the only external dependency, this module
//    can be used with any web framework (axum, actix-web, warp) or
//    even without one (CLI, tests, batch processing).

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. Separate data models from HTTP framework code
// 2. Use separate structs for Create vs. Update DTOs
// 3. Option<T> in UpdateTodo enables PATCH semantics
// 4. HashMap provides O(1) lookups by ID
// 5. Clone returned values to avoid borrow conflicts
// 6. Auto-incrementing IDs start at 1 (REST convention)
// 7. Validation functions return Result<(), AppError>
// 8. Store is framework-agnostic (works with any async runtime)
// 9. Thread safety added externally (Arc<RwLock<>>) by the framework
// 10. Default trait makes store creation ergonomic
