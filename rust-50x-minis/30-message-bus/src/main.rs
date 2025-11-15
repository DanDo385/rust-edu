// Project 30: Message Bus
//
// An async publish-subscribe message bus using Tokio.
// Demonstrates event-driven architecture, async message passing, and channel-based
// communication. This is the foundation of microservices and event-driven systems.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tokio::time::{sleep, Duration};

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
    bus.publish("news", "Breaking: Rust 2.0 announced!".to_string()).await;
    bus.publish("news", "Tokio becomes sentient".to_string()).await;

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

    // Multiple subscribers to same topic
    let mut sub1 = bus.subscribe("alerts").await;
    let mut sub2 = bus.subscribe("alerts").await;
    let mut sub3 = bus.subscribe("alerts").await;

    // Publish one message
    bus.publish("alerts", "System alert: High CPU usage".to_string()).await;

    // All subscribers receive it
    println!("Subscriber 1 received: {:?}", sub1.recv().await);
    println!("Subscriber 2 received: {:?}", sub2.recv().await);
    println!("Subscriber 3 received: {:?}", sub3.recv().await);

    println!();
}

async fn demo_multiple_topics() {
    println!("=== Multiple Topics ===\n");

    let bus = MessageBus::new();

    // Subscribe to different topics
    let mut orders_sub = bus.subscribe("orders").await;
    let mut payments_sub = bus.subscribe("payments").await;
    let mut users_sub = bus.subscribe("users").await;

    // Publish to different topics
    bus.publish("orders", "Order #123 created".to_string()).await;
    bus.publish("payments", "Payment $50 processed".to_string()).await;
    bus.publish("users", "User 'alice' registered".to_string()).await;

    // Each subscriber only receives its topic
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

    bus.publish("events", "Event 2 (not received)".to_string()).await;
    println!("After unsubscribe: No messages received");

    println!();
}

// ============================================================================
// MESSAGE TYPE
// ============================================================================
// For this simple implementation, messages are just Strings
// In production, use an enum or trait for different message types

type Message = String;

// ============================================================================
// MESSAGE BUS STRUCTURE
// ============================================================================

#[derive(Clone)]
struct MessageBus {
    // Map from topic to list of subscribers
    // RwLock allows multiple readers or one writer (for subscribing/unsubscribing)
    // Each subscriber gets messages via an mpsc channel
    topics: Arc<RwLock<HashMap<String, Vec<mpsc::Sender<Message>>>>>,
}

