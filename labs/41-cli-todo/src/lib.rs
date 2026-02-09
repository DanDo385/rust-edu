//! # CLI To-Do App - Student API
//!
//! Implement todo list management operations without touching the CLI or persistence.
//! The exercise teaches you how to design an in-memory domain model that can later
//! be serialized to disk by the `main.rs` driver.

/// A single task in the todo list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    id: usize,
    description: String,
    completed: bool,
}

impl Task {
    pub fn new(_id: usize, _description: String) -> Self {
        todo!("Create a new Task with an ID and description")
    }

    pub fn id(&self) -> usize {
        todo!("Return the task ID")
    }

    pub fn description(&self) -> &str {
        todo!("Return the description")
    }

    pub fn is_completed(&self) -> bool {
        todo!("Return completion state")
    }

    pub fn display_string(&self) -> String {
        todo!("Create a human-readable display string for the task")
    }
}

/// In-memory todo list for adding, completing, and removing tasks.
pub struct TodoList {
    _private: (),
}

impl TodoList {
    pub fn new() -> Self {
        todo!("Create an empty TodoList")
    }

    pub fn from_tasks(_tasks: Vec<Task>) -> Self {
        todo!("Build a TodoList from pre-existing tasks")
    }

    pub fn add_task(&mut self, _description: String) -> usize {
        todo!("Add a task and return its ID")
    }

    pub fn complete_task(&mut self, _id: usize) -> Result<(), String> {
        todo!("Mark a task complete")
    }

    pub fn remove_task(&mut self, _id: usize) -> Result<Task, String> {
        todo!("Remove a task by ID")
    }

    pub fn clear_all(&mut self) -> usize {
        todo!("Clear all tasks and return how many were removed")
    }

    pub fn get_tasks(&self) -> &[Task] {
        todo!("Return a slice of all tasks")
    }

    pub fn pending_tasks(&self) -> Vec<&Task> {
        todo!("Return only pending tasks")
    }

    pub fn completed_tasks(&self) -> Vec<&Task> {
        todo!("Return only completed tasks")
    }

    pub fn pending_count(&self) -> usize {
        todo!("Count pending tasks")
    }

    pub fn completed_count(&self) -> usize {
        todo!("Count completed tasks")
    }

    pub fn total_count(&self) -> usize {
        todo!("Return total number of tasks")
    }

    pub fn is_empty(&self) -> bool {
        todo!("Return whether the list is empty")
    }

    pub fn find_task(&self, _id: usize) -> Option<&Task> {
        todo!("Find a task by ID")
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        todo!("Serialize the task list to JSON")
    }

    pub fn from_json(_json: &str) -> Result<Self, serde_json::Error> {
        todo!("Deserialize the task list from JSON")
    }
}

#[doc(hidden)]
pub mod solution;
