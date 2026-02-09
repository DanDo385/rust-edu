//! # An Async Message Bus - Interactive Demo
//! 
//! This binary demonstrates the `MessageBus` from our library by simulating
//! a simple chat system with different rooms (topics).
//! Run with: cargo run -p message-bus

use message_bus::solution::{MessageBus, Message};
use std::sync::Arc;
use tokio::time::{sleep, Duration};

// Use the `tokio::main` macro to automatically set up the async runtime.
#[tokio::main]
async fn main() {
    println!("=== Async Message Bus Demo ===\n");

    let bus = Arc::new(MessageBus::new());

    // ============================================================================
    // DEMO: Spawn subscribers for different topics
    // ============================================================================

    println!("Spawning subscribers for topics 'general' and 'random'...");

    // Subscriber 1: Listens to 'general'
    let bus_clone1 = Arc::clone(&bus);
    tokio::spawn(async move {
        let mut rx = bus_clone1.subscribe("general".to_string()).await;
        println!("   [Subscriber 1] Listening on #general");
        loop {
            match rx.recv().await {
                Ok(msg) => println!("   [Subscriber 1 / #general] Received: {}", String::from_utf8_lossy(&msg)),
                Err(e) => {
                    println!("   [Subscriber 1 / #general] Error: {:?}", e);
                    break;
                }
            }
        }
    });

    // Subscriber 2: Listens to 'general' AND 'random'
    let bus_clone2 = Arc::clone(&bus);
    tokio::spawn(async move {
        let mut rx_general = bus_clone2.subscribe("general".to_string()).await;
        let mut rx_random = bus_clone2.subscribe("random".to_string()).await;
        println!("   [Subscriber 2] Listening on #general and #random");
        
        loop {
            tokio::select! {
                Ok(msg) = rx_general.recv() => {
                    println!("   [Subscriber 2 / #general] Received: {}", String::from_utf8_lossy(&msg));
                },
                Ok(msg) = rx_random.recv() => {
                    println!("   [Subscriber 2 / #random] Received: {}", String::from_utf8_lossy(&msg));
                },
                else => break,
            }
        }
    });

    // Subscriber 3: Listens only to 'random'
    let bus_clone3 = Arc::clone(&bus);
    tokio::spawn(async move {
        let mut rx = bus_clone3.subscribe("random".to_string()).await;
        println!("   [Subscriber 3] Listening on #random");
        loop {
            match rx.recv().await {
                Ok(msg) => println!("   [Subscriber 3 / #random] Received: {}", String::from_utf8_lossy(&msg)),
                Err(e) => {
                    println!("   [Subscriber 3 / #random] Error: {:?}", e);
                    break;
                }
            }
        }
    });

    // Give subscribers a moment to start up
    sleep(Duration::from_millis(100)).await;
    println!("\nSubscribers are ready. Starting to publish messages...\n");

    // ============================================================================
    // DEMO: Publish messages
    // ============================================================================

    let msg1: Message = "Hello, everyone!".into();
    println!("Publishing '{}' to #general...", String::from_utf8_lossy(&msg1));
    let count1 = bus.publish("general".to_string(), msg1).await;
    println!("   -> Sent to {} subscribers.\n", count1);
    sleep(Duration::from_millis(200)).await;

    let msg2: Message = "Does anyone have a joke?".into();
    println!("Publishing '{}' to #random...", String::from_utf8_lossy(&msg2));
    let count2 = bus.publish("random".to_string(), msg2).await;
    println!("   -> Sent to {} subscribers.\n", count2);
    sleep(Duration::from_millis(200)).await;

    let msg3: Message = "This is an important announcement!".into();
    println!("Publishing '{}' to #general...", String::from_utf8_lossy(&msg3));
    let count3 = bus.publish("general".to_string(), msg3).await;
    println!("   -> Sent to {} subscribers.\n", count3);
    sleep(Duration::from_millis(200)).await;
    
    let msg4: Message = "This message is for a topic no one is listening to.".into();
    println!("Publishing to #private...");
    let count4 = bus.publish("private".to_string(), msg4).await;
    println!("   -> Sent to {} subscribers.\n", count4);
    sleep(Duration::from_millis(200)).await;


    println!("=== Demo Complete! ===");
    println!("(The program will hang here as subscriber tasks are still running.)");
    println!("(Press Ctrl+C to exit.)");
    
    // In a real app, you'd have a shutdown signal to gracefully end tasks.
    // For this demo, we let them run forever.
    // sleep(Duration::from_secs(1)).await; // Allow last messages to be printed
}