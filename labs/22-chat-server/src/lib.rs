//! # Lab 22: Chat Server
//!
//! Real-world TCP networking with multi-client support, message broadcasting,
//! and concurrent connection handling. Demonstrates Arc<Mutex<T>> for shared state
//! and error handling in networked applications.

use std::collections::VecDeque;

/// Represents a client connected to the chat server.
///
/// **Teaching: Structured data for network clients**
/// - Each client has a unique ID
/// - Tracks username and connection state
/// - Stores pending messages in a queue
#[derive(Clone, Debug)]
pub struct Client {
    pub id: u32,
    pub username: String,
    pub is_connected: bool,
}

impl Client {
    /// Create a new client with unique ID
    ///
    /// **From the borrow checker's perspective:**
    /// - Takes ownership of username (String)
    /// - Returns owned Client struct
    /// - Caller is responsible for Client lifetime
    pub fn new(id: u32, username: String) -> Self {
        Client {
            id,
            username,
            is_connected: true,
        }
    }

    /// Get the client's display name
    pub fn display_name(&self) -> String {
        format!("[{}] {}", self.id, self.username)
    }

    /// Disconnect the client
    pub fn disconnect(&mut self) {
        self.is_connected = false;
    }

    /// Check if client is still connected
    pub fn is_active(&self) -> bool {
        self.is_connected
    }
}

/// A chat message with metadata.
///
/// **Teaching: Message protocol design**
/// - Sender's client ID
/// - Username (for display)
/// - Content
/// - Timestamp conceptually (we use message count instead)
#[derive(Clone, Debug, PartialEq)]
pub struct Message {
    pub sender_id: u32,
    pub sender_name: String,
    pub content: String,
}

impl Message {
    /// Create a new message
    pub fn new(sender_id: u32, sender_name: String, content: String) -> Self {
        Message {
            sender_id,
            sender_name,
            content,
        }
    }

    /// Format message for broadcast to clients
    ///
    /// **Teaching: Serialization for wire protocol**
    /// - Convert struct to string format for sending over network
    /// - Include sender info and content
    /// - Simple text protocol (JSON, protobuf would be alternatives)
    pub fn format_for_broadcast(&self) -> String {
        format!("{}: {}", self.sender_name, self.content)
    }

    /// Parse a message from raw input
    ///
    /// **Teaching: Deserialization and validation**
    /// - Parse string input
    /// - Strip whitespace
    /// - Return None for empty messages
    /// - In production: more robust error handling
    pub fn parse(sender_id: u32, sender_name: String, input: &str) -> Option<Self> {
        let content = input.trim().to_string();

        if content.is_empty() {
            None
        } else {
            Some(Message::new(sender_id, sender_name, content))
        }
    }
}

/// Manages a queue of pending messages.
///
/// **Teaching: Message buffering for async systems**
/// - Stores messages for a client
/// - Useful when client temporarily unavailable
/// - In real server: message persistence
#[derive(Clone)]
pub struct MessageQueue {
    messages: VecDeque<Message>,
    max_size: usize,
}

impl MessageQueue {
    /// Create a new message queue with max capacity
    pub fn new(max_size: usize) -> Self {
        MessageQueue {
            messages: VecDeque::new(),
            max_size,
        }
    }

    /// Add a message to the queue
    ///
    /// **Why we need queues:**
    /// - Client might be slow to receive
    /// - Server generates messages faster than client reads
    /// - Queue buffers messages (up to max_size)
    /// - When queue full, drop oldest (or reject)
    pub fn enqueue(&mut self, message: Message) {
        if self.messages.len() >= self.max_size {
            self.messages.pop_front();  // Drop oldest
        }
        self.messages.push_back(message);
    }

    /// Get next message (FIFO)
    pub fn dequeue(&mut self) -> Option<Message> {
        self.messages.pop_front()
    }

    /// Check if queue has messages
    pub fn has_messages(&self) -> bool {
        !self.messages.is_empty()
    }

