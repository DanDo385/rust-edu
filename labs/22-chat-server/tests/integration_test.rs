//! Integration tests for Lab 22: Chat Server
//!
//! Tests core chat server components without requiring actual TCP connections.
//! These verify the message protocol, client management, and broadcast logic.

use chat_server::solution::{Client, Message, MessageQueue, ClientRegistry, is_command, parse_command};

// ============================================================================
// CLIENT TESTS
// ============================================================================

#[test]
fn test_client_creation() {
    let client = Client::new(1, "alice".to_string());
    assert_eq!(client.id, 1);
    assert_eq!(client.username, "alice");
    assert!(client.is_active());
}

#[test]
fn test_client_disconnect() {
    let mut client = Client::new(2, "bob".to_string());
    assert!(client.is_active());

    client.disconnect();
    assert!(!client.is_active());
}

#[test]
fn test_client_display_name() {
    let client = Client::new(42, "charlie".to_string());
    assert_eq!(client.display_name(), "[42] charlie");
}

#[test]
fn test_client_clone() {
    let client1 = Client::new(5, "dave".to_string());
    let client2 = client1.clone();

    assert_eq!(client1.id, client2.id);
    assert_eq!(client1.username, client2.username);
}

#[test]
fn test_client_multiple_ids() {
    let c1 = Client::new(1, "alice".to_string());
    let c2 = Client::new(2, "bob".to_string());

    assert_ne!(c1.id, c2.id);
    assert_ne!(c1.username, c2.username);
}

// ============================================================================
// MESSAGE TESTS
// ============================================================================

#[test]
fn test_message_creation() {
    let msg = Message::new(1, "alice".to_string(), "hello".to_string());
    assert_eq!(msg.sender_id, 1);
    assert_eq!(msg.sender_name, "alice");
    assert_eq!(msg.content, "hello");
}

#[test]
fn test_message_format_for_broadcast() {
    let msg = Message::new(1, "alice".to_string(), "hello world".to_string());
    let formatted = msg.format_for_broadcast();
    assert_eq!(formatted, "alice: hello world");
}

#[test]
fn test_message_parse_valid() {
    let msg = Message::parse(1, "alice".to_string(), "hello");
    assert!(msg.is_some());
    assert_eq!(msg.unwrap().content, "hello");
}

#[test]
fn test_message_parse_with_spaces() {
    let msg = Message::parse(1, "alice".to_string(), "  hello world  ");
    assert!(msg.is_some());
    assert_eq!(msg.unwrap().content, "hello world");
}

#[test]
fn test_message_parse_empty() {
    assert!(Message::parse(1, "alice".to_string(), "").is_none());
}

#[test]
fn test_message_parse_whitespace_only() {
    assert!(Message::parse(1, "alice".to_string(), "   ").is_none());
}

#[test]
fn test_message_equality() {
    let msg1 = Message::new(1, "alice".to_string(), "hello".to_string());
    let msg2 = Message::new(1, "alice".to_string(), "hello".to_string());
    let msg3 = Message::new(2, "alice".to_string(), "hello".to_string());

    assert_eq!(msg1, msg2);
    assert_ne!(msg1, msg3);
}

#[test]
fn test_message_different_senders() {
    let msg1 = Message::new(1, "alice".to_string(), "test".to_string());
    let msg2 = Message::new(2, "bob".to_string(), "test".to_string());

    assert_eq!(msg1.content, msg2.content);
    assert_ne!(msg1.sender_id, msg2.sender_id);
}

// ============================================================================
// MESSAGE QUEUE TESTS
// ============================================================================

#[test]
fn test_queue_creation() {
    let queue = MessageQueue::new(10);
    assert!(!queue.has_messages());
    assert_eq!(queue.size(), 0);
}

#[test]
fn test_queue_enqueue_single() {
    let mut queue = MessageQueue::new(10);
    let msg = Message::new(1, "alice".to_string(), "hello".to_string());

    queue.enqueue(msg);
    assert!(queue.has_messages());
    assert_eq!(queue.size(), 1);
}

