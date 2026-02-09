//! # Web Server Model Demo

use web_server_axum::solution::{CreateTodo, TodoStore};

fn main() {
    println!("=== Web Server Model Demo ===\n");

    let mut store = TodoStore::new();
    let t1 = store.add_todo(CreateTodo { title: "Learn axum".to_string(), completed: false });
    let t2 = store.add_todo(CreateTodo { title: "Write tests".to_string(), completed: true });

    println!("created: {:?}", t1);
    println!("created: {:?}", t2);
    println!("count: {} (completed: {}, pending: {})", store.count(), store.completed_count(), store.pending_count());
}
