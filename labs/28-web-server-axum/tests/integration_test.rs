// Lab 28: Web Server with Axum - Integration Tests
//
// These tests exercise the TodoStore and data models WITHOUT
// starting a web server or requiring async runtime. All business
// logic is tested synchronously through the store API.

use web_server_axum::*;

// ============================================================================
// STORE CREATION TESTS
// ============================================================================

#[test]
fn test_new_store_is_empty() {
    let store = TodoStore::new();
    assert_eq!(store.count(), 0);
    assert!(store.is_empty());
}

#[test]
fn test_default_store_is_empty() {
    let store = TodoStore::default();
    assert_eq!(store.count(), 0);
    assert!(store.is_empty());
}

#[test]
fn test_new_and_default_are_equivalent() {
    let store_new = TodoStore::new();
    let store_default = TodoStore::default();
    assert_eq!(store_new.count(), store_default.count());
    assert_eq!(store_new.is_empty(), store_default.is_empty());
}

// ============================================================================
// ADD TODO TESTS
// ============================================================================

#[test]
fn test_add_todo_returns_created_todo() {
    let mut store = TodoStore::new();
    let todo = store.add_todo(CreateTodo {
        title: "Learn Rust".to_string(),
        completed: false,
    });
    assert_eq!(todo.title, "Learn Rust");
    assert!(!todo.completed);
}

#[test]
fn test_add_todo_assigns_id_starting_at_1() {
    let mut store = TodoStore::new();
    let todo = store.add_todo(CreateTodo {
        title: "First".to_string(),
        completed: false,
    });
    assert_eq!(todo.id, 1, "First todo should have ID 1");
}

#[test]
fn test_add_todo_auto_increments_id() {
    let mut store = TodoStore::new();
    let todo1 = store.add_todo(CreateTodo {
        title: "First".to_string(),
        completed: false,
    });
    let todo2 = store.add_todo(CreateTodo {
        title: "Second".to_string(),
        completed: false,
    });
    let todo3 = store.add_todo(CreateTodo {
        title: "Third".to_string(),
        completed: true,
    });
    assert_eq!(todo1.id, 1);
    assert_eq!(todo2.id, 2);
    assert_eq!(todo3.id, 3);
}

#[test]
fn test_add_todo_increments_count() {
    let mut store = TodoStore::new();
    assert_eq!(store.count(), 0);

    store.add_todo(CreateTodo {
        title: "A".to_string(),
        completed: false,
    });
    assert_eq!(store.count(), 1);

    store.add_todo(CreateTodo {
        title: "B".to_string(),
        completed: false,
    });
    assert_eq!(store.count(), 2);
}

#[test]
fn test_add_todo_completed_true() {
    let mut store = TodoStore::new();
    let todo = store.add_todo(CreateTodo {
        title: "Already done".to_string(),
        completed: true,
    });
    assert!(todo.completed);
}

#[test]
fn test_store_not_empty_after_add() {
    let mut store = TodoStore::new();
    store.add_todo(CreateTodo {
        title: "Task".to_string(),
        completed: false,
    });
    assert!(!store.is_empty());
}

// ============================================================================
// GET TODO TESTS
// ============================================================================

#[test]
fn test_get_existing_todo() {
    let mut store = TodoStore::new();
    store.add_todo(CreateTodo {
        title: "Test".to_string(),
        completed: false,
    });

    let todo = store.get_todo(1);
    assert!(todo.is_some());
    assert_eq!(todo.unwrap().title, "Test");
    assert_eq!(todo.unwrap().id, 1);
}

#[test]
fn test_get_nonexistent_todo() {
    let store = TodoStore::new();
    let result = store.get_todo(999);
    assert!(result.is_none(), "Getting nonexistent ID should return None");
}

#[test]
fn test_get_todo_after_multiple_adds() {
    let mut store = TodoStore::new();
    store.add_todo(CreateTodo {
        title: "First".to_string(),
        completed: false,
    });
    store.add_todo(CreateTodo {
        title: "Second".to_string(),
        completed: true,
    });
    store.add_todo(CreateTodo {
        title: "Third".to_string(),
        completed: false,
    });

    let todo2 = store.get_todo(2).unwrap();
    assert_eq!(todo2.title, "Second");
    assert!(todo2.completed);
}

#[test]
fn test_get_todo_id_zero_returns_none() {
    let store = TodoStore::new();
    // IDs start at 1, so 0 should never exist.
    assert!(store.get_todo(0).is_none());
}

// ============================================================================
// GET ALL TODOS TESTS
// ============================================================================

#[test]
fn test_get_all_todos_empty_store() {
    let store = TodoStore::new();
    let todos = store.get_all_todos();
    assert!(todos.is_empty());
}

#[test]
fn test_get_all_todos_returns_all() {
    let mut store = TodoStore::new();
    store.add_todo(CreateTodo {
        title: "A".to_string(),
        completed: false,
    });
    store.add_todo(CreateTodo {
        title: "B".to_string(),
        completed: true,
    });
    store.add_todo(CreateTodo {
        title: "C".to_string(),
        completed: false,
    });

    let todos = store.get_all_todos();
    assert_eq!(todos.len(), 3);
}

#[test]
fn test_get_all_todos_sorted_by_id() {
    let mut store = TodoStore::new();
    store.add_todo(CreateTodo {
        title: "C".to_string(),
        completed: false,
    });
    store.add_todo(CreateTodo {
        title: "A".to_string(),
        completed: false,
    });
    store.add_todo(CreateTodo {
        title: "B".to_string(),
        completed: false,
    });

    let sorted = store.get_all_todos_sorted();
    assert_eq!(sorted.len(), 3);
    assert_eq!(sorted[0].id, 1);
    assert_eq!(sorted[0].title, "C");
    assert_eq!(sorted[1].id, 2);
    assert_eq!(sorted[1].title, "A");
    assert_eq!(sorted[2].id, 3);
    assert_eq!(sorted[2].title, "B");
}

#[test]
fn test_get_all_todos_returns_clones() {
    let mut store = TodoStore::new();
    store.add_todo(CreateTodo {
        title: "Original".to_string(),
        completed: false,
    });

    let mut todos = store.get_all_todos();
    // Mutating the returned vec should NOT affect the store.
    todos[0].title = "Modified".to_string();

    let original = store.get_todo(1).unwrap();
    assert_eq!(
        original.title, "Original",
        "Store should be unaffected by mutations to returned clones"
    );
}

// ============================================================================
// UPDATE TODO TESTS
// ============================================================================

#[test]
fn test_update_todo_title() {
    let mut store = TodoStore::new();
    store.add_todo(CreateTodo {
        title: "Old title".to_string(),
        completed: false,
    });

    let result = store.update_todo(
        1,
        UpdateTodo {
            title: Some("New title".to_string()),
            completed: None,
        },
    );

    assert!(result.is_some());
    let updated = result.unwrap();
    assert_eq!(updated.title, "New title");
    assert!(!updated.completed, "Completed should be unchanged");
}

#[test]
fn test_update_todo_completed() {
    let mut store = TodoStore::new();
    store.add_todo(CreateTodo {
        title: "Task".to_string(),
        completed: false,
    });

    let result = store.update_todo(
        1,
        UpdateTodo {
            title: None,
            completed: Some(true),
        },
    );

    assert!(result.is_some());
    let updated = result.unwrap();
    assert_eq!(updated.title, "Task", "Title should be unchanged");
    assert!(updated.completed);
}

#[test]
fn test_update_todo_both_fields() {
    let mut store = TodoStore::new();
    store.add_todo(CreateTodo {
        title: "Old".to_string(),
        completed: false,
    });

    let result = store.update_todo(
        1,
        UpdateTodo {
            title: Some("New".to_string()),
            completed: Some(true),
        },
    );

    let updated = result.unwrap();
    assert_eq!(updated.title, "New");
    assert!(updated.completed);
}

#[test]
fn test_update_todo_no_fields() {
    let mut store = TodoStore::new();
    store.add_todo(CreateTodo {
        title: "Unchanged".to_string(),
        completed: false,
    });

    let result = store.update_todo(
        1,
        UpdateTodo {
            title: None,
            completed: None,
        },
    );

    let updated = result.unwrap();
    assert_eq!(
        updated.title, "Unchanged",
        "No-op update should leave fields unchanged"
    );
    assert!(!updated.completed);
}

