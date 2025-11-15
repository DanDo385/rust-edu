# Project 25: Web Server with Axum

## Overview
Build a RESTful API server using Axum, a modern web framework built on top of Tokio. Learn about routing, handlers, middleware, JSON serialization, and async web development in Rust.

## Concepts Taught
- **Web framework**: Axum for routing and handlers
- **RESTful API**: HTTP methods (GET, POST, PUT, DELETE)
- **Async/await**: Asynchronous request handling
- **JSON serialization**: Using serde for data exchange
- **Route parameters**: Extracting data from URLs
- **Middleware**: Request/response processing pipeline
- **Error handling**: Proper HTTP error responses
- **State management**: Shared application state with Arc
- **CRUD operations**: Create, Read, Update, Delete

## Why Axum?

### Modern Rust Web Framework
- Built on Tokio (battle-tested async runtime)
- Type-safe routing and extractors
- Excellent performance (comparable to C++ frameworks)
- Great ergonomics with minimal boilerplate
- Strong ecosystem integration

### Comparison with Other Frameworks

| Framework | Language | Async | Type Safety | Performance |
|-----------|----------|-------|-------------|-------------|
| Axum | Rust | ✓ | Strong | Excellent |
| Actix-web | Rust | ✓ | Strong | Excellent |
| Rocket | Rust | ✓ | Strong | Very Good |
| Express | JavaScript | ✓ | Weak | Good |
| FastAPI | Python | ✓ | Medium | Good |
| Gin | Go | ✓ | Medium | Very Good |

## Running This Project

```bash
cd 25-web-server-axum
cargo run

# In another terminal, test the API:
curl http://localhost:3000/
curl http://localhost:3000/todos
curl -X POST http://localhost:3000/todos -H "Content-Type: application/json" -d '{"title":"Learn Rust","completed":false}'
curl http://localhost:3000/todos/1
curl -X PUT http://localhost:3000/todos/1 -H "Content-Type: application/json" -d '{"title":"Learn Rust","completed":true}'
curl -X DELETE http://localhost:3000/todos/1
```

## API Endpoints

This project implements a simple Todo API:

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/` | Health check / welcome message |
| GET | `/todos` | List all todos |
| GET | `/todos/:id` | Get a specific todo |
| POST | `/todos` | Create a new todo |
| PUT | `/todos/:id` | Update a todo |
| DELETE | `/todos/:id` | Delete a todo |

## Performance Considerations

**Request throughput:**
- Axum can handle 100,000+ requests/second
- Zero-cost abstractions (no runtime overhead)
- Async I/O prevents thread blocking

**Memory usage:**
- Shared state with Arc (atomic reference counting)
- RwLock for concurrent reads, exclusive writes
- Minimal allocations per request

**Scalability:**
- Tokio runtime manages thread pool automatically
- Can handle thousands of concurrent connections
- Horizontal scaling with load balancer

## Real-World Web Server Features

1. **Authentication/Authorization**: JWT tokens, OAuth, session management
2. **Database integration**: PostgreSQL, MySQL, MongoDB
3. **Caching**: Redis for fast data access
4. **Rate limiting**: Prevent abuse and DoS
5. **CORS**: Cross-Origin Resource Sharing for browsers
6. **Logging**: Request logging with tracing
7. **Metrics**: Prometheus, monitoring
8. **WebSockets**: Real-time bidirectional communication
9. **File uploads**: Multipart form data
10. **SSL/TLS**: HTTPS with certificates

## Beginner Pitfalls

### Pitfall 1: Forgetting async/await
```rust
// ❌ Won't compile - handler must be async
async fn handler() -> String {
    let result = fetch_data();  // Missing .await
    result
}
```
**Fix**: Add `.await` to async calls:
```rust
async fn handler() -> String {
    let result = fetch_data().await;
    result
}
```

### Pitfall 2: Blocking the Runtime
```rust
// ❌ Blocks the async runtime (very bad!)
async fn handler() -> String {
    std::thread::sleep(Duration::from_secs(10));  // Blocks thread!
    "Done".to_string()
}
```
**Fix**: Use async sleep:
```rust
async fn handler() -> String {
    tokio::time::sleep(Duration::from_secs(10)).await;
    "Done".to_string()
}
```

### Pitfall 3: Not Handling Errors Properly
```rust
// ❌ Panics on error (crashes server!)
async fn handler() -> Json<Todo> {
    let todo = fetch_todo().unwrap();  // Don't do this!
    Json(todo)
}
```
**Fix**: Return Result with proper error type:
```rust
async fn handler() -> Result<Json<Todo>, StatusCode> {
    let todo = fetch_todo().map_err(|_| StatusCode::NOT_FOUND)?;
    Ok(Json(todo))
}
```

## Advanced Topics

1. **Database pooling**: Connection management with sqlx or diesel
2. **Middleware tower**: Request/response transformation
3. **WebSocket support**: Real-time communication
4. **Server-Sent Events**: Streaming data to clients
5. **OpenAPI/Swagger**: API documentation generation
6. **GraphQL**: Alternative to REST with async-graphql
7. **gRPC**: High-performance RPC with tonic

## Additional Challenges

1. **Add persistence**: Store todos in SQLite or PostgreSQL
2. **Add authentication**: JWT-based user authentication
3. **Add validation**: Check todo fields before saving
4. **Add pagination**: Limit results with offset/limit
5. **Add filtering**: Query todos by completion status
6. **Add WebSocket**: Real-time todo updates
7. **Add rate limiting**: Prevent API abuse
8. **Add caching**: Redis for frequently accessed todos

## Next Steps

- **Project 26**: Thread pool implementation
- **Project 28**: Key-value store with persistence
- **Project 41**: Web scraper with HTTP client

## Expected Output

```
=== Axum Web Server ===

Server configuration:
- Address: 0.0.0.0:3000
- Routes: 6 endpoints
- State: Shared Arc<RwLock<TodoStore>>

Starting server...
Server running on http://0.0.0.0:3000

Press Ctrl+C to stop.

[Logs will appear here as requests come in]
```

## Dependencies

```toml
[dependencies]
axum = "0.7"           # Web framework
tokio = { version = "1", features = ["full"] }  # Async runtime
serde = { version = "1", features = ["derive"] }  # Serialization
serde_json = "1"       # JSON support
tower = "0.4"          # Middleware
tower-http = { version = "0.5", features = ["trace", "cors"] }  # HTTP middleware
tracing = "0.1"        # Logging
tracing-subscriber = "0.3"  # Log formatting
```

## Testing Your API

Use curl, Postman, or write integration tests:

```bash
# Create a todo
curl -X POST http://localhost:3000/todos \
  -H "Content-Type: application/json" \
  -d '{"title":"Build web server","completed":false}'

# Get all todos
curl http://localhost:3000/todos

# Get specific todo
curl http://localhost:3000/todos/1

# Update todo
curl -X PUT http://localhost:3000/todos/1 \
  -H "Content-Type: application/json" \
  -d '{"title":"Build web server","completed":true}'

# Delete todo
curl -X DELETE http://localhost:3000/todos/1
```

## Security Considerations

1. **Input validation**: Never trust user input
2. **SQL injection**: Use parameterized queries
3. **XSS protection**: Sanitize output
4. **CSRF tokens**: Protect state-changing operations
5. **Rate limiting**: Prevent DoS attacks
6. **HTTPS**: Encrypt data in transit
7. **Authentication**: Verify user identity
8. **Authorization**: Check permissions
