//! # Chat Server Components Demo

use chat_server::solution::{self, ClientRegistry, Message, MessageQueue};

fn main() {
    println!("=== Chat Server Components Demo ===\n");

    let mut registry = ClientRegistry::new();
    let alice = registry.register("alice".to_string());
    let bob = registry.register("bob".to_string());

    let mut queue = MessageQueue::new(5);
    let msg = Message::new(alice.id, alice.username.clone(), "hello bob".to_string());
    queue.enqueue(msg.clone());

    println!("alice display: {}", alice.display_name());
    println!("bob display: {}", bob.display_name());
    println!("broadcast: {}", msg.format_for_broadcast());
    println!("queued messages: {}", queue.size());
    println!("active clients: {}", registry.active_count());
    println!("is '/users' command? {}", solution::is_command("/users"));
}