#[test]
fn test_update_nonexistent_todo() {
    let mut store = TodoStore::new();
    let result = store.update_todo(
        999,
        UpdateTodo {
            title: Some("Ghost".to_string()),
            completed: None,
        },
    );
    assert!(result.is_none(), "Updating nonexistent todo should return None");
}

#[test]
fn test_update_persists_in_store() {
    let mut store = TodoStore::new();
    store.add_todo(CreateTodo {
        title: "Before".to_string(),
        completed: false,
    });

    store.update_todo(
        1,
        UpdateTodo {
            title: Some("After".to_string()),
            completed: None,
        },
    );

    // Verify the update persists when we fetch again.
    let todo = store.get_todo(1).unwrap();
    assert_eq!(todo.title, "After");
}

// ============================================================================
// DELETE TODO TESTS
// ============================================================================

#[test]
fn test_delete_existing_todo() {
    let mut store = TodoStore::new();
    store.add_todo(CreateTodo {
        title: "To delete".to_string(),
        completed: false,
    });

    let result = store.delete_todo(1);
    assert!(result.is_some());
    assert_eq!(result.unwrap().title, "To delete");
}

#[test]
fn test_delete_reduces_count() {
    let mut store = TodoStore::new();
    store.add_todo(CreateTodo {
        title: "A".to_string(),
        completed: false,
    });
    store.add_todo(CreateTodo {
        title: "B".to_string(),
        completed: false,
    });
    assert_eq!(store.count(), 2);

    store.delete_todo(1);
    assert_eq!(store.count(), 1);
}

#[test]
fn test_delete_nonexistent_todo() {
    let mut store = TodoStore::new();
    let result = store.delete_todo(999);
    assert!(result.is_none(), "Deleting nonexistent todo should return None");
}

#[test]
fn test_delete_same_todo_twice() {
    let mut store = TodoStore::new();
    store.add_todo(CreateTodo {
        title: "Delete me".to_string(),
        completed: false,
    });

    let first = store.delete_todo(1);
    assert!(first.is_some());

    let second = store.delete_todo(1);
    assert!(
        second.is_none(),
        "Deleting same todo twice should return None the second time"
    );
}

#[test]
fn test_delete_does_not_affect_other_todos() {
    let mut store = TodoStore::new();
    store.add_todo(CreateTodo {
        title: "Keep".to_string(),
        completed: false,
    });
    store.add_todo(CreateTodo {
        title: "Delete".to_string(),
        completed: false,
    });
    store.add_todo(CreateTodo {
        title: "Keep too".to_string(),
        completed: false,
    });

    store.delete_todo(2);

    assert!(store.get_todo(1).is_some(), "Todo 1 should still exist");
    assert!(store.get_todo(2).is_none(), "Todo 2 should be deleted");
    assert!(store.get_todo(3).is_some(), "Todo 3 should still exist");
    assert_eq!(store.count(), 2);
}

#[test]
fn test_get_todo_after_delete_returns_none() {
    let mut store = TodoStore::new();
    store.add_todo(CreateTodo {
        title: "Temporary".to_string(),
        completed: false,
    });

    store.delete_todo(1);
    assert!(store.get_todo(1).is_none());
}

// ============================================================================
// COUNT AND STATUS TESTS
// ============================================================================

#[test]
fn test_count_empty() {
    let store = TodoStore::new();
    assert_eq!(store.count(), 0);
}

#[test]
fn test_count_after_adds() {
    let mut store = TodoStore::new();
    for i in 0..10 {
        store.add_todo(CreateTodo {
            title: format!("Todo {}", i),
            completed: false,
        });
    }
    assert_eq!(store.count(), 10);
}

#[test]
fn test_is_empty_true() {
    let store = TodoStore::new();
    assert!(store.is_empty());
}

#[test]
fn test_is_empty_false() {
    let mut store = TodoStore::new();
    store.add_todo(CreateTodo {
        title: "Not empty".to_string(),
        completed: false,
    });
    assert!(!store.is_empty());
}

#[test]
fn test_completed_count() {
    let mut store = TodoStore::new();
    store.add_todo(CreateTodo {
        title: "A".to_string(),
        completed: true,
    });
    store.add_todo(CreateTodo {
        title: "B".to_string(),
        completed: false,
    });
    store.add_todo(CreateTodo {
        title: "C".to_string(),
        completed: true,
    });

    assert_eq!(store.completed_count(), 2);
    assert_eq!(store.pending_count(), 1);
}

