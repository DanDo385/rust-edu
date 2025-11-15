# Project 30: Message Bus

## Overview
This project implements an async publish-subscribe (pub/sub) message bus using Tokio. You'll learn about event-driven architectures, async message passing, and how systems like Redis Pub/Sub, RabbitMQ, and Kafka work. This is the foundation of microservices and event-driven systems.

## Concepts Taught
- **Publish-subscribe pattern** (pub/sub)
- **Async message passing** with tokio channels
- **Channel-based architecture** for decoupling
- **Multiple publishers** and **multiple subscribers**
- **Topic-based routing** (pattern matching)
- **Broadcast channels** for one-to-many
- **Backpressure handling** with bounded channels
- **Async/await** with Tokio runtime

## Why Message Bus?

### The Problem: Tight Coupling
Without a message bus, services communicate directly:
```
Service A → Service B
Service A → Service C
Service A → Service D
```

Problems:
- Each service needs to know about all others
- Adding a new consumer requires changing producers
- Synchronous coupling (A waits for B, C, D)
- Failure in one affects all

### The Solution: Message Bus
Services communicate through a central bus:
```
Service A → Message Bus → Service B
                        → Service C
                        → Service D
```

Benefits:
- **Decoupling**: Producers don't know about consumers
- **Scalability**: Add consumers without changing producers
- **Reliability**: Failed consumers don't affect producers
- **Asynchronous**: Non-blocking communication
- **Flexibility**: Route messages based on topics

**Real-world usage:**
- **RabbitMQ/Kafka**: Message brokers for microservices
- **Redis Pub/Sub**: Fast in-memory message bus
- **NATS**: Cloud-native messaging system
- **Event sourcing**: Domain events between bounded contexts
- **Webhooks**: Notify subscribers of events

## Architecture

```
MessageBus
├── topics: HashMap<String, Vec<Sender>>
├── command_rx: Receiver<BusCommand>
└── run in background task

BusCommand:
├── Subscribe { topic, sender }
├── Unsubscribe { topic, id }
└── Publish { topic, message }

Flow:
Publisher → publish("topic", msg) → BusCommand::Publish
                                   → Bus broadcasts to subscribers
                                   → Subscriber receives message
```

## Pub/Sub Patterns

### 1. Topic-Based (our implementation)
```rust
// Publisher
bus.publish("orders.created", order).await;

// Subscriber
bus.subscribe("orders.*").await;  // Pattern matching
```

### 2. Content-Based
```rust
// Filter by message content
bus.subscribe_filter(|msg| msg.amount > 1000).await;
```

### 3. Queue-Based (one consumer per message)
```rust
// Load balancing across consumers
bus.subscribe_queue("work.queue").await;
```

## Beginner Pitfalls & Async Notes

### Pitfall 1: Forgetting to Await
```rust
bus.publish("topic", msg);  // ❌ Returns Future, doesn't execute!
bus.publish("topic", msg).await;  // ✅ Actually publishes
```

### Pitfall 2: Blocking in Async Context
```rust
async fn subscriber() {
    let msg = rx.recv().await;
    thread::sleep(Duration::from_secs(1));  // ❌ Blocks entire runtime!
    tokio::time::sleep(Duration::from_secs(1)).await;  // ✅ Yields to other tasks
}
```

### Pitfall 3: Unbounded Channel Growth
```rust
// Unbounded channel can grow without limit
let (tx, rx) = mpsc::unbounded_channel();  // ❌ Memory leak risk!

// Bounded channel applies backpressure
let (tx, rx) = mpsc::channel(100);  // ✅ Limits queue size
```

### Pitfall 4: Forgetting to Spawn Background Task
```rust
let bus = MessageBus::new();
// ❌ Bus doesn't run, messages never delivered!

let bus = MessageBus::new();
tokio::spawn(async move { bus.run().await });  // ✅ Runs in background
```

## Code Walkthrough

See `src/main.rs` for a detailed implementation that demonstrates:
1. Creating an async message bus with Tokio
2. Publishing messages to topics
3. Subscribing to topics and receiving messages
4. Multiple publishers and subscribers
5. Pattern matching for topic routing
6. Graceful shutdown and cleanup
7. Error handling in async context

## Performance Considerations

### Channel Performance
- **mpsc::channel**: ~100-500 ns per message
- **broadcast::channel**: ~200-1000 ns per message (one-to-many)
- **Tokio channels**: Lock-free, very fast
- **Bounded vs unbounded**: Bounded is safer (prevents OOM)

### Memory Usage
- **Unbounded channel**: Can grow to GB if consumers are slow
- **Bounded channel**: Fixed memory (size × message size)
- **1000 message buffer, 1KB per message**: ~1 MB

### Throughput
- **Single thread**: ~1-5 million messages/sec
- **Multi-threaded**: ~10-50 million messages/sec
- **Network overhead**: ~10-100k messages/sec (TCP)

### Latency
- **In-process**: ~1-10 μs (microseconds)
- **Localhost TCP**: ~100 μs
- **Cross-datacenter**: ~10-100 ms

## Comparison: Rust vs Message Brokers

| Feature | Our Bus | Redis Pub/Sub | RabbitMQ | Kafka |
|---------|---------|---------------|----------|-------|
| Type | In-process | In-memory | Queue-based | Log-based |
| Persistence | No | No | Optional | Yes |
| Ordering | FIFO per topic | FIFO | FIFO per queue | Partition order |
| Backpressure | Bounded channel | No | Yes | Yes |
| Throughput | 10M msg/s | 1M msg/s | 100K msg/s | 1M msg/s |
| Latency | <10 μs | ~100 μs | ~1 ms | ~5 ms |
| Use case | Single process | Cache + pubsub | Microservices | Event streaming |

**Rust advantage**: In-process bus is 100-1000x faster than network brokers.

## Additional Challenges

1. **Pattern Matching**: Support wildcard subscriptions (`orders.*`, `*.created`)

2. **Message Filtering**: Filter by message content, not just topic

3. **Persistent Messages**: Store messages to disk for durability

4. **Dead Letter Queue**: Route failed messages to special topic

5. **Message Replay**: Allow subscribers to replay old messages

6. **Request/Reply**: Implement RPC over the message bus

7. **Network Transport**: Add TCP/UDP for cross-process communication

## Real-World Message Bus Features

Production message buses add:
- **Durability**: Persist messages to disk (WAL)
- **Replication**: Multi-node for high availability
- **Partitioning**: Distribute load across nodes
- **Ordering guarantees**: Per-partition or total order
- **Delivery semantics**: At-most-once, at-least-once, exactly-once
- **Monitoring**: Metrics, lag tracking, alerting
- **Schema validation**: Enforce message structure
- **Compression**: Reduce bandwidth usage

## Future Directions

- **Next**: Continue with advanced projects
- **Related**: Async/await (Project 20), channels (Project 18), thread pool (Project 26)
- **Advanced**: Build distributed message broker with Raft consensus

## Running This Project

```bash
cd 30-message-bus
cargo run
```

**Note**: Add to `Cargo.toml`:
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

## Expected Output

You should see:
- Message bus initialization
- Publishers sending messages to various topics
- Subscribers receiving and processing messages
- Multiple subscribers on same topic (broadcast)
- Unsubscribe and cleanup
- Statistics (messages sent/received, active subscribers)
