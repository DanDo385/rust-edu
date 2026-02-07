// Lab 41: CLI To-Do App - Integration Tests
//
// These tests verify the TodoList and Task types work correctly
// in-memory without any file I/O. All tests are deterministic and fast.

use cli_todo::*;

// ============================================================================
// TASK CONSTRUCTION AND PROPERTIES
// ============================================================================

#[test]
fn test_task_new_is_not_completed() {
    let task = Task::new(1, "Do homework".to_string());
    assert!(!task.is_completed());
}

#[test]
fn test_task_id_and_description() {
    let task = Task::new(42, "Buy groceries".to_string());
    assert_eq!(task.id(), 42);
    assert_eq!(task.description(), "Buy groceries");
}

#[test]
fn test_task_display_string_pending() {
    let task = Task::new(3, "Clean house".to_string());
    assert_eq!(task.display_string(), "[3] [ ] Clean house");
}

#[test]
fn test_task_clone() {
    let task = Task::new(1, "Original".to_string());
    let cloned = task.clone();
    assert_eq!(task, cloned);
}

#[test]
fn test_task_equality() {
    let a = Task::new(1, "Same task".to_string());
    let b = Task::new(1, "Same task".to_string());
    assert_eq!(a, b);
}

// ============================================================================
// TODO LIST CREATION
// ============================================================================

#[test]
fn test_new_todo_list_is_empty() {
    let list = TodoList::new();
    assert!(list.is_empty());
    assert_eq!(list.total_count(), 0);
    assert_eq!(list.pending_count(), 0);
    assert_eq!(list.completed_count(), 0);
}

#[test]
fn test_default_todo_list_is_empty() {
    let list = TodoList::default();
    assert!(list.is_empty());
}

#[test]
fn test_from_tasks() {
    let tasks = vec![
        Task::new(5, "Task five".to_string()),
        Task::new(10, "Task ten".to_string()),
    ];
    let list = TodoList::from_tasks(tasks);
    assert_eq!(list.total_count(), 2);

    // Adding a new task should get ID 11 (max existing + 1)
    let mut list = list;
    let new_id = list.add_task("New task".to_string());
    assert_eq!(new_id, 11);
}

#[test]
fn test_from_tasks_empty() {
    let list = TodoList::from_tasks(vec![]);
    assert!(list.is_empty());
}

// ============================================================================
// ADD TASK
// ============================================================================

#[test]
fn test_add_task_returns_sequential_ids() {
    let mut list = TodoList::new();
    let id1 = list.add_task("First".to_string());
    let id2 = list.add_task("Second".to_string());
    let id3 = list.add_task("Third".to_string());

    assert_eq!(id1, 1);
    assert_eq!(id2, 2);
    assert_eq!(id3, 3);
}

#[test]
fn test_add_task_increments_count() {
    let mut list = TodoList::new();
    assert_eq!(list.total_count(), 0);

    list.add_task("Task 1".to_string());
    assert_eq!(list.total_count(), 1);

    list.add_task("Task 2".to_string());
    assert_eq!(list.total_count(), 2);
}

#[test]
fn test_add_task_is_pending_by_default() {
    let mut list = TodoList::new();
    list.add_task("New task".to_string());

    assert_eq!(list.pending_count(), 1);
    assert_eq!(list.completed_count(), 0);
}

#[test]
fn test_add_task_with_empty_description() {
    let mut list = TodoList::new();
    let id = list.add_task(String::new());
    let task = list.find_task(id).unwrap();
    assert_eq!(task.description(), "");
}

#[test]
fn test_add_task_with_long_description() {
    let mut list = TodoList::new();
    let long_desc = "A".repeat(10_000);
    let id = list.add_task(long_desc.clone());
    let task = list.find_task(id).unwrap();
    assert_eq!(task.description(), long_desc);
}

// ============================================================================
// COMPLETE TASK
// ============================================================================

#[test]
fn test_complete_task_success() {
    let mut list = TodoList::new();
    let id = list.add_task("Do laundry".to_string());

    let result = list.complete_task(id);
    assert!(result.is_ok());

    let task = list.find_task(id).unwrap();
    assert!(task.is_completed());
}

#[test]
fn test_complete_task_updates_counts() {
    let mut list = TodoList::new();
    list.add_task("Task A".to_string());
    let id_b = list.add_task("Task B".to_string());

    assert_eq!(list.pending_count(), 2);
    assert_eq!(list.completed_count(), 0);

    list.complete_task(id_b).unwrap();

    assert_eq!(list.pending_count(), 1);
    assert_eq!(list.completed_count(), 1);
}