#[test]
fn test_pending_count() {
    let mut store = TodoStore::new();
    store.add_todo(CreateTodo {
        title: "A".to_string(),
        completed: false,
    });
    store.add_todo(CreateTodo {
        title: "B".to_string(),
        completed: false,
    });
    store.add_todo(CreateTodo {
        title: "C".to_string(),
        completed: true,
    });

    assert_eq!(store.pending_count(), 2);
}

#[test]
fn test_completed_plus_pending_equals_total() {
    let mut store = TodoStore::new();
    store.add_todo(CreateTodo {
        title: "A".to_string(),
        completed: true,
    });
    store.add_todo(CreateTodo {
        title: "B".to_string(),
        completed: false,
    });
    store.add_todo(CreateTodo {
        title: "C".to_string(),
        completed: true,
    });
    store.add_todo(CreateTodo {
        title: "D".to_string(),
        completed: false,
    });

    assert_eq!(
        store.completed_count() + store.pending_count(),
        store.count(),
        "Completed + pending should equal total count"
    );
}

#[test]
fn test_completed_count_empty_store() {
    let store = TodoStore::new();
    assert_eq!(store.completed_count(), 0);
    assert_eq!(store.pending_count(), 0);
}

#[test]
fn test_completed_count_after_update() {
    let mut store = TodoStore::new();
    store.add_todo(CreateTodo {
        title: "Task".to_string(),
        completed: false,
    });
    assert_eq!(store.completed_count(), 0);
    assert_eq!(store.pending_count(), 1);

    store.update_todo(
        1,
        UpdateTodo {
            title: None,
            completed: Some(true),
        },
    );
    assert_eq!(store.completed_count(), 1);
    assert_eq!(store.pending_count(), 0);
}

// ============================================================================
// VALIDATION TESTS
// ============================================================================

#[test]
fn test_validate_create_todo_valid() {
    let create = CreateTodo {
        title: "Valid title".to_string(),
        completed: false,
    };
    assert!(validate_create_todo(&create).is_ok());
}

#[test]
fn test_validate_create_todo_empty_title() {
    let create = CreateTodo {
        title: "".to_string(),
        completed: false,
    };
    let err = validate_create_todo(&create).unwrap_err();
    assert_eq!(err, AppError::BadRequest("Title cannot be empty".to_string()));
}

#[test]
fn test_validate_create_todo_whitespace_only_title() {
    let create = CreateTodo {
        title: "   ".to_string(),
        completed: false,
    };
    let err = validate_create_todo(&create).unwrap_err();
    assert_eq!(err, AppError::BadRequest("Title cannot be empty".to_string()));
}

#[test]
fn test_validate_create_todo_title_too_long() {
    let create = CreateTodo {
        title: "x".repeat(201),
        completed: false,
    };
    let err = validate_create_todo(&create).unwrap_err();
    assert_eq!(
        err,
        AppError::BadRequest("Title too long (max 200 chars)".to_string())
    );
}

#[test]
fn test_validate_create_todo_title_exactly_200() {
    let create = CreateTodo {
        title: "x".repeat(200),
        completed: false,
    };
    assert!(
        validate_create_todo(&create).is_ok(),
        "Title of exactly 200 chars should be valid"
    );
}

#[test]
fn test_validate_update_todo_valid_title() {
    let update = UpdateTodo {
        title: Some("New title".to_string()),
        completed: None,
    };
    assert!(validate_update_todo(&update).is_ok());
}

#[test]
fn test_validate_update_todo_no_fields() {
    let update = UpdateTodo {
        title: None,
        completed: None,
    };
    assert!(
        validate_update_todo(&update).is_ok(),
        "Update with no fields should be valid"
    );
}

#[test]
fn test_validate_update_todo_empty_title() {
    let update = UpdateTodo {
        title: Some("".to_string()),
        completed: None,
    };
    assert!(validate_update_todo(&update).is_err());
}

#[test]
fn test_validate_update_todo_whitespace_title() {
    let update = UpdateTodo {
        title: Some("  \t  ".to_string()),
        completed: None,
    };
    assert!(validate_update_todo(&update).is_err());
}

