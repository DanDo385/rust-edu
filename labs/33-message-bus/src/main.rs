// Lab 33: Message Bus (Pub/Sub) - Demo
//
// An async publish-subscribe message bus using Tokio.
// Demonstrates event-driven architecture, async message passing, and channel-based
// communication. This is the foundation of microservices and event-driven systems.

use message_bus::MessageBus;

#[tokio::main]
async fn main() {
    println!("=== Async Message Bus (Pub/Sub) ===\n");

    // ============================================================================
    // BASIC PUB/SUB
    // ============================================================================
    demo_basic_pubsub().await;

    // ============================================================================
    // MULTIPLE SUBSCRIBERS
    // ============================================================================
    demo_multiple_subscribers().await;

    // ============================================================================
    // MULTIPLE TOPICS
    // ============================================================================
    demo_multiple_topics().await;

    // ============================================================================
    // UNSUBSCRIBE
    // ============================================================================
    demo_unsubscribe().await;
}

async fn demo_basic_pubsub() {
    println!("=== Basic Publish-Subscribe ===\n");

    let bus = MessageBus::new();

    // Subscribe to "news" topic
    let mut subscriber = bus.subscribe("news").await;

    // Publish some messages
    let count = bus.publish("news", "Breaking: Rust 2.0 announced!".to_string()).await;
    println!("Published to {} subscriber(s)", count);
    let count = bus.publish("news", "Tokio becomes sentient".to_string()).await;
    println!("Published to {} subscriber(s)", count);

    // Receive messages
    if let Some(msg) = subscriber.recv().await {
        println!("Subscriber received: {}", msg);
    }
    if let Some(msg) = subscriber.recv().await {
        println!("Subscriber received: {}", msg);
    }

    println!();
}

async fn demo_multiple_subscribers() {
    println!("=== Multiple Subscribers (Broadcast) ===\n");

    let bus = MessageBus::new();

    let mut sub1 = bus.subscribe("alerts").await;
    let mut sub2 = bus.subscribe("alerts").await;
    let mut sub3 = bus.subscribe("alerts").await;

    let count = bus.publish("alerts", "System alert: High CPU usage".to_string()).await;
    println!("Published to {} subscriber(s)", count);

    println!("Subscriber 1 received: {:?}", sub1.recv().await);
    println!("Subscriber 2 received: {:?}", sub2.recv().await);
    println!("Subscriber 3 received: {:?}", sub3.recv().await);

    println!();
}

async fn demo_multiple_topics() {
    println!("=== Multiple Topics ===\n");

    let bus = MessageBus::new();

    let mut orders_sub = bus.subscribe("orders").await;
    let mut payments_sub = bus.subscribe("payments").await;
    let mut users_sub = bus.subscribe("users").await;

    bus.publish("orders", "Order #123 created".to_string()).await;
    bus.publish("payments", "Payment $50 processed".to_string()).await;
    bus.publish("users", "User 'alice' registered".to_string()).await;

    println!("Orders: {:?}", orders_sub.recv().await);
    println!("Payments: {:?}", payments_sub.recv().await);
    println!("Users: {:?}", users_sub.recv().await);

    println!();
}

async fn demo_unsubscribe() {
    println!("=== Unsubscribe ===\n");

    let bus = MessageBus::new();

    let mut sub = bus.subscribe("events").await;

    bus.publish("events", "Event 1".to_string()).await;
    println!("Before unsubscribe: {:?}", sub.recv().await);

    // Unsubscribe by dropping the receiver
    drop(sub);

    let count = bus.publish("events", "Event 2 (not received)".to_string()).await;
    println!("After unsubscribe: published to {} subscriber(s)", count);

    println!();
}

// ============================================================================
// WHAT RUST DOES UNDER THE HOOD
// ============================================================================
//
// 1. ASYNC/AWAIT
//    - async fn returns a Future
//    - await suspends the future until result is ready
//    - Tokio scheduler multiplexes futures on threads
//    - Zero-cost abstraction (compiles to state machine)
//
// 2. MPSC CHANNEL (tokio)
//    - Lock-free queue implementation
//    - Sender clones increment reference count
//    - Bounded channel applies backpressure when full
//    - Very fast: ~100-500 ns per message
//
// 3. RWLOCK (tokio)
//    - Allows multiple readers OR one writer
//    - Async-aware (yields to other tasks when waiting)
//    - Used for topics HashMap (rarely written, often read)
//    - Faster than Mutex for read-heavy workloads
//
// 4. ARC (ATOMIC REFERENCE COUNTING)
//    - Thread-safe shared ownership
//    - Clone increments count atomically
//    - Dropped when count reaches zero
//    - Minimal overhead (~2 atomic operations per clone/drop)

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. Pub/sub decouples publishers from subscribers
// 2. Topic-based routing allows flexible message distribution
// 3. Async channels enable non-blocking message passing
// 4. RwLock is ideal for read-heavy data structures
// 5. Bounded channels prevent memory exhaustion
// 6. Background tasks process messages concurrently
// 7. Cleanup removes disconnected subscribers
// 8. In-process message bus is extremely fast (<10 us latency)