#[test]
fn test_queue_dequeue() {
    let mut queue = MessageQueue::new(10);
    let msg = Message::new(1, "alice".to_string(), "hello".to_string());

    queue.enqueue(msg.clone());
    let dequeued = queue.dequeue();

    assert_eq!(dequeued, Some(msg));
    assert!(!queue.has_messages());
}

#[test]
fn test_queue_fifo_order() {
    let mut queue = MessageQueue::new(10);

    let msg1 = Message::new(1, "alice".to_string(), "first".to_string());
    let msg2 = Message::new(1, "alice".to_string(), "second".to_string());
    let msg3 = Message::new(1, "alice".to_string(), "third".to_string());

    queue.enqueue(msg1.clone());
    queue.enqueue(msg2.clone());
    queue.enqueue(msg3.clone());

    assert_eq!(queue.dequeue(), Some(msg1));
    assert_eq!(queue.dequeue(), Some(msg2));
    assert_eq!(queue.dequeue(), Some(msg3));
    assert_eq!(queue.dequeue(), None);
}

#[test]
fn test_queue_max_size() {
    let mut queue = MessageQueue::new(2);

    let msg1 = Message::new(1, "alice".to_string(), "first".to_string());
    let msg2 = Message::new(1, "alice".to_string(), "second".to_string());
    let msg3 = Message::new(1, "alice".to_string(), "third".to_string());

    queue.enqueue(msg1);
    queue.enqueue(msg2);
    assert_eq!(queue.size(), 2);

    queue.enqueue(msg3);  // Should drop oldest
    assert_eq!(queue.size(), 2);
}

#[test]
fn test_queue_large_capacity() {
    let mut queue = MessageQueue::new(1000);

    for i in 0..100 {
        let msg = Message::new(1, "alice".to_string(), format!("msg{}", i));
        queue.enqueue(msg);
    }

    assert_eq!(queue.size(), 100);
}

// ============================================================================
// CLIENT REGISTRY TESTS
// ============================================================================

#[test]
fn test_registry_creation() {
    let registry = ClientRegistry::new();
    assert_eq!(registry.client_count(), 0);
    assert_eq!(registry.active_count(), 0);
}

#[test]
fn test_registry_register_single() {
    let mut registry = ClientRegistry::new();
    let client = registry.register("alice".to_string());

    assert_eq!(client.id, 1);
    assert_eq!(registry.client_count(), 1);
}

#[test]
fn test_registry_register_multiple() {
    let mut registry = ClientRegistry::new();

    let c1 = registry.register("alice".to_string());
    let c2 = registry.register("bob".to_string());
    let c3 = registry.register("charlie".to_string());

    assert_eq!(c1.id, 1);
    assert_eq!(c2.id, 2);
    assert_eq!(c3.id, 3);
    assert_eq!(registry.client_count(), 3);
}

#[test]
fn test_registry_find_client() {
    let mut registry = ClientRegistry::new();
    let client = registry.register("alice".to_string());

    let found = registry.find_client(1);
    assert!(found.is_some());
    assert_eq!(found.unwrap().username, "alice");
}

#[test]
fn test_registry_find_nonexistent() {
    let registry = ClientRegistry::new();
    assert!(registry.find_client(99).is_none());
}

#[test]
fn test_registry_active_clients() {
    let mut registry = ClientRegistry::new();

    registry.register("alice".to_string());
    registry.register("bob".to_string());

    let active = registry.active_clients();
    assert_eq!(active.len(), 2);
}

#[test]
fn test_registry_disconnect() {
    let mut registry = ClientRegistry::new();
    registry.register("alice".to_string());

    assert_eq!(registry.active_count(), 1);

    registry.disconnect(1);
    assert_eq!(registry.active_count(), 0);
    assert_eq!(registry.client_count(), 1);  // Still in registry
}

#[test]
fn test_registry_disconnect_multiple() {
    let mut registry = ClientRegistry::new();

    registry.register("alice".to_string());
    registry.register("bob".to_string());
    registry.register("charlie".to_string());

    registry.disconnect(1);
    registry.disconnect(3);

    assert_eq!(registry.active_count(), 1);
    assert_eq!(registry.client_count(), 3);
}