#[test]
fn test_validate_update_todo_title_too_long() {
    let update = UpdateTodo {
        title: Some("y".repeat(201)),
        completed: None,
    };
    assert!(validate_update_todo(&update).is_err());
}

#[test]
fn test_validate_update_todo_only_completed() {
    let update = UpdateTodo {
        title: None,
        completed: Some(true),
    };
    assert!(
        validate_update_todo(&update).is_ok(),
        "Only setting completed should be valid"
    );
}

// ============================================================================
// APP ERROR TESTS
// ============================================================================

#[test]
fn test_app_error_not_found_display() {
    let err = AppError::NotFound;
    assert_eq!(format!("{}", err), "Resource not found");
}

#[test]
fn test_app_error_bad_request_display() {
    let err = AppError::BadRequest("Invalid input".to_string());
    assert_eq!(format!("{}", err), "Bad request: Invalid input");
}

#[test]
fn test_app_error_equality() {
    assert_eq!(AppError::NotFound, AppError::NotFound);
    assert_eq!(
        AppError::BadRequest("msg".to_string()),
        AppError::BadRequest("msg".to_string())
    );
    assert_ne!(AppError::NotFound, AppError::BadRequest("msg".to_string()));
}

#[test]
fn test_app_error_debug_format() {
    let err = AppError::NotFound;
    let debug = format!("{:?}", err);
    assert!(debug.contains("NotFound"));
}

// ============================================================================
// TODO STRUCT TESTS
// ============================================================================

#[test]
fn test_todo_equality() {
    let a = Todo {
        id: 1,
        title: "Test".to_string(),
        completed: false,
    };
    let b = Todo {
        id: 1,
        title: "Test".to_string(),
        completed: false,
    };
    assert_eq!(a, b);
}

#[test]
fn test_todo_inequality() {
    let a = Todo {
        id: 1,
        title: "A".to_string(),
        completed: false,
    };
    let b = Todo {
        id: 2,
        title: "A".to_string(),
        completed: false,
    };
    assert_ne!(a, b, "Todos with different IDs should not be equal");
}

#[test]
fn test_todo_clone() {
    let original = Todo {
        id: 1,
        title: "Clone me".to_string(),
        completed: true,
    };
    let cloned = original.clone();
    assert_eq!(original, cloned);
}

#[test]
fn test_todo_debug_format() {
    let todo = Todo {
        id: 1,
        title: "Test".to_string(),
        completed: false,
    };
    let debug = format!("{:?}", todo);
    assert!(debug.contains("Todo"));
    assert!(debug.contains("Test"));
}

// ============================================================================
// SERDE SERIALIZATION TESTS
// ============================================================================

#[test]
fn test_todo_serializes_to_json() {
    let todo = Todo {
        id: 1,
        title: "Test".to_string(),
        completed: false,
    };
    let json = serde_json::to_string(&todo).unwrap();
    assert!(json.contains("\"id\":1"));
    assert!(json.contains("\"title\":\"Test\""));
    assert!(json.contains("\"completed\":false"));
}

#[test]
fn test_todo_deserializes_from_json() {
    let json = r#"{"id":42,"title":"From JSON","completed":true}"#;
    let todo: Todo = serde_json::from_str(json).unwrap();
    assert_eq!(todo.id, 42);
    assert_eq!(todo.title, "From JSON");
    assert!(todo.completed);
}

#[test]
fn test_create_todo_deserializes_from_json() {
    let json = r#"{"title":"New task","completed":false}"#;
    let create: CreateTodo = serde_json::from_str(json).unwrap();
    assert_eq!(create.title, "New task");
    assert!(!create.completed);
}

#[test]
fn test_update_todo_deserializes_partial() {
    let json = r#"{"completed":true}"#;
    let update: UpdateTodo = serde_json::from_str(json).unwrap();
    assert!(update.title.is_none(), "Absent fields should be None");
    assert_eq!(update.completed, Some(true));
}

#[test]
fn test_update_todo_deserializes_full() {
    let json = r#"{"title":"Updated","completed":false}"#;
    let update: UpdateTodo = serde_json::from_str(json).unwrap();
    assert_eq!(update.title, Some("Updated".to_string()));
    assert_eq!(update.completed, Some(false));
}

// ============================================================================
// ID MANAGEMENT TESTS
// ============================================================================

