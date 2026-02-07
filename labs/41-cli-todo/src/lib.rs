// Lab 41: CLI To-Do App
//
// This module provides the core data structures and logic for a to-do list
// application, completely decoupled from I/O (no file system, no println!).
// The TodoList works entirely in-memory, making it straightforward to test.
//
// Key Concepts:
// - Serde derive macros for serialization/deserialization
// - In-memory data structures that mirror persistent storage patterns
// - Separation of business logic from I/O concerns
// - ID generation and lookup patterns
// - Filtering and counting with iterator adaptors

use serde::{Deserialize, Serialize};

// ============================================================================
// TASK
// ============================================================================

/// A single to-do task with an ID, description, and completion status.
///
/// Derives Serialize and Deserialize so the entire TodoList can be
/// serialized to JSON (or any other serde-supported format) for persistence.
///
/// # Memory Model
/// - id (usize): 8 bytes
/// - description (String): 24 bytes on stack + heap-allocated UTF-8 chars
/// - completed (bool): 1 byte + 7 bytes padding
/// Total stack footprint: ~40 bytes (platform-dependent)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Task {
    id: usize,
    description: String,
    completed: bool,
}

impl Task {
    /// Create a new incomplete task with the given ID and description.
    pub fn new(id: usize, description: String) -> Self {
        Task {
            id,
            description,
            completed: false,
        }
    }

    /// Get the task's unique ID.
    pub fn id(&self) -> usize {
        self.id
    }

    /// Get the task's description.
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Check whether the task is completed.
    pub fn is_completed(&self) -> bool {
        self.completed
    }

    /// Get a display string for the task (e.g., "[1] [ ] Buy groceries").
    pub fn display_string(&self) -> String {
        let status = if self.completed { "x" } else { " " };
        format!("[{}] [{}] {}", self.id, status, self.description)
    }
}

// ============================================================================
// TODO LIST
// ============================================================================

/// An in-memory to-do list that manages a collection of tasks.
///
/// This struct intentionally avoids file I/O so it can be tested
/// without touching the filesystem. The main.rs wires this up to
/// file persistence via serde_json.
///
/// # Memory Model
/// - tasks (Vec<Task>): 24 bytes on stack (ptr, len, cap) + heap for elements
/// - next_id (usize): 8 bytes
/// Total: O(n) where n is the number of tasks
pub struct TodoList {
    tasks: Vec<Task>,
    next_id: usize,
}

impl TodoList {
    /// Create a new, empty to-do list.
    pub fn new() -> Self {
        TodoList {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    /// Create a TodoList from an existing Vec of tasks.
    ///
    /// Sets next_id to one past the maximum existing ID, so new tasks
    /// will not collide with restored tasks.
    pub fn from_tasks(tasks: Vec<Task>) -> Self {
        let next_id = tasks.iter().map(|t| t.id()).max().unwrap_or(0) + 1;
        TodoList { tasks, next_id }
    }

    /// Add a new task with the given description. Returns the assigned ID.
    pub fn add_task(&mut self, description: String) -> usize {
        let id = self.next_id;
        let task = Task::new(id, description);
        self.tasks.push(task);
        self.next_id += 1;
        id
    }

    /// Mark the task with the given ID as completed.
    ///
    /// Returns Ok(()) if the task was found and marked complete,
    /// or Err with a message if not found or already completed.
    pub fn complete_task(&mut self, id: usize) -> Result<(), String> {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id() == id) {
            if task.completed {
                Err(format!("Task #{} is already completed", id))
            } else {
                task.completed = true;
                Ok(())
            }
        } else {
            Err(format!("Task #{} not found", id))
        }
    }

    /// Remove the task with the given ID.
    ///
    /// Returns Ok(removed_task) if found, or Err with a message if not found.
    pub fn remove_task(&mut self, id: usize) -> Result<Task, String> {
        if let Some(pos) = self.tasks.iter().position(|t| t.id() == id) {
            Ok(self.tasks.remove(pos))
        } else {
            Err(format!("Task #{} not found", id))
        }
    }

    /// Remove all tasks and return how many were cleared.
    pub fn clear_all(&mut self) -> usize {
        let count = self.tasks.len();
        self.tasks.clear();
        count
    }

    /// Get a slice of all tasks (both pending and completed).
    pub fn get_tasks(&self) -> &[Task] {
        &self.tasks
    }

    /// Get only pending (incomplete) tasks.
    pub fn pending_tasks(&self) -> Vec<&Task> {
        self.tasks.iter().filter(|t| !t.completed).collect()
    }

    /// Get only completed tasks.
    pub fn completed_tasks(&self) -> Vec<&Task> {
        self.tasks.iter().filter(|t| t.completed).collect()
    }

    /// Count of pending tasks.
    pub fn pending_count(&self) -> usize {
        self.tasks.iter().filter(|t| !t.completed).count()
    }

    /// Count of completed tasks.
    pub fn completed_count(&self) -> usize {
        self.tasks.iter().filter(|t| t.completed).count()
    }

    /// Total number of tasks.
    pub fn total_count(&self) -> usize {
        self.tasks.len()
    }

    /// Check if the list is empty.
    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    /// Find a task by ID.
    pub fn find_task(&self, id: usize) -> Option<&Task> {
        self.tasks.iter().find(|t| t.id() == id)
    }

    /// Serialize the task list to a JSON string.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self.tasks)
    }

    /// Deserialize a task list from a JSON string.
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        let tasks: Vec<Task> = serde_json::from_str(json)?;
        Ok(Self::from_tasks(tasks))
    }
}

impl Default for TodoList {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// WHAT RUST DOES UNDER THE HOOD
// ============================================================================
// 1. SERDE DERIVE
//    #[derive(Serialize, Deserialize)] generates impl blocks at compile time
//    using procedural macros. The generated code is zero-overhead:
//    no reflection, no runtime type information.
//
// 2. VEC OPERATIONS
//    - push: O(1) amortized (doubles capacity when full)
//    - remove(pos): O(n) because elements shift left to fill the gap
//    - iter().find(): O(n) linear scan
//    - iter().position(): O(n) linear scan
//    For large lists, a HashMap<usize, Task> would give O(1) lookup.
//
// 3. RESULT<T, String>
//    Using String for errors is simple but not ideal for production.
//    In production, define a custom error enum implementing std::error::Error.
//    Result forces callers to handle errors explicitly.
//
// 4. ITERATOR ADAPTORS
//    filter(), count(), and collect() are lazy iterators.
//    They are fused into a single pass over the data by the compiler.
//    No intermediate Vec is allocated for filter().count().
//
// 5. MEMORY
//    Vec<Task> is contiguous in memory (good cache locality).
//    Each Task contains a heap-allocated String for the description.
//    clear() deallocates task Strings but keeps Vec capacity for reuse.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_new() {
        let task = Task::new(1, "Test task".to_string());
        assert_eq!(task.id(), 1);
        assert_eq!(task.description(), "Test task");
        assert!(!task.is_completed());
    }

    #[test]
    fn test_task_display_string_incomplete() {
        let task = Task::new(1, "Buy milk".to_string());
        assert_eq!(task.display_string(), "[1] [ ] Buy milk");
    }

    #[test]
    fn test_todo_list_default() {
        let list = TodoList::default();
        assert!(list.is_empty());
        assert_eq!(list.total_count(), 0);
    }
}
