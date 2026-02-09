//! # Lab 28: Web Server Data Model
//!
//! Student-facing model and store API for a todo backend.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Todo {
    pub id: u64,
    pub title: String,
    pub completed: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateTodo {
    pub title: String,
    pub completed: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpdateTodo {
    pub title: Option<String>,
    pub completed: Option<bool>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AppError {
    NotFound,
    BadRequest(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!("Format AppError")
    }
}

impl std::error::Error for AppError {}

#[derive(Debug)]
pub struct TodoStore;

impl TodoStore {
    pub fn new() -> Self {
        todo!("Create TodoStore")
    }

    pub fn add_todo(&mut self, create_todo: CreateTodo) -> Todo {
        let _ = create_todo;
        todo!("Add todo")
    }

    pub fn get_todo(&self, id: u64) -> Option<&Todo> {
        let _ = id;
        todo!("Get todo")
    }

    pub fn get_all_todos(&self) -> Vec<Todo> {
        todo!("List all todos")
    }

    pub fn get_all_todos_sorted(&self) -> Vec<Todo> {
        todo!("List all todos sorted")
    }

    pub fn update_todo(&mut self, id: u64, update: UpdateTodo) -> Option<Todo> {
        let _ = (id, update);
        todo!("Update todo")
    }

    pub fn delete_todo(&mut self, id: u64) -> Option<Todo> {
        let _ = id;
        todo!("Delete todo")
    }

    pub fn count(&self) -> usize {
        todo!("Count todos")
    }

    pub fn is_empty(&self) -> bool {
        todo!("Check store empty")
    }

    pub fn completed_count(&self) -> usize {
        todo!("Count completed todos")
    }

    pub fn pending_count(&self) -> usize {
        todo!("Count pending todos")
    }
}

impl Default for TodoStore {
    fn default() -> Self {
        Self::new()
    }
}

pub fn validate_create_todo(create: &CreateTodo) -> Result<(), AppError> {
    let _ = create;
    todo!("Validate create todo")
}

pub fn validate_update_todo(update: &UpdateTodo) -> Result<(), AppError> {
    let _ = update;
    todo!("Validate update todo")
}

#[doc(hidden)]
pub mod solution;
