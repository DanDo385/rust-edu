// Project 19: Chat Server
//
// A multi-client TCP chat server that demonstrates real-world networking,
// concurrency, and message broadcasting. Clients can connect via telnet or
// netcat, send messages, and receive broadcasts from other clients.
//
// Architecture:
// - Main thread: Accepts new connections
// - Client threads: One per client, reads messages
// - Shared state: Arc<Mutex<Vec<Client>>> for broadcasting

use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

// ============================================================================
// CLIENT STRUCTURE
// ============================================================================
// Represents a connected client with their ID and stream for writing.

struct Client {
    id: usize,
    stream: TcpStream,
}

// ============================================================================
// SHARED STATE
// ============================================================================
// Arc<Mutex<Vec<Client>>> allows multiple threads to safely share
// the list of connected clients.
//
// - Arc: Multiple threads can own the client list
// - Mutex: Only one thread can modify the list at a time
// - Vec<Client>: The actual list of connected clients

type ClientList = Arc<Mutex<Vec<Client>>>;

fn main() {
    println!("=== Rust TCP Chat Server ===\n");

    // Bind to localhost:8080
    let listener = TcpListener::bind("127.0.0.1:8080")
        .expect("Failed to bind to address");

    println!("Chat server listening on 127.0.0.1:8080");
    println!("Connect with: telnet localhost 8080");
    println!("Or: nc localhost 8080\n");

    // Shared client list (Arc allows sharing across threads)
    let clients: ClientList = Arc::new(Mutex::new(Vec::new()));

    // Counter for assigning unique IDs to clients
    let mut client_id = 0;

    // Accept connections in a loop
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                client_id += 1;
                println!("Client {} connected from {}", client_id, stream.peer_addr().unwrap());

                // Clone the Arc to share with the new thread
                let clients_clone = Arc::clone(&clients);

                // Spawn a thread to handle this client
                thread::spawn(move || {
                    handle_client(client_id, stream, clients_clone);
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}

// ============================================================================
// CLIENT HANDLER
// ============================================================================
// This function runs in its own thread for each client. It:
// 1. Adds the client to the shared list
// 2. Sends a welcome message
// 3. Reads messages from the client
// 4. Broadcasts messages to all other clients
// 5. Removes the client when they disconnect

fn handle_client(id: usize, stream: TcpStream, clients: ClientList) {
    // Clone the stream for reading (we'll keep original for writing)
    let reader_stream = match stream.try_clone() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to clone stream for client {}: {}", id, e);
            return;
        }
    };

    // Add this client to the shared list
    {
        let mut clients_lock = clients.lock().unwrap();
        clients_lock.push(Client {
            id,
            stream: stream.try_clone().unwrap(),
        });
    }  // Lock released here

    // Send welcome message to this client
    send_to_client(&stream, "Welcome to the chat server!\n");
    send_to_client(&stream, &format!("You are client #{}\n", id));
    send_to_client(&stream, "Type your message and press Enter.\n\n");

    // Announce to all other clients
    broadcast_message(
        &clients,
        &format!(">>> Client {} has joined the chat\n", id),
        Some(id),
    );

    // Read messages from this client
    let mut reader = BufReader::new(reader_stream);
    let mut line = String::new();

    loop {
        line.clear();

        match reader.read_line(&mut line) {
            Ok(0) => {
                // Client disconnected (EOF)
                println!("Client {} disconnected", id);
                break;
            }
            Ok(_) => {
                // Received a message
                let message = line.trim();

                if message.is_empty() {
                    continue;
                }

                println!("Client {}: {}", id, message);

                // Broadcast to all other clients
                let broadcast_msg = format!("Client {}: {}\n", id, message);
                broadcast_message(&clients, &broadcast_msg, Some(id));
            }
            Err(e) => {
                // Error reading from client (probably disconnected)
                eprintln!("Error reading from client {}: {}", id, e);
                break;
            }
        }
    }

    // Remove this client from the list
    {
        let mut clients_lock = clients.lock().unwrap();
        clients_lock.retain(|c| c.id != id);
        println!("Client {} removed from list ({} clients remaining)",
                 id, clients_lock.len());
    }

    // Announce departure to remaining clients
    broadcast_message(
        &clients,
        &format!("<<< Client {} has left the chat\n", id),
        Some(id),
    );
}

// ============================================================================
// BROADCASTING MESSAGES
// ============================================================================
// Send a message to all connected clients (optionally excluding sender).
//
// IMPORTANT: This function must be careful with locking!
// We clone all streams while holding the lock, then release the lock
// before doing I/O. This minimizes lock contention.