impl MessageBus {
    /// Creates a new message bus
    fn new() -> Self {
        MessageBus {
            topics: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Subscribes to a topic and returns a receiver for messages
    async fn subscribe(&self, topic: &str) -> mpsc::Receiver<Message> {
        // Create a channel for this subscriber
        // Bounded channel (capacity 100) to prevent unbounded memory growth
        let (tx, rx) = mpsc::channel(100);

        // Add this subscriber to the topic
        let mut topics = self.topics.write().await;
        topics
            .entry(topic.to_string())
            .or_insert_with(Vec::new)
            .push(tx);

        println!("Subscribed to topic: {}", topic);

        rx
    }

    /// Publishes a message to a topic
    ///
    /// All subscribers to this topic will receive the message
    async fn publish(&self, topic: &str, message: Message) {
        let topics = self.topics.read().await;

        if let Some(subscribers) = topics.get(topic) {
            println!("Publishing to topic '{}': {} ({} subscribers)",
                     topic, message, subscribers.len());

            // Send message to all subscribers
            for (i, subscriber) in subscribers.iter().enumerate() {
                // Send is async and can fail if receiver is dropped
                match subscriber.send(message.clone()).await {
                    Ok(_) => {}
                    Err(_) => {
                        // Subscriber has disconnected
                        // In production, clean up dead subscribers
                        println!("  Subscriber {} has disconnected", i);
                    }
                }
            }
        } else {
            println!("Publishing to topic '{}': {} (no subscribers)",
                     topic, message);
        }
    }

    /// Cleans up disconnected subscribers
    ///
    /// Call this periodically to remove closed channels
    async fn cleanup(&self) {
        let mut topics = self.topics.write().await;

        for (_topic, subscribers) in topics.iter_mut() {
            // Keep only subscribers whose channels are still open
            subscribers.retain(|sub| !sub.is_closed());
        }

        // Remove topics with no subscribers
        topics.retain(|_topic, subs| !subs.is_empty());
    }

    /// Returns statistics about the message bus
    async fn stats(&self) -> BusStats {
        let topics = self.topics.read().await;

        let topic_count = topics.len();
        let subscriber_count: usize = topics.values().map(|v| v.len()).sum();

        BusStats {
            topics: topic_count,
            subscribers: subscriber_count,
        }
    }
}

#[derive(Debug)]
struct BusStats {
    topics: usize,
    subscribers: usize,
}

// ============================================================================
// ADVANCED EXAMPLE: BACKGROUND WORKER PATTERN
// ============================================================================

async fn _demo_background_workers() {
    println!("=== Background Workers Pattern ===\n");

    let bus = MessageBus::new();

    // Spawn multiple background workers that process messages
    for worker_id in 0..3 {
        let mut subscriber = bus.subscribe("jobs").await;

        tokio::spawn(async move {
            while let Some(job) = subscriber.recv().await {
                println!("Worker {} processing: {}", worker_id, job);

                // Simulate work
                sleep(Duration::from_millis(100)).await;

                println!("Worker {} completed: {}", worker_id, job);
            }
        });
    }

    // Publish jobs
    for i in 0..10 {
        bus.publish("jobs", format!("Job #{}", i)).await;
        sleep(Duration::from_millis(50)).await;
    }

    // Wait for workers to finish
    sleep(Duration::from_secs(2)).await;

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
//
// 5. MESSAGE PASSING
//    - Messages are cloned for each subscriber (memory cost!)
//    - For large messages, use Arc<Message> to avoid cloning
//    - Channels are bounded to prevent memory exhaustion
//
// 6. CLEANUP
//    - is_closed() checks if receiver is dropped
//    - Automatically removes dead subscribers
//    - In production, run cleanup periodically in background task

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
// 8. In-process message bus is extremely fast (<10 μs latency)

// ============================================================================
// PUB/SUB PATTERNS
// ============================================================================
//
// 1. BROADCAST (one-to-many):
//    One publisher, multiple subscribers per topic
//    All subscribers receive every message
//    Our implementation uses this
//
// 2. WORK QUEUE (load balancing):
//    Multiple publishers, multiple workers
//    Each message consumed by ONE worker
//    Use tokio::sync::mpsc directly
//
// 3. REQUEST/REPLY:
//    Publisher sends request with reply-to topic
//    Subscriber sends response to reply-to topic
//    Implement with correlation IDs
//
// 4. TOPIC HIERARCHIES:
//    Topics like "orders.created", "orders.cancelled"
//    Subscribe with wildcards: "orders.*"
//    Implement with pattern matching

// ============================================================================
// PERFORMANCE ANALYSIS
// ============================================================================
//
// LATENCY (in-process):
// - Channel send/recv: ~100-500 ns
// - RwLock read: ~20-50 ns
// - HashMap lookup: ~50-100 ns
// - Total: ~500-1000 ns per message (<1 microsecond!)
//
// THROUGHPUT (single thread):
// - ~1-5 million messages/sec
// - Limited by channel and HashMap overhead
// - Real bottleneck: what subscribers do with messages
//
// MEMORY:
// - Each subscriber: 1 channel (small fixed overhead)
// - Each message: cloned N times (N = subscriber count)
// - Bounded channel: capacity × message_size per subscriber
// - Example: 100 subscribers, 100 capacity, 1KB msg = ~10 MB
//
// SCALABILITY:
// - RwLock allows concurrent reads (multiple publishes)
// - Each subscriber runs in separate task (parallel processing)
// - Limited by number of CPU cores for compute-bound subscribers

// ============================================================================
// COMPARISON WITH NETWORK MESSAGE BROKERS
// ============================================================================
//
// IN-PROCESS (our implementation):
// Pros: Extremely fast (<1 μs), no serialization, type-safe
// Cons: Single process only, no persistence, lost on crash
// Use: High-frequency events within one application
//
// REDIS PUB/SUB:
// Pros: Very fast (~100 μs), simple, cross-process
// Cons: No persistence, fire-and-forget, no backpressure
// Use: Cache invalidation, real-time notifications
//
// RABBITMQ:
// Pros: Durable, flexible routing, transactions, cross-language
// Cons: Slower (~1 ms), complex setup, requires broker
// Use: Microservices communication, reliable messaging
//
// KAFKA:
// Pros: High throughput, persistent, replay capability, partitioned
// Cons: Complex, higher latency (~5-10 ms), requires cluster
// Use: Event streaming, data pipelines, audit logs
//
// Choose based on requirements:
// - In-process: Speed, simplicity
// - Redis: Speed, cross-process, no persistence
// - RabbitMQ: Reliability, flexibility
// - Kafka: Scale, persistence, replay

// ============================================================================
// COMMON BEGINNER MISTAKES
// ============================================================================
// ❌ Forgetting to .await
//    bus.publish("topic", msg);  // Compiles but doesn't execute!
//    Fix: bus.publish("topic", msg).await;
//
// ❌ Blocking in async context
//    thread::sleep(Duration::from_secs(1));  // Blocks thread!
//    Fix: tokio::time::sleep(Duration::from_secs(1)).await;
//
// ❌ Using unbounded channels
//    mpsc::unbounded_channel();  // Can grow without limit!
//    Fix: mpsc::channel(capacity);
//
// ❌ Not cleaning up dead subscribers
//    Old subscribers accumulate, wasting memory
//    Fix: Periodically call cleanup() or check is_closed()
//
// ❌ Holding locks too long
//    let topics = self.topics.write().await;
//    expensive_operation();  // Other tasks blocked!
//    Fix: Release lock before expensive work

// ============================================================================
// EXTENDING THIS IMPLEMENTATION
// ============================================================================
//
// PRODUCTION IMPROVEMENTS:
//
// 1. PATTERN MATCHING
//    Subscribe to "orders.*" to match "orders.created", "orders.cancelled"
//    Use regex or glob patterns
//
// 2. MESSAGE FILTERING
//    Filter by message content, not just topic:
//    bus.subscribe_filter("orders", |msg| msg.amount > 1000).await
//
// 3. PRIORITY QUEUES
//    High-priority messages jump the queue
//    Use BinaryHeap or separate channels
//
// 4. DEAD LETTER QUEUE
//    Failed messages go to special topic for debugging
//    Implement with error handling in subscribers
//
// 5. MESSAGE REPLAY
//    Store recent messages for late subscribers
//    Use ring buffer or Vec with capacity limit
//
// 6. PERSISTENCE
//    Write messages to disk for durability
//    Implement append-only log (like Project 28)
//
// 7. MONITORING
//    Track: messages/sec, subscriber lag, error rate
//    Expose via Prometheus or logs
//
// 8. NETWORK TRANSPORT
//    Add TCP/UDP for cross-process communication
//    Serialize messages with serde

// ============================================================================
// TOKIO RUNTIME EXPLAINED
// ============================================================================
//
// #[tokio::main] expands to:
// fn main() {
//     tokio::runtime::Runtime::new().unwrap().block_on(async {
//         // Your async code here
//     })
// }
//
// TOKIO SCHEDULER:
// - Work-stealing thread pool (like Rayon)
// - Runs multiple tasks on N threads (default: num_cpus)
// - Tasks are Futures that can be suspended (.await)
// - Very efficient: Millions of tasks with minimal overhead
//
// ASYNC TASKS:
// - tokio::spawn creates a new task
// - Tasks run concurrently (not necessarily parallel)
// - Scheduler multiplexes tasks on threads
// - No context switch overhead (unlike OS threads)
//
// WHEN TO USE ASYNC:
// - I/O-bound: Network, disk, timers (async shines!)
// - CPU-bound: Use thread pool or rayon instead
// - Mixed: Use tokio::task::spawn_blocking for CPU work