#[test]
fn test_registry_default() {
    let registry = ClientRegistry::default();
    assert_eq!(registry.client_count(), 0);
}

// ============================================================================
// COMMAND PARSING TESTS
// ============================================================================

#[test]
fn test_is_command_quit() {
    assert!(is_command("/quit"));
}

#[test]
fn test_is_command_users() {
    assert!(is_command("/users"));
}

#[test]
fn test_is_command_help() {
    assert!(is_command("/help"));
}

#[test]
fn test_is_not_command_message() {
    assert!(!is_command("hello world"));
}

#[test]
fn test_is_command_with_spaces() {
    assert!(is_command("  /quit"));
}

#[test]
fn test_parse_command_valid() {
    let cmd = parse_command("/quit");
    assert_eq!(cmd, Some("quit"));
}

#[test]
fn test_parse_command_with_args() {
    let cmd = parse_command("/kick alice");
    assert_eq!(cmd, Some("kick alice"));
}

#[test]
fn test_parse_command_not_command() {
    assert!(parse_command("hello").is_none());
}

// ============================================================================
// INTEGRATION SCENARIOS
// ============================================================================

#[test]
fn test_server_workflow() {
    // Simulate: client connects, sends message, receives broadcast
    let mut registry = ClientRegistry::new();

    // Client 1 joins
    let client1 = registry.register("alice".to_string());
    assert_eq!(registry.active_count(), 1);

    // Client 2 joins
    let _client2 = registry.register("bob".to_string());
    assert_eq!(registry.active_count(), 2);

    // Client 1 sends message
    let msg = Message::parse(
        client1.id,
        client1.username.clone(),
        "hello everyone",
    ).unwrap();

    assert_eq!(msg.format_for_broadcast(), "alice: hello everyone");

    // Both clients still active
    assert_eq!(registry.active_count(), 2);
}

#[test]
fn test_broadcast_to_multiple_clients() {
    let mut registry = ClientRegistry::new();
    let mut queue1 = MessageQueue::new(10);
    let mut queue2 = MessageQueue::new(10);

    let client1 = registry.register("alice".to_string());
    let _client2 = registry.register("bob".to_string());

    // Client 1 broadcasts message
    let msg = Message::new(client1.id, client1.username.clone(), "Hello!".to_string());

    // Add to all clients' queues (except sender in real implementation)
    queue2.enqueue(msg.clone());

    assert!(queue1.is_empty() || !queue1.has_messages());  // Sender doesn't receive
    assert!(queue2.has_messages());

    let received = queue2.dequeue();
    assert_eq!(received.unwrap().sender_name, "alice");
}

#[test]
fn test_message_history() {
    let mut queue = MessageQueue::new(100);

    // Simulate 10 messages in history
    for i in 1..=10 {
        let msg = Message::new(
            1,
            "alice".to_string(),
            format!("message {}", i),
        );
        queue.enqueue(msg);
    }

    assert_eq!(queue.size(), 10);

    // Retrieve first message
    let first = queue.dequeue();
    assert_eq!(first.unwrap().content, "message 1");
}

#[test]
fn test_command_vs_message() {
    // User types command
    let input1 = "/quit";
    assert!(is_command(input1));
    let cmd = parse_command(input1);
    assert_eq!(cmd, Some("quit"));

    // User types message
    let input2 = "hello everyone";
    assert!(!is_command(input2));
    let msg = Message::parse(1, "alice".to_string(), input2);
    assert!(msg.is_some());
}

#[test]
fn test_multiple_client_sessions() {
    let mut registry = ClientRegistry::new();

    // Simulate multiple clients joining and leaving
    for i in 0..5 {
        let username = format!("user{}", i);
        let client = registry.register(username);
        assert_eq!(client.id as usize, i + 1);
    }

    assert_eq!(registry.active_count(), 5);

    // Some clients disconnect
    registry.disconnect(1);
    registry.disconnect(3);

    assert_eq!(registry.active_count(), 3);
    assert_eq!(registry.client_count(), 5);
}