fn broadcast_message(clients: &ClientList, message: &str, exclude_id: Option<usize>) {
    // Clone all client streams while holding the lock
    let client_streams: Vec<(usize, TcpStream)> = {
        let clients_lock = clients.lock().unwrap();

        clients_lock
            .iter()
            .filter(|c| Some(c.id) != exclude_id)  // Exclude sender if specified
            .filter_map(|c| {
                // Try to clone the stream
                match c.stream.try_clone() {
                    Ok(stream) => Some((c.id, stream)),
                    Err(e) => {
                        eprintln!("Failed to clone stream for client {}: {}", c.id, e);
                        None
                    }
                }
            })
            .collect()
    };  // Lock released here!

    // Now send to each client (without holding the lock)
    for (id, mut stream) in client_streams {
        if let Err(e) = stream.write_all(message.as_bytes()) {
            eprintln!("Failed to send to client {}: {}", id, e);
            // Note: In a production server, we'd remove this client from the list
        }

        // Flush to ensure message is sent immediately
        if let Err(e) = stream.flush() {
            eprintln!("Failed to flush stream for client {}: {}", id, e);
        }
    }
}

// ============================================================================
// SENDING TO A SINGLE CLIENT
// ============================================================================
// Helper function to send a message to a specific client.

fn send_to_client(stream: &TcpStream, message: &str) {
    let mut stream_clone = match stream.try_clone() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to clone stream: {}", e);
            return;
        }
    };

    if let Err(e) = stream_clone.write_all(message.as_bytes()) {
        eprintln!("Failed to send message: {}", e);
        return;
    }

    if let Err(e) = stream_clone.flush() {
        eprintln!("Failed to flush stream: {}", e);
    }
}

// ============================================================================
// HOW THIS WORKS
// ============================================================================
//
// 1. MAIN THREAD:
//    - Listens for incoming connections
//    - Spawns a thread for each new client
//    - Shares the client list via Arc<Mutex<>>
//
// 2. CLIENT THREADS:
//    - Add themselves to the client list
//    - Read messages in a loop
//    - Broadcast each message to all other clients
//    - Remove themselves on disconnect
//
// 3. MESSAGE FLOW:
//    Client 1 sends "Hello"
//    → handle_client reads from stream
//    → broadcast_message called
//    → Lock client list
//    → Clone all streams (except sender)
//    → Release lock
//    → Write to each cloned stream
//    → Clients 2, 3, 4... receive "Client 1: Hello"
//
// 4. THREAD SAFETY:
//    - Arc: Multiple threads own the client list
//    - Mutex: Only one thread accesses list at a time
//    - We minimize lock time by cloning streams first
//    - I/O happens outside the lock (important for performance!)

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. TcpListener::bind creates a server socket
// 2. listener.incoming() yields a stream of connections
// 3. One thread per client is simple but limited scalability
// 4. Arc<Mutex<Vec<Client>>> enables shared mutable state
// 5. Clone TcpStream to use in multiple places
// 6. Use BufReader for efficient line-oriented reading
// 7. Lock only for minimal time (clone data, then release lock)
// 8. Handle errors gracefully (disconnections are normal)
// 9. Flush writes to ensure messages are sent
// 10. This pattern works for 100s of clients, use async for 1000s+

// ============================================================================
// COMMON MISTAKES
// ============================================================================
// ❌ Holding lock while doing I/O (blocks all broadcasts!)
// ❌ Not handling disconnections (resource leaks)
// ❌ Not using BufReader (inefficient reading)
// ❌ Not flushing writes (messages stuck in buffer)
// ❌ Not validating client input (security!)
// ❌ Panicking on errors (one bad client crashes server)
// ❌ Not cloning TcpStream when sharing (ownership issues)
// ❌ Using thread-per-client for thousands of clients
// ❌ Not removing disconnected clients from list
// ❌ Deadlocks from nested lock acquisition

// ============================================================================
// TESTING THE SERVER
// ============================================================================
//
// Terminal 1 (Start server):
// $ cargo run
//
// Terminal 2 (Client 1):
// $ telnet localhost 8080
// Welcome to the chat server!
// You are client #1
// Hello everyone!
//
// Terminal 3 (Client 2):
// $ nc localhost 8080
// Welcome to the chat server!
// You are client #2
// >>> Client 1 has joined the chat
// Client 1: Hello everyone!
// Hi there!
//
// Terminal 2 sees:
// >>> Client 2 has joined the chat
// Client 2: Hi there!

// ============================================================================
// IMPROVEMENTS FOR PRODUCTION
// ============================================================================
// 1. Use async I/O (tokio) for better scalability
// 2. Add authentication and user accounts
// 3. Implement chat rooms / channels
// 4. Store message history
// 5. Add rate limiting to prevent spam
// 6. Use a proper protocol (not just newline-delimited text)
// 7. Handle backpressure (slow clients)
// 8. Add logging and monitoring
// 9. Graceful shutdown
// 10. Use a thread pool instead of thread-per-client
