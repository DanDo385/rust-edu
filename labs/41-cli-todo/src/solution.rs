//! # CLI To-Do List Solution
//!
//! Fully implemented in-memory task manager with JSON serialization for persistence.
//!
//! ## Classroom Narrative
//!
//! 1. **Modeling tasks**: `Task` owns its ID and description (heap string). The pow of `Vec<Task>` inside `TodoList` owns each task; the vector itself carries a pointer/len/cap on the stack.
//! 2. **State transitions**: Methods that mutate tasks take `&mut self` so the borrow checker guarantees exclusive access to the vector while we flip `completed` flags, remove elements, or bump `next_id`.
//! 3. **Persistence helpers**: Serialization borrows slices/strings immutably; they’re passed via `&self` so no clones occur unless serde demands them.
//!
//! ### Symbol Drill
//!
//! - `&self` is for read-only inspections (`get_tasks`, `pending_count`). These borrows reference the same tasks inside `TodoList`, not copies.
//! - `&mut self` is required for mutations (`add_task`, `complete_task`). It implies mutable exclusivity; you can’t mutate while any shared borrows are alive.
//! - `*` does not appear in this module.
//!
//! ## Step-by-step Teaching Breakdown
//!
//! 1. **Construction**: `new` and `from_tasks` set up the vector and `next_id`. `from_tasks` clones the passed vector because it needs to own it; this demonstrates how ownership must be transferred or cloned.
//! 2. **Mutations**: `add_task` pushes a new task (owned struct) onto the vector. `complete_task` borrows `self.tasks` mutably through `iter_mut()`, ensuring no other reads happen simultaneously.
//! 3. **Queries**: Functions like `pending_tasks` and `total_count` iterate over shared borrows, producing views (`&Task`) without cloning.
//! 4. **Serialization**: `to_json` borrows `self.tasks` immutably and lets serde decide whether to copy data. `from_json` returns a new vector, owning results from the JSON parser.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// A single CLI task with metadata.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Task {
    id: usize,
    description: String,
    completed: bool,
}

impl Task {
    /// Create a new task with the provided ID and description.
    pub fn new(id: usize, description: String) -> Self {
        Task {
            id,
            description,
            completed: false,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn is_completed(&self) -> bool {
        self.completed
    }

    pub fn display_string(&self) -> String {
        let mark = if self.completed { 'x' } else { ' ' };
        format!("[{}] [{}] {}", self.id, mark, self.description)
    }
}

/// An in-memory todo list with ID allocation, lookup helpers, and persistence helpers.
#[derive(Debug, Clone)]
pub struct TodoList {
    tasks: Vec<Task>,
    next_id: usize,
}

impl TodoList {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    pub fn from_tasks(tasks: Vec<Task>) -> Self {
        let next_id = tasks.iter().map(|t| t.id()).max().unwrap_or(0) + 1;
        Self { tasks, next_id }
    }

    pub fn add_task(&mut self, description: String) -> usize {
        let id = self.next_id;
        self.tasks.push(Task::new(id, description));
        self.next_id += 1;
        id
    }

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

    pub fn remove_task(&mut self, id: usize) -> Result<Task, String> {
        if let Some(index) = self.tasks.iter().position(|t| t.id() == id) {
            Ok(self.tasks.remove(index))
        } else {
            Err(format!("Task #{} not found", id))
        }
    }

    pub fn clear_all(&mut self) -> usize {
        let count = self.tasks.len();
        self.tasks.clear();
        count
    }

    pub fn get_tasks(&self) -> &[Task] {
        &self.tasks
    }

    pub fn pending_tasks(&self) -> Vec<&Task> {
        self.tasks.iter().filter(|t| !t.completed).collect()
    }

    pub fn completed_tasks(&self) -> Vec<&Task> {
        self.tasks.iter().filter(|t| t.completed).collect()
    }

    pub fn pending_count(&self) -> usize {
        self.tasks.iter().filter(|t| !t.completed).count()
    }

    pub fn completed_count(&self) -> usize {
        self.tasks.iter().filter(|t| t.completed).count()
    }

    pub fn total_count(&self) -> usize {
        self.tasks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    pub fn find_task(&self, id: usize) -> Option<&Task> {
        self.tasks.iter().find(|t| t.id() == id)
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self.tasks)
    }

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

/// Retains extra helpers for CLI demos (deduplicate before writing file server).
pub fn unique_descriptions(todo: &TodoList) -> HashSet<String> {
    todo.get_tasks().iter().map(|task| task.description().to_string()).collect()
}
