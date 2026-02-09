//! # An Async Message Bus - Complete Solution
//!
//! ## What We're Building
//!
//! A `MessageBus` that facilitates the publish-subscribe pattern, allowing for
//! many-to-many asynchronous communication. It's built on Tokio and is designed
//! to be thread-safe and performant.
//!
//! ## Why Rust Is Perfect For This
//!
//! - **Fearless Concurrency**: Rust's ownership model, combined with `Arc` and `Mutex`,
//!   provides a safe way to manage shared state across asynchronous tasks.
//! - **Rich Ecosystem**: Tokio provides high-quality, production-ready primitives
//!   like `broadcast` channels that are perfect for this pattern.
//! - **`async`/`.await`**: Provides a way to write concurrent code that is almost as
//!   readable as synchronous code, without the complexities of callbacks or manual
//!   state machines.
//!
//! ## Key Rust Concepts You'll Learn
//!
//! - **`Arc<Mutex<T>>`**: The standard pattern for mutable state that is shared
//!   across multiple threads/tasks.
//! - **`tokio::sync::broadcast`**: A channel where a single sender can communicate
//!   with multiple receivers. When a value is sent, all receivers get a clone of it.
//! - **`HashMap::entry` API**: An efficient and idiomatic way to "get or insert"
//!   a value in a `HashMap`.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};
use bytes::Bytes;

/// A simple type alias for our messages. `Bytes` is a smart pointer for byte
/// slices that is efficient for cloning and passing between threads.
pub type Message = Bytes;

/// The shared state of the message bus.
///
/// It contains a `HashMap` where keys are topic names and values are the
/// `Sender` ends of broadcast channels.
struct BusState {
    topics: HashMap<String, broadcast::Sender<Message>>,
}

impl BusState {
    /// Creates a new, empty `BusState`.
    fn new() -> Self {
        BusState {
            topics: HashMap::new(),
        }
    }
}

/// The main `MessageBus` struct.
///
/// It is a lightweight handle that can be cloned and shared across tasks.
/// It holds a reference-counted pointer (`Arc`) to the shared, mutable state.
#[derive(Clone)]
pub struct MessageBus {
    state: Arc<Mutex<BusState>>,
}

impl MessageBus {
    /// Creates a new, empty `MessageBus`.
    pub fn new() -> Self {
        MessageBus {
            // Initialize the shared state, wrapped for concurrency.
            state: Arc::new(Mutex::new(BusState::new())),
        }
    }

    /// Subscribes to a topic, returning a `Receiver` to listen for messages.
    ///
    /// If the topic does not exist, it is created.
    pub async fn subscribe(&self, topic: String) -> broadcast::Receiver<Message> {
        // Lock the mutex to get mutable access to the state.
        // The `.await` here means this task will yield if the lock is currently
        // held by another task, allowing the thread to do other work.
        let mut state = self.state.lock().await;

        // Use the `entry` API to handle both cases (topic exists or not) elegantly.
        // `or_insert_with` will only execute the closure if the key is not present.
        let sender = state.topics.entry(topic).or_insert_with(|| {
            // If the topic is new, create a new broadcast channel.
            // A capacity of 32 is a reasonable default. If messages are sent
            // faster than a subscriber can read them, the oldest messages
            // will be dropped.
            let (sender, _) = broadcast::channel(32);
            sender
        });

        // Whether the sender was old or new, create a new receiver from it.
        // The `broadcast::Sender` can create multiple receivers.
        sender.subscribe()

        // The mutex guard `state` is dropped here, automatically unlocking the mutex.
    }

    /// Publishes a message to a topic.
    ///
    /// The message is sent to all active subscribers. Returns the number
    /// of subscribers the message was successfully sent to.
    pub async fn publish(&self, topic: String, message: Message) -> usize {
        // Lock the mutex to get read-only access to the state.
        let state = self.state.lock().await;

        // Try to get the sender for the given topic.
        if let Some(sender) = state.topics.get(&topic) {
            // If the sender exists, send the message.
            // `sender.send()` will send the message to all subscribed receivers.
            // It returns `Ok(count)` with the number of receivers that got the
            // message. If it returns `Err`, it means there are no receivers,
            // so we can treat that as 0.
            sender.send(message).unwrap_or(0)
        } else {
            // If the topic doesn't exist, no one is subscribed.
            0
        }
    }
}