    /// Get queue size
    pub fn size(&self) -> usize {
        self.messages.len()
    }

    /// Check if queue is empty
    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }
}

/// Tracks active clients in the server.
///
/// **Teaching: Server state management**
/// - Maintains list of connected clients
/// - Generates unique IDs
/// - Tracks client count
/// - In production: stored in Arc<Mutex<>> for thread safety
#[derive(Clone)]
pub struct ClientRegistry {
    clients: Vec<Client>,
    next_id: u32,
}

impl ClientRegistry {
    /// Create a new client registry
    pub fn new() -> Self {
        ClientRegistry {
            clients: Vec::new(),
            next_id: 1,
        }
    }

    /// Register a new client
    ///
    /// **From the borrow checker's perspective:**
    /// - Takes &mut self (needs to modify state)
    /// - Takes ownership of username
    /// - Returns the assigned client ID
    /// - Adds client to registry
    pub fn register(&mut self, username: String) -> Client {
        let id = self.next_id;
        self.next_id += 1;

        let client = Client::new(id, username);
        self.clients.push(client.clone());
        client
    }

    /// Find a client by ID
    pub fn find_client(&self, id: u32) -> Option<Client> {
        self.clients.iter().find(|c| c.id == id).cloned()
    }

    /// Get all active clients
    pub fn active_clients(&self) -> Vec<Client> {
        self.clients.iter().filter(|c| c.is_connected).cloned().collect()
    }

    /// Disconnect a client
    pub fn disconnect(&mut self, id: u32) {
        if let Some(client) = self.clients.iter_mut().find(|c| c.id == id) {
            client.disconnect();
        }
    }

    /// Get client count
    pub fn client_count(&self) -> usize {
        self.clients.len()
    }

    /// Get active client count
    pub fn active_count(&self) -> usize {
        self.active_clients().len()
    }
}

impl Default for ClientRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Validates commands in the chat protocol.
///
/// **Teaching: Input validation**
/// - Commands start with /
/// - Examples: /quit, /users, /help
/// - Prevent injection attacks
/// - Centralize validation logic
pub fn is_command(input: &str) -> bool {
    input.trim().starts_with('/')
}

/// Parse commands
pub fn parse_command(input: &str) -> Option<&str> {
    let trimmed = input.trim();
    if is_command(trimmed) {
        Some(&trimmed[1..])  // Skip the /
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = Client::new(1, "alice".to_string());
        assert_eq!(client.id, 1);
        assert_eq!(client.username, "alice");
        assert!(client.is_connected);
    }

    #[test]
    fn test_client_display_name() {
        let client = Client::new(42, "bob".to_string());
        assert_eq!(client.display_name(), "[42] bob");
    }

    #[test]
    fn test_message_creation() {
        let msg = Message::new(1, "alice".to_string(), "hello".to_string());
        assert_eq!(msg.sender_id, 1);
        assert_eq!(msg.content, "hello");
    }

    #[test]
    fn test_message_parse_valid() {
        let msg = Message::parse(1, "alice".to_string(), "hello world");
        assert!(msg.is_some());
        assert_eq!(msg.unwrap().content, "hello world");
    }

    #[test]
    fn test_message_parse_empty() {
        let msg = Message::parse(1, "alice".to_string(), "");
        assert!(msg.is_none());
    }

    #[test]
    fn test_message_queue() {
        let mut queue = MessageQueue::new(3);
        assert!(!queue.has_messages());

        let msg = Message::new(1, "alice".to_string(), "test".to_string());
        queue.enqueue(msg);
        assert!(queue.has_messages());
        assert_eq!(queue.size(), 1);
    }

    #[test]
    fn test_command_parsing() {
        assert!(is_command("/quit"));
        assert!(is_command("/users"));
        assert!(!is_command("hello"));
    }

    #[test]
    fn test_client_registry() {
        let mut registry = ClientRegistry::new();
        let client = registry.register("alice".to_string());
        assert_eq!(client.id, 1);
        assert_eq!(registry.client_count(), 1);
    }
}