#[test]
fn test_ids_are_unique() {
    let mut store = TodoStore::new();
    let mut ids = Vec::new();
    for i in 0..100 {
        let todo = store.add_todo(CreateTodo {
            title: format!("Todo {}", i),
            completed: false,
        });
        assert!(
            !ids.contains(&todo.id),
            "ID {} was already assigned",
            todo.id
        );
        ids.push(todo.id);
    }
}

#[test]
fn test_ids_dont_reuse_after_delete() {
    let mut store = TodoStore::new();
    let todo1 = store.add_todo(CreateTodo {
        title: "First".to_string(),
        completed: false,
    });
    assert_eq!(todo1.id, 1);

    store.delete_todo(1);

    let todo2 = store.add_todo(CreateTodo {
        title: "Second".to_string(),
        completed: false,
    });
    assert_eq!(
        todo2.id, 2,
        "IDs should not be reused after deletion"
    );
}

// ============================================================================
// LARGE-SCALE STORE TESTS
// ============================================================================

#[test]
fn test_store_with_many_todos() {
    let mut store = TodoStore::new();
    for i in 0..1000 {
        store.add_todo(CreateTodo {
            title: format!("Todo number {}", i),
            completed: i % 2 == 0,
        });
    }

    assert_eq!(store.count(), 1000);
    assert_eq!(store.completed_count(), 500);
    assert_eq!(store.pending_count(), 500);

    // Verify random access works.
    let todo500 = store.get_todo(500).unwrap();
    assert_eq!(todo500.title, "Todo number 499");
}

#[test]
fn test_store_delete_all() {
    let mut store = TodoStore::new();
    for i in 0..10 {
        store.add_todo(CreateTodo {
            title: format!("Todo {}", i),
            completed: false,
        });
    }
    assert_eq!(store.count(), 10);

    for id in 1..=10 {
        store.delete_todo(id);
    }
    assert_eq!(store.count(), 0);
    assert!(store.is_empty());
}

// ============================================================================
// COMBINED WORKFLOW TESTS
// ============================================================================

#[test]
fn test_crud_workflow() {
    let mut store = TodoStore::new();

    // Create
    let created = store.add_todo(CreateTodo {
        title: "Buy groceries".to_string(),
        completed: false,
    });
    assert_eq!(created.id, 1);
    assert_eq!(store.count(), 1);

    // Read
    let fetched = store.get_todo(1).unwrap();
    assert_eq!(fetched.title, "Buy groceries");
    assert!(!fetched.completed);

    // Update
    let updated = store
        .update_todo(
            1,
            UpdateTodo {
                title: None,
                completed: Some(true),
            },
        )
        .unwrap();
    assert_eq!(updated.title, "Buy groceries");
    assert!(updated.completed);

    // Delete
    let deleted = store.delete_todo(1).unwrap();
    assert_eq!(deleted.title, "Buy groceries");
    assert_eq!(store.count(), 0);
}

#[test]
fn test_full_api_simulation() {
    let mut store = TodoStore::new();

    // Simulate adding initial data (like in main.rs seed).
    store.add_todo(CreateTodo {
        title: "Learn Rust".to_string(),
        completed: false,
    });
    store.add_todo(CreateTodo {
        title: "Build web server".to_string(),
        completed: false,
    });
    store.add_todo(CreateTodo {
        title: "Deploy to production".to_string(),
        completed: false,
    });

    assert_eq!(store.count(), 3);
    assert_eq!(store.completed_count(), 0);

    // Mark first as completed.
    store.update_todo(
        1,
        UpdateTodo {
            title: None,
            completed: Some(true),
        },
    );
    assert_eq!(store.completed_count(), 1);

    // Add another.
    let new_todo = store.add_todo(CreateTodo {
        title: "Write tests".to_string(),
        completed: false,
    });
    assert_eq!(new_todo.id, 4);
    assert_eq!(store.count(), 4);

    // Delete one.
    store.delete_todo(3);
    assert_eq!(store.count(), 3);

    // Verify final state.
    let all = store.get_all_todos_sorted();
    assert_eq!(all.len(), 3);
    assert_eq!(all[0].id, 1);
    assert!(all[0].completed);
    assert_eq!(all[1].id, 2);
    assert!(!all[1].completed);
    assert_eq!(all[2].id, 4);
    assert!(!all[2].completed);
}
