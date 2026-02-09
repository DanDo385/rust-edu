# Project 33 - An Async Message Bus

## What You're Building (Plain English)

You're building a "message bus," a central hub for different parts of an application to communicate without knowing about each other directly. It's like a public announcement system. Some parts of your code can "publish" a message to a specific topic (like "weather-alerts"), and other parts can "subscribe" to that topic to receive all messages published to it.

This is also known as the "publish-subscribe" (or "pub/sub") pattern. You'll build this system using Rust's powerful `async` capabilities with the `tokio` runtime, so it can handle many subscribers and topics concurrently with high performance.

## New Rust Concepts in This Project

-   **`async`/`.await`**: You'll write asynchronous code that can perform I/O-bound tasks (like waiting for network messages) without blocking the entire thread.

-   **Tokio Runtime**: The most popular `async` runtime for Rust. You'll use `tokio::spawn` to run concurrent tasks.

-   **Tokio Channels**: Asynchronous channels for communication between tasks. We'll use `tokio::sync::broadcast` which is specifically designed for the pub/sub pattern where one sender has many receivers.

-   **Shared State with `Arc<Mutex>`**: To manage the list of topics and subscribers safely across multiple concurrent tasks, you'll wrap your central state in an `Arc<Mutex>`.
    -   `Arc` (Atomically-Reference-Counted): Allows multiple owners of the same data.
    -   `Mutex` (Mutual Exclusion): Ensures only one thread can access the data at a time.

-   **`async-trait` Crate**: This will allow you to define traits with `async` methods, which isn't directly supported in Rust yet. This helps create a clean, testable interface.

## Rust Syntax You'll See

```rust
use tokio::sync::{broadcast, Mutex};
use std::sync::Arc;
use async_trait::async_trait;

// A message could be as simple as bytes
type Message = bytes::Bytes;

// Our message bus state, shared safely across tasks
struct BusState {
    topics: std::collections::HashMap<String, broadcast::Sender<Message>>,
}
type SharedBusState = Arc<Mutex<BusState>>;

#[async_trait]
trait MessageBus {
    async fn publish(&self, topic: String, message: Message) -> usize;
    async fn subscribe(&self, topic: String) -> broadcast::Receiver<Message>;
}

// An async function
async fn my_async_function() {
    // .await pauses the function without blocking the thread
    let result = some_other_async_function().await;
}

// Spawning a concurrent task
// tokio::spawn(async move {
//     // This code runs in the background
// });
```

## How to Run

```bash
# Run the main binary (a demo of the message bus)
cargo run -p message-bus

# Run the tests
cargo test -p message-bus

# Check if code compiles
cargo check -p message-bus
```

## The Exercises

You will implement `MessageBus`.

1.  **`MessageBus` Struct**: Create the main struct that holds the shared state (`Arc<Mutex<...>>`). The state will contain a `HashMap` mapping topic names (`String`) to `broadcast::Sender<Message>`.

2.  **`new()`**: A constructor to create a new, empty message bus.

3.  **`subscribe()`**:
    -   This `async` method takes a topic name.
    -   It locks the shared state.
    -   If the topic already exists in the `HashMap`, it calls `subscribe()` on the existing `broadcast::Sender` to get a new `Receiver`.
    -   If the topic *doesn't* exist, it creates a *new* `broadcast` channel, stores the `Sender` part in the `HashMap`, and returns the `Receiver`.
    -   It returns the `broadcast::Receiver<Message>`.

4.  **`publish()`**:
    -   This `async` method takes a topic name and a `Message`.
    -   It locks the shared state.
    -   It looks up the topic in the `HashMap`.
    -   If the topic exists, it calls `send()` on the `broadcast::Sender`. The `send` method returns the number of subscribers that received the message.
    -   If the topic doesn't exist, no one is subscribed, so it does nothing and can return 0.
    -   It returns the number of subscribers the message was sent to.

## Solution Explanation (No Code - Just Ideas)

**The Shared State**: The core of the bus is the `Arc<Mutex<HashMap<String, broadcast::Sender<Message>>>>`.
-   `HashMap`: Maps a topic name like `"news"` to the entry point of a broadcast channel for that topic.
-   `broadcast::Sender`: The publisher's end of the channel. When you `send` on it, all corresponding `Receiver`s get the message.
-   `Mutex`: We need to lock the `HashMap` whenever we add a new topic or look one up. This prevents two threads from trying to create the same topic at the same time. The lock is held for a very short duration.
-   `Arc`: The `Arc` allows multiple publisher and subscriber tasks to all hold a reference to the same `Mutex`-protected `HashMap`, so they are all communicating through the one central state.

**`subscribe("sports")`**:
1.  Spawn a task for a subscriber.
2.  Inside the task, call `bus.subscribe("sports").await`.
3.  This locks the central `HashMap`.
4.  It checks if `"sports"` exists. Let's say it doesn't.
5.  It creates a new `broadcast` channel. The `Sender` is put into the `HashMap` under the key `"sports"`.
6.  The `Receiver` is returned to the subscriber task.
7.  The lock is released. The subscriber can now `.await` on its `Receiver` for messages.

**`publish("sports", msg)`**:
1.  Lock the central `HashMap`.
2.  Look up `"sports"`. It finds the `Sender` we created earlier.
3.  Call `.send(msg)` on that sender. The `broadcast` channel wakes up any waiting `Receiver`s.
4.  The lock is released.

## Where Rust Shines

-   **Fearless Concurrency**: The combination of `Arc`, `Mutex`, and `async`/`await` provides a powerful and safe way to build complex concurrent systems. The compiler prevents data races.
-   **Structured Concurrency**: Tokio's task model makes it easy to reason about concurrent operations.
-   **Performance**: Tokio's work-stealing scheduler is highly efficient, and broadcast channels are optimized for the one-to-many communication pattern.
-   **Type Safety**: The type system ensures you can only send valid `Message` types and that subscribers receive the correct type.

## Common Beginner Mistakes

1.  **Holding a `MutexGuard` across an `.await` point**:
    -   `let guard = self.state.lock().await;`
    -   `some_other_function().await;` // <-- Holding the lock here!
    -   This can cause deadlocks, as no other task can acquire the lock while this one is waiting.
    -   **Fix**: Keep lock durations short. Lock, perform a quick synchronous action, and then unlock before any `.await`.

2.  **Forgetting `Arc`**: Just using a `Mutex` isn't enough. To share the `Mutex` between threads/tasks, it must be wrapped in an `Arc`.

3.  **Channel selection**: Using the wrong type of channel. `tokio::sync::mpsc` is for multi-producer, single-consumer. For pub/sub, `broadcast` is the correct choice.

Enjoy building your message bus! This project is a great step into the world of asynchronous, concurrent systems programming in Rust.