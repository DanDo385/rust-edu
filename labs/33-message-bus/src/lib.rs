//! # An Async Message Bus - Your Implementation
//!
//! This project is about building a concurrent, asynchronous, topic-based
//! message bus using Tokio.
//!
//! ## Your Task
//!
//! Implement the `MessageBus` and its methods.
//!
//! 1.  **`MessageBus` Struct**: This struct will hold the shared state of your bus.
//!     The state should be a `HashMap` mapping topic `String`s to broadcast
//!     senders. This state needs to be safely shared across threads, so you'll
//!     wrap it in `Arc<Mutex<...>>`.
//!
//! 2.  **`new()`**: A simple constructor for your `MessageBus`.
//!
//! 3.  **`subscribe()`**: An `async` method that allows a task to start listening
//!     to a topic. It should return a `broadcast::Receiver`. If the topic doesn't
//!     exist, it should be created on the fly.
//!
//! 4.  **`publish()`**: An `async` method that sends a message to all subscribers
//!     of a given topic.
//!
//! ## Running Your Code
//!
//! ```bash
//! cargo test -p message-bus
//! cargo run -p message-bus
//! ```
//!
//! ## Stuck?
//!
//! Check out `src/solution.rs` for a complete, heavily-commented solution.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};
use bytes::Bytes;

/// A simple type alias for messages. `Bytes` is an efficient, clonable
/// view over a byte buffer.
pub type Message = Bytes;

// TODO: Define the shared state of the message bus.
// It should be a struct that contains a `HashMap` mapping topic names (String)
// to `broadcast::Sender<Message>`.
//
// struct BusState {
//     topics: HashMap<String, broadcast::Sender<Message>>,
// }
struct BusState {
    topics: HashMap<String, broadcast::Sender<Message>>,
}

// TODO: Define the MessageBus struct.
// It should contain the shared state, wrapped in `Arc<Mutex<...>>` for
// thread-safe sharing.
//
// #[derive(Clone)]
// pub struct MessageBus {
//     state: Arc<Mutex<BusState>>,
// }
#[derive(Clone)]
pub struct MessageBus {
    state: Arc<Mutex<BusState>>,
}


impl MessageBus {
    /// Creates a new, empty `MessageBus`.
    pub fn new() -> Self {
        // TODO: Initialize the `MessageBus` with a new, empty state.
        // The state should be wrapped in an `Arc` and a `Mutex`.
        todo!("Initialize MessageBus with empty state");
    }

    /// Subscribes to a topic, returning a `Receiver` to listen for messages.
    ///
    /// If the topic does not exist, it is created.
    pub async fn subscribe(&self, topic: String) -> broadcast::Receiver<Message> {
        // TODO: Implement the subscribe logic.
        // 1. Lock the mutex to get access to the state.
        //    `let mut state = self.state.lock().await;`
        //
        // 2. Check if the topic exists in the `HashMap` using `entry()`.
        //    The `entry()` API is perfect for this "get or insert" logic.
        //    `state.topics.entry(topic).or_insert_with(|| ... )`
        //
        // 3. If the topic is new, create a new `broadcast::channel` and
        //    store the `Sender` part.
        //
        // 4. Call `subscribe()` on the `Sender` (whether it's old or new)
        //    to get a new `Receiver`.
        //
        // 5. Return the `Receiver`. The `MutexGuard` is automatically
        //    unlocked when it goes out of scope.
        todo!("Implement the subscribe method");
    }

    /// Publishes a message to a topic.
    ///
    /// The message is sent to all active subscribers. Returns the number
    /// of subscribers the message was sent to.
    pub async fn publish(&self, topic: String, message: Message) -> usize {
        // TODO: Implement the publish logic.
        // 1. Lock the mutex to get access to the state.
        //    `let state = self.state.lock().await;`
        //
        // 2. Find the `Sender` for the given topic in the `HashMap`.
        //
        // 3. If a `Sender` exists, call its `send()` method. The `send()`
        //    method returns a `Result`. If `Ok`, the value inside is the
        //    number of receivers the message was sent to. If no one is
        //    listening, `send` will return an `Err`, in which case you
        //    can return 0.
        //
        // 4. If the topic does not exist, no one is subscribed, so return 0.
        todo!("Implement the publish method");
    }
}


// Re-export the solution module so people can compare
#[doc(hidden)]
pub mod solution;
