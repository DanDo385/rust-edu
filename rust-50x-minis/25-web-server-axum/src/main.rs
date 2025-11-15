// Project 25: Web Server with Axum
//
// Implements a RESTful API server using Axum framework.
// Demonstrates async web development, routing, JSON handling,
// and shared state management in Rust.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use tower_http::trace::TraceLayer;
use tracing::{info, warn};

// ============================================================================
// MAIN ENTRY POINT
// ============================================================================

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    info!("=== Axum Web Server ===");
    info!("");

    // Create shared application state
    let app_state = Arc::new(RwLock::new(TodoStore::new()));

    // Seed with some initial data
    {
        let mut store = app_state.write().unwrap();
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
    }

    // Build our application with routes
    let app = Router::new()
        // Root endpoint
        .route("/", get(root_handler))
        // Todo endpoints
        .route("/todos", get(list_todos).post(create_todo))
        .route(
            "/todos/:id",
            get(get_todo).put(update_todo).delete(delete_todo),
        )
        // Health check
        .route("/health", get(health_check))
        // Add shared state
        .with_state(app_state)
        // Add tracing middleware
        .layer(TraceLayer::new_for_http());

    // Configure server address
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    info!("Server configuration:");
    info!("  Address: {}", addr);
    info!("  Routes: 6 endpoints");
    info!("  State: Shared Arc<RwLock<TodoStore>>");
    info!("");
    info!("Starting server...");
    info!("Server running on http://{}", addr);
    info!("");
    info!("Try these commands:");
    info!("  curl http://localhost:3000/");
    info!("  curl http://localhost:3000/todos");
    info!("  curl -X POST http://localhost:3000/todos -H 'Content-Type: application/json' -d '{{\"title\":\"Test\",\"completed\":false}}'");
    info!("");
    info!("Press Ctrl+C to stop.");
    info!("");

    // Start the server
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// ============================================================================
// DATA MODELS
// ============================================================================

/// Todo item
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Todo {
    id: u64,
    title: String,
    completed: bool,
}

/// Request body for creating a new todo
#[derive(Debug, Deserialize)]
struct CreateTodo {
    title: String,
    completed: bool,
}

/// Request body for updating a todo
#[derive(Debug, Deserialize)]
struct UpdateTodo {
    title: Option<String>,
    completed: Option<bool>,
}

// ============================================================================
// APPLICATION STATE
// ============================================================================

/// In-memory todo store
/// In a real application, this would be a database
#[derive(Debug)]
struct TodoStore {
    todos: HashMap<u64, Todo>,
    next_id: u64,
}

impl TodoStore {
    fn new() -> Self {
        TodoStore {
            todos: HashMap::new(),
            next_id: 1,
        }
    }

    fn add_todo(&mut self, create_todo: CreateTodo) -> Todo {
        let todo = Todo {
            id: self.next_id,
            title: create_todo.title,
            completed: create_todo.completed,
        };

        self.todos.insert(self.next_id, todo.clone());
        self.next_id += 1;

        todo
    }

    fn get_todo(&self, id: u64) -> Option<&Todo> {
        self.todos.get(&id)
    }

    fn get_all_todos(&self) -> Vec<Todo> {
        self.todos.values().cloned().collect()
    }

