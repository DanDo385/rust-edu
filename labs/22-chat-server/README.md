# Project 19: Chat Server

## Overview
Build a multi-client TCP chat server that demonstrates real-world networking, concurrency, and message broadcasting. Clients can connect, send messages, and receive broadcasts from other clients. This project combines TCP sockets, threads, and shared state.

## Concepts Taught
- **TCP networking** with TcpListener and TcpStream
- **TcpListener::bind** for accepting connections
- **TcpStream** for client connections
- **Handling multiple clients** with threads
- **Message broadcasting** to all connected clients
- **Arc<Mutex<Vec<Client>>>** for shared client list
- **Graceful connection handling** and error recovery
- **Text protocol** design and parsing
- **Real-world concurrency** patterns

## Why Rust Behaves This Way

### TCP Networking
Rust's standard library provides low-level TCP primitives:
- **TcpListener**: Listens for incoming connections
- **TcpStream**: Represents a connection to a client
- **Read/Write traits**: For sending/receiving data

Unlike high-level frameworks (Node.js Express, Python Flask), Rust gives you:
- Fine-grained control over connections
- No hidden magic or middleware
- Direct access to sockets
- Ability to build your own protocols

### Thread-Per-Client Model
This server uses one thread per client:
- Simple to implement
- Good for moderate number of clients (< 1000)
- Each thread blocks on I/O independently
- No callback hell or async complexity

**Alternatives:**
- **Async I/O** (tokio): Better for high concurrency (Project 18)
- **Thread pool**: Reuse threads for many clients (Project 26)
- **Event loop** (mio): Manual non-blocking I/O

### Broadcasting Pattern
When a client sends a message, it must be sent to all other clients:
1. Lock the shared client list
2. Iterate through all clients
3. Send message to each
4. Handle disconnections gracefully

This requires shared mutable state: `Arc<Mutex<Vec<Client>>>`

## Beginner Pitfalls & Borrow Checker Notes

### Pitfall 1: Holding Lock While Doing I/O
```rust
let clients = clients_lock.lock().unwrap();
for client in clients.iter() {
    client.stream.write_all(msg.as_bytes())?;  // ❌ I/O while holding lock!
}
```
**Fix**: Clone what you need, then release lock:
```rust
let streams = {
    let clients = clients_lock.lock().unwrap();
    clients.iter().map(|c| c.stream.try_clone()).collect::<Vec<_>>()
};
// Lock released here
for stream in streams {
    stream?.write_all(msg.as_bytes())?;
}
```

### Pitfall 2: Not Handling Disconnections
```rust
stream.write_all(msg.as_bytes())?;  // ❌ What if client disconnected?
```
**Fix**: Handle errors and remove disconnected clients:
```rust
if stream.write_all(msg.as_bytes()).is_err() {
    // Client disconnected, remove from list
}
```

### Pitfall 3: Deadlock from Nested Locks
```rust
let clients = clients_lock.lock().unwrap();
broadcast_message(&clients_lock, msg)?;  // ❌ Tries to lock again!
```
**Fix**: Release lock before calling functions that lock:
```rust
drop(clients);  // Explicitly release
broadcast_message(&clients_lock, msg)?;
```

### Pitfall 4: Not Using BufReader/BufWriter
```rust
stream.read(&mut buf)?;  // ❌ Inefficient, no line buffering
```
**Fix**: Use BufReader for line-oriented protocols:
```rust
let mut reader = BufReader::new(stream);
let mut line = String::new();
reader.read_line(&mut line)?;  // ✅ Efficient buffered reading
```

## Code Walkthrough

See `src/main.rs` for a complete implementation including:
1. TCP server setup with TcpListener
2. Accepting connections in a loop
3. Spawning a thread for each client
4. Reading messages from clients
5. Broadcasting to all connected clients
6. Handling disconnections gracefully
7. Client list management with Arc<Mutex<>>
8. Error handling and recovery

## Performance Considerations

**Thread-Per-Client Scalability:**
- Each thread: ~2MB stack + OS overhead
- Max clients: ~1000-10000 (depends on system)
- Context switching overhead with many threads
- Simple to implement, good for moderate load

**Better Alternatives:**
- **Async I/O (tokio)**: 10,000+ clients easily
- **Thread pool**: Reuse threads, cap overhead
- **Event loop (mio)**: Manual but very efficient

**Optimization Tips:**
1. Use BufReader/BufWriter (reduces syscalls)
2. Clone TcpStream sparingly (each clone is a file descriptor)
3. Minimize time holding locks (broadcast bottleneck)
4. Consider batching messages
5. Use thread pool for production (Project 26)

## Comparison: Rust vs Node.js vs Go

| Feature | Rust | Node.js | Go |
|---------|------|---------|-----|
| TCP API | TcpListener/TcpStream | net.createServer | net.Listen |
| Concurrency | Threads or async | Async (event loop) | Goroutines |
| Max clients | ~1000 (threads), 10000+ (async) | 10000+ (async) | 100000+ (goroutines) |
| Memory/client | ~2MB (thread), ~KB (async) | ~KB (async) | ~2KB (goroutine) |
| Type safety | Compile-time | Runtime | Compile-time |
| Error handling | Result<T, E> | try/catch | error return values |

## Additional Challenges

1. **Nicknames**: Allow clients to set nicknames, display in messages.

2. **Private Messages**: Implement @username to send direct messages.

3. **Chat Rooms**: Multiple rooms, clients can join/leave.

4. **Message History**: Store last N messages, send to new clients.

5. **Async Version**: Rewrite using tokio for better scalability.

6. **Authentication**: Add password authentication for clients.

7. **Binary Protocol**: Replace text protocol with efficient binary format.

8. **Reconnection**: Handle client reconnections with session IDs.

## Key Takeaways

1. **TcpListener** accepts incoming connections
2. **TcpStream** represents a client connection
3. One thread per client is simple but doesn't scale to thousands
4. **Arc<Mutex<Vec<Client>>>** enables shared client state
5. Lock for minimal time when broadcasting
6. Handle disconnections gracefully (remove from list)
7. Use **BufReader/BufWriter** for efficiency
8. Clone TcpStream to share across threads
9. Text protocols are easy to debug but inefficient
10. Real-world servers need async I/O for high concurrency

## Common Mistakes

❌ Holding lock while doing I/O (blocks all other operations)
❌ Not handling client disconnections (resource leaks)
❌ Not using buffered I/O (too many syscalls)
❌ Nested lock acquisition (deadlocks)
❌ Not validating client input (security risk)
❌ Keeping disconnected clients in list (memory leak)
❌ Using thread-per-client for high concurrency (use async)
❌ Not handling partial reads/writes
❌ Forgetting to flush writers (messages buffered)
❌ No error recovery (one error crashes server)

## Future Directions

- **Next**: Testing and benchmarking (Project 20)
- **Advanced**: Async web server with axum (Project 25)
- **Production**: Thread pool implementation (Project 26)

## Running This Project

```bash
cd 19-chat-server
cargo run
```

In separate terminals, connect with telnet:
```bash
telnet localhost 8080
```

Or use netcat:
```bash
nc localhost 8080
```

## Expected Output

Server console:
```
Chat server listening on 127.0.0.1:8080
Client 1 connected
Client 2 connected
Client 1: Hello!
Broadcasting to 2 clients
Client 2: Hi there!
Broadcasting to 2 clients
```

Client 1:
```
Welcome to the chat!
You: Hello!
Client 2: Hi there!
```

Client 2:
```
Welcome to the chat!
Client 1: Hello!
You: Hi there!
```