#[test]
fn test_complete_task_not_found() {
    let mut list = TodoList::new();
    let result = list.complete_task(999);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not found"));
}

#[test]
fn test_complete_task_already_completed() {
    let mut list = TodoList::new();
    let id = list.add_task("Done task".to_string());

    list.complete_task(id).unwrap();
    let result = list.complete_task(id);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("already completed"));
}

#[test]
fn test_complete_all_tasks() {
    let mut list = TodoList::new();
    let id1 = list.add_task("A".to_string());
    let id2 = list.add_task("B".to_string());
    let id3 = list.add_task("C".to_string());

    list.complete_task(id1).unwrap();
    list.complete_task(id2).unwrap();
    list.complete_task(id3).unwrap();

    assert_eq!(list.pending_count(), 0);
    assert_eq!(list.completed_count(), 3);
}

// ============================================================================
// REMOVE TASK
// ============================================================================

#[test]
fn test_remove_task_success() {
    let mut list = TodoList::new();
    let id = list.add_task("Temporary".to_string());

    let removed = list.remove_task(id).unwrap();
    assert_eq!(removed.description(), "Temporary");
    assert_eq!(list.total_count(), 0);
}

#[test]
fn test_remove_task_not_found() {
    let mut list = TodoList::new();
    let result = list.remove_task(999);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not found"));
}

#[test]
fn test_remove_task_preserves_others() {
    let mut list = TodoList::new();
    let id1 = list.add_task("Keep".to_string());
    let id2 = list.add_task("Remove".to_string());
    let id3 = list.add_task("Also keep".to_string());

    list.remove_task(id2).unwrap();

    assert_eq!(list.total_count(), 2);
    assert!(list.find_task(id1).is_some());
    assert!(list.find_task(id2).is_none());
    assert!(list.find_task(id3).is_some());
}

#[test]
fn test_remove_completed_task() {
    let mut list = TodoList::new();
    let id = list.add_task("Will complete then remove".to_string());
    list.complete_task(id).unwrap();

    let removed = list.remove_task(id).unwrap();
    assert!(removed.is_completed());
    assert_eq!(list.completed_count(), 0);
}

// ============================================================================
// CLEAR ALL
// ============================================================================

#[test]
fn test_clear_all_returns_count() {
    let mut list = TodoList::new();
    list.add_task("A".to_string());
    list.add_task("B".to_string());
    list.add_task("C".to_string());

    let cleared = list.clear_all();
    assert_eq!(cleared, 3);
}

#[test]
fn test_clear_all_empties_list() {
    let mut list = TodoList::new();
    list.add_task("Task".to_string());
    list.clear_all();

    assert!(list.is_empty());
    assert_eq!(list.total_count(), 0);
    assert_eq!(list.pending_count(), 0);
    assert_eq!(list.completed_count(), 0);
}

#[test]
fn test_clear_all_on_empty_list() {
    let mut list = TodoList::new();
    let cleared = list.clear_all();
    assert_eq!(cleared, 0);
}

#[test]
fn test_clear_then_add() {
    let mut list = TodoList::new();
    list.add_task("Old task".to_string());
    list.clear_all();

    // IDs should continue incrementing (not reset)
    let id = list.add_task("New task".to_string());
    assert!(id > 1);
}

// ============================================================================
// GET TASKS AND FILTERING
// ============================================================================

#[test]
fn test_get_tasks_returns_all() {
    let mut list = TodoList::new();
    list.add_task("A".to_string());
    list.add_task("B".to_string());

    let tasks = list.get_tasks();
    assert_eq!(tasks.len(), 2);
}

#[test]
fn test_pending_tasks_filter() {
    let mut list = TodoList::new();
    let id1 = list.add_task("Pending".to_string());
    let id2 = list.add_task("Done".to_string());
    let _id3 = list.add_task("Also pending".to_string());

    list.complete_task(id2).unwrap();

    let pending = list.pending_tasks();
    assert_eq!(pending.len(), 2);
    assert!(pending.iter().all(|t| !t.is_completed()));

    // Verify specific tasks
    assert!(pending.iter().any(|t| t.id() == id1));
}

#[test]
fn test_completed_tasks_filter() {
    let mut list = TodoList::new();
    list.add_task("Pending".to_string());
    let id2 = list.add_task("Done".to_string());

    list.complete_task(id2).unwrap();

    let completed = list.completed_tasks();
    assert_eq!(completed.len(), 1);
    assert_eq!(completed[0].id(), id2);
    assert!(completed[0].is_completed());
}

// ============================================================================
// FIND TASK
// ============================================================================

#[test]
fn test_find_task_exists() {
    let mut list = TodoList::new();
    let id = list.add_task("Findable".to_string());

    let task = list.find_task(id);
    assert!(task.is_some());
    assert_eq!(task.unwrap().description(), "Findable");
}

#[test]
fn test_find_task_not_exists() {
    let list = TodoList::new();
    assert!(list.find_task(1).is_none());
}

#[test]
fn test_find_task_after_remove() {
    let mut list = TodoList::new();
    let id = list.add_task("Temporary".to_string());
    list.remove_task(id).unwrap();
    assert!(list.find_task(id).is_none());
}

// ============================================================================
// JSON SERIALIZATION / DESERIALIZATION
// ============================================================================

#[test]
fn test_to_json_and_from_json_roundtrip() {
    let mut list = TodoList::new();
    list.add_task("Task one".to_string());
    list.add_task("Task two".to_string());
    let id3 = list.add_task("Task three".to_string());
    list.complete_task(id3).unwrap();

    let json = list.to_json().unwrap();
    let restored = TodoList::from_json(&json).unwrap();

    assert_eq!(restored.total_count(), 3);
    assert_eq!(restored.pending_count(), 2);
    assert_eq!(restored.completed_count(), 1);

    // Verify task data survived roundtrip
    let task = restored.find_task(id3).unwrap();
    assert_eq!(task.description(), "Task three");
    assert!(task.is_completed());
}

#[test]
fn test_from_json_empty_array() {
    let list = TodoList::from_json("[]").unwrap();
    assert!(list.is_empty());
}

#[test]
fn test_from_json_invalid_returns_error() {
    let result = TodoList::from_json("not valid json");
    assert!(result.is_err());
}

#[test]
fn test_to_json_empty_list() {
    let list = TodoList::new();
    let json = list.to_json().unwrap();
    assert_eq!(json, "[]");
}

#[test]
fn test_json_roundtrip_preserves_ids() {
    let mut list = TodoList::new();
    list.add_task("A".to_string());
    list.add_task("B".to_string());

    let json = list.to_json().unwrap();
    let restored = TodoList::from_json(&json).unwrap();

    // New task should get id 3 (max existing id 2 + 1)
    let mut restored = restored;
    let new_id = restored.add_task("C".to_string());
    assert_eq!(new_id, 3);
}

// ============================================================================
// DISPLAY STRING
// ============================================================================

#[test]
fn test_task_display_string_completed() {
    let mut list = TodoList::new();
    let id = list.add_task("Finished task".to_string());
    list.complete_task(id).unwrap();

    let task = list.find_task(id).unwrap();
    assert_eq!(task.display_string(), format!("[{}] [x] Finished task", id));
}

// ============================================================================
// COMPLEX SCENARIOS
// ============================================================================

#[test]
fn test_add_complete_remove_workflow() {
    let mut list = TodoList::new();

    // Add several tasks
    let id1 = list.add_task("Buy groceries".to_string());
    let id2 = list.add_task("Clean house".to_string());
    let id3 = list.add_task("Write code".to_string());
    let id4 = list.add_task("Read book".to_string());

    assert_eq!(list.total_count(), 4);
    assert_eq!(list.pending_count(), 4);

    // Complete some
    list.complete_task(id1).unwrap();
    list.complete_task(id3).unwrap();

    assert_eq!(list.pending_count(), 2);
    assert_eq!(list.completed_count(), 2);

    // Remove one pending and one completed
    list.remove_task(id2).unwrap();
    list.remove_task(id1).unwrap();

    assert_eq!(list.total_count(), 2);
    assert_eq!(list.pending_count(), 1);
    assert_eq!(list.completed_count(), 1);

    // Verify remaining
    assert!(list.find_task(id3).unwrap().is_completed());
    assert!(!list.find_task(id4).unwrap().is_completed());
}

#[test]
fn test_stress_many_tasks() {
    let mut list = TodoList::new();

    // Add 100 tasks
    let ids: Vec<usize> = (0..100)
        .map(|i| list.add_task(format!("Task {}", i)))
        .collect();

    assert_eq!(list.total_count(), 100);

    // Complete even-numbered tasks
    for &id in ids.iter().filter(|&&id| id % 2 == 0) {
        list.complete_task(id).unwrap();
    }

    assert_eq!(list.completed_count(), 50);
    assert_eq!(list.pending_count(), 50);

    // Remove the first 25 tasks
    for &id in &ids[..25] {
        list.remove_task(id).unwrap();
    }

    assert_eq!(list.total_count(), 75);
}