    fn update_todo(&mut self, id: u64, update: UpdateTodo) -> Option<Todo> {
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

    fn delete_todo(&mut self, id: u64) -> Option<Todo> {
        self.todos.remove(&id)
    }

    fn count(&self) -> usize {
        self.todos.len()
    }
}

// ============================================================================
// ROUTE HANDLERS
// ============================================================================

/// Root endpoint - welcome message
async fn root_handler() -> impl IntoResponse {
    info!("GET / - Root endpoint");

    let response = serde_json::json!({
        "message": "Welcome to the Rust Todo API!",
        "version": "1.0.0",
        "endpoints": {
            "GET /": "This message",
            "GET /health": "Health check",
            "GET /todos": "List all todos",
            "POST /todos": "Create a new todo",
            "GET /todos/:id": "Get a specific todo",
            "PUT /todos/:id": "Update a todo",
            "DELETE /todos/:id": "Delete a todo"
        }
    });

    Json(response)
}

/// Health check endpoint
async fn health_check(State(state): State<Arc<RwLock<TodoStore>>>) -> impl IntoResponse {
    info!("GET /health - Health check");

    let store = state.read().unwrap();
    let count = store.count();

    let response = serde_json::json!({
        "status": "healthy",
        "todos_count": count
    });

    Json(response)
}

/// List all todos
async fn list_todos(State(state): State<Arc<RwLock<TodoStore>>>) -> impl IntoResponse {
    info!("GET /todos - List all todos");

    let store = state.read().unwrap();
    let todos = store.get_all_todos();

    info!("  Returning {} todos", todos.len());

    Json(todos)
}

/// Get a specific todo by ID
async fn get_todo(
    State(state): State<Arc<RwLock<TodoStore>>>,
    Path(id): Path<u64>,
) -> Result<Json<Todo>, AppError> {
    info!("GET /todos/{} - Get todo", id);

    let store = state.read().unwrap();

    match store.get_todo(id) {
        Some(todo) => {
            info!("  Found todo: {}", todo.title);
            Ok(Json(todo.clone()))
        }
        None => {
            warn!("  Todo {} not found", id);
            Err(AppError::NotFound)
        }
    }
}

/// Create a new todo
async fn create_todo(
    State(state): State<Arc<RwLock<TodoStore>>>,
    Json(payload): Json<CreateTodo>,
) -> Result<(StatusCode, Json<Todo>), AppError> {
    info!("POST /todos - Create todo");
    info!("  Title: {}", payload.title);

    // Validate input
    if payload.title.trim().is_empty() {
        warn!("  Invalid: empty title");
        return Err(AppError::BadRequest("Title cannot be empty".to_string()));
    }

    if payload.title.len() > 200 {
        warn!("  Invalid: title too long");
        return Err(AppError::BadRequest("Title too long (max 200 chars)".to_string()));
    }

    let mut store = state.write().unwrap();
    let todo = store.add_todo(payload);

    info!("  Created todo {} with ID {}", todo.title, todo.id);

    Ok((StatusCode::CREATED, Json(todo)))
}

/// Update an existing todo
async fn update_todo(
    State(state): State<Arc<RwLock<TodoStore>>>,
    Path(id): Path<u64>,
    Json(payload): Json<UpdateTodo>,
) -> Result<Json<Todo>, AppError> {
    info!("PUT /todos/{} - Update todo", id);

    // Validate input
    if let Some(ref title) = payload.title {
        if title.trim().is_empty() {
            warn!("  Invalid: empty title");
            return Err(AppError::BadRequest("Title cannot be empty".to_string()));
        }
        if title.len() > 200 {
            warn!("  Invalid: title too long");
            return Err(AppError::BadRequest("Title too long (max 200 chars)".to_string()));
        }
    }

    let mut store = state.write().unwrap();

    match store.update_todo(id, payload) {
        Some(todo) => {
            info!("  Updated todo: {}", todo.title);
            Ok(Json(todo))
        }
        None => {
            warn!("  Todo {} not found", id);
            Err(AppError::NotFound)
        }
    }
}

/// Delete a todo
async fn delete_todo(
    State(state): State<Arc<RwLock<TodoStore>>>,
    Path(id): Path<u64>,
) -> Result<StatusCode, AppError> {
    info!("DELETE /todos/{} - Delete todo", id);

    let mut store = state.write().unwrap();

    match store.delete_todo(id) {
        Some(todo) => {
            info!("  Deleted todo: {}", todo.title);
            Ok(StatusCode::NO_CONTENT)
        }
        None => {
            warn!("  Todo {} not found", id);
            Err(AppError::NotFound)
        }
    }
}

// ============================================================================
// ERROR HANDLING
// ============================================================================

/// Application-specific errors
#[derive(Debug)]
enum AppError {
    NotFound,
    BadRequest(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "Resource not found".to_string()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
        };

        let body = Json(serde_json::json!({
            "error": message
        }));

        (status, body).into_response()
    }
}

// ============================================================================
// WHAT RUST DOES UNDER THE HOOD
// ============================================================================
// 1. ASYNC RUNTIME (TOKIO)
//    Tokio manages a thread pool (default: num_cores threads).
//    Async tasks are scheduled on available threads.
//    When a task awaits, the thread can run other tasks.
//    This allows handling thousands of concurrent connections!
//
// 2. ZERO-COPY SERIALIZATION
//    serde_json serializes data directly to response buffer.
//    No intermediate allocations or copies.
//    Rust's ownership ensures this is always safe.
//
// 3. ARC + RWLOCK
//    Arc (atomic reference counting) allows shared ownership.
//    RwLock allows multiple readers OR one writer.
//    Readers don't block each other (great for read-heavy workloads).
//    Writer blocks all readers and other writers.
//
// 4. TYPE-SAFE EXTRACTORS
//    Path, Json, State are "extractors" that parse request data.
//    Extraction happens before handler is called.
//    If extraction fails, handler isn't called (returns error automatically).
//    All type-checked at compile time!
//
// 5. TOWER MIDDLEWARE
//    Middleware wraps handlers in layers (like onion).
//    Each layer can inspect/modify request and response.
//    TraceLayer logs requests without any handler code changes.
//    Zero-cost abstraction - compiled to efficient code.

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. Axum is a modern, ergonomic web framework
// 2. Built on Tokio for async performance
// 3. Type-safe routing and extractors
// 4. Shared state with Arc<RwLock<T>>
// 5. Handlers are async functions
// 6. JSON serialization with serde
// 7. Middleware with tower
// 8. Proper error handling with custom error types
// 9. RESTful API design (GET, POST, PUT, DELETE)
// 10. Production-ready performance

// ============================================================================
// AXUM BEST PRACTICES
// ============================================================================
// ✅ DO:
// - Use extractors for type-safe request parsing
// - Return Result from handlers for proper error handling
// - Use Arc<RwLock<T>> for shared state
// - Add tracing/logging middleware
// - Validate input in handlers
// - Use appropriate HTTP status codes
// - Structure routes logically
// - Use async/await for I/O operations
//
// ❌ DON'T:
// - Block the async runtime with sync I/O
// - Use unwrap() in handlers (panics crash the server!)
// - Share mutable state without synchronization
// - Forget error handling
// - Return generic error messages
// - Use thread::sleep (use tokio::time::sleep)
// - Ignore status codes
// - Mix sync and async incorrectly

// ============================================================================
// COMMON BEGINNER MISTAKES
// ============================================================================
// ❌ Forgetting .await on async calls
// ❌ Using unwrap() in handlers (panics!)
// ❌ Blocking runtime with std::thread::sleep
// ❌ Not validating input
// ❌ Forgetting to add State to Router
// ❌ Wrong status code (200 for create instead of 201)
// ❌ Not handling errors properly
// ❌ Holding locks across .await points (deadlock risk!)
// ❌ Not using middleware for cross-cutting concerns
// ❌ Mixing Arc<Mutex<T>> and Arc<RwLock<T>> incorrectly

// ============================================================================
// EXTENDING THIS PROJECT
// ============================================================================
// 1. Add SQLite/PostgreSQL database with sqlx
// 2. Implement JWT authentication
// 3. Add pagination for todo list
// 4. Add filtering and sorting
// 5. Implement WebSocket for real-time updates
// 6. Add CORS middleware for browser clients
// 7. Add rate limiting
// 8. Implement caching with Redis
// 9. Add OpenAPI/Swagger documentation
// 10. Deploy to cloud (AWS, GCP, Azure)

// ============================================================================
// TESTING YOUR API
// ============================================================================
// # Health check
// curl http://localhost:3000/health
//
// # List todos
// curl http://localhost:3000/todos
//
// # Create todo
// curl -X POST http://localhost:3000/todos \
//   -H "Content-Type: application/json" \
//   -d '{"title":"Learn Axum","completed":false}'
//
// # Get specific todo
// curl http://localhost:3000/todos/1
//
// # Update todo
// curl -X PUT http://localhost:3000/todos/1 \
//   -H "Content-Type: application/json" \
//   -d '{"completed":true}'
//
// # Delete todo
// curl -X DELETE http://localhost:3000/todos/1
