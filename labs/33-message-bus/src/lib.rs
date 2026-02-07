// Lab 33: Message Bus (Pub/Sub) - Library
//
// An async publish-subscribe message bus using Tokio.
// Demonstrates event-driven architecture, async message passing, and channel-based
// communication. This is the foundation of microservices and event-driven systems.
//
// ============================================================================
// OWNERSHIP & MEMORY MODEL
// ============================================================================
// The MessageBus uses Arc<RwLock<HashMap<String, Vec<Sender>>>> to allow:
// - Arc: multiple owners across async tasks (shared ownership, thread-safe)
// - RwLock: multiple readers OR one writer at a time (tokio async-aware)
// - HashMap: maps topic names (String) to subscriber lists
// - Vec<Sender>: each subscriber has an mpsc::Sender channel endpoint
//
// Messages are cloned for each subscriber. For large messages, wrapping in
// Arc<Message> avoids expensive cloning. The bounded channel (capacity 100)
// applies backpressure to prevent unbounded memory growth.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};

// ============================================================================
// MESSAGE TYPE
// ============================================================================
// For this implementation, messages are Strings.
// In production, use an enum or trait object for different message types.

/// The type used for messages in the bus. Currently a simple String.
pub type Message = String;

// ============================================================================
// BUS STATISTICS
// ============================================================================

/// Statistics about the current state of the message bus.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BusStats {
    /// Number of topics that have at least one subscriber.
    pub topics: usize,
    /// Total number of subscribers across all topics.
    pub subscribers: usize,
}

// ============================================================================
// MESSAGE BUS STRUCTURE
// ============================================================================

/// An async publish-subscribe message bus.
///
/// The bus routes messages by topic: publishers send to a topic name,
/// and all subscribers on that topic receive a copy. This is the
/// "broadcast" (one-to-many) pattern.
///
/// Thread-safe and cloneable thanks to Arc internals. Clone a MessageBus
/// to share it across async tasks.
#[derive(Clone)]
pub struct MessageBus {
    /// Map from topic name to list of subscriber channel senders.
    /// RwLock allows concurrent reads (publish) with exclusive writes (subscribe).
    topics: Arc<RwLock<HashMap<String, Vec<mpsc::Sender<Message>>>>>,
}

impl MessageBus {
    /// Creates a new, empty message bus with no topics or subscribers.
    pub fn new() -> Self {
        MessageBus {
            topics: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Subscribes to a topic, returning an mpsc::Receiver for incoming messages.
    ///
    /// The returned receiver will receive all messages published to `topic`
    /// after this subscription is created. The internal channel is bounded
    /// with a capacity of 100 to apply backpressure.
    ///
    /// Dropping the returned Receiver effectively unsubscribes (the Sender
    /// will detect a closed channel on the next publish).
    pub async fn subscribe(&self, topic: &str) -> mpsc::Receiver<Message> {
        let (tx, rx) = mpsc::channel(100);

        let mut topics = self.topics.write().await;
        topics
            .entry(topic.to_string())
            .or_insert_with(Vec::new)
            .push(tx);

        rx
    }

    /// Publishes a message to all subscribers of the given topic.
    ///
    /// If the topic has no subscribers, the message is silently dropped.
    /// If a subscriber's channel is closed (receiver dropped), that send
    /// fails silently -- use `cleanup()` to remove dead subscribers.
    ///
    /// Returns the number of subscribers that successfully received the message.
    pub async fn publish(&self, topic: &str, message: Message) -> usize {
        let topics = self.topics.read().await;

        let mut delivered = 0;
        if let Some(subscribers) = topics.get(topic) {
            for subscriber in subscribers.iter() {
                if subscriber.send(message.clone()).await.is_ok() {
                    delivered += 1;
                }
            }
        }

        delivered
    }

    /// Removes disconnected subscribers (those whose Receiver has been dropped).
    ///
    /// Also removes topics that have no remaining subscribers.
    /// Call this periodically in long-running applications to prevent
    /// accumulation of dead subscriber entries.
    pub async fn cleanup(&self) {
        let mut topics = self.topics.write().await;

        for (_topic, subscribers) in topics.iter_mut() {
            subscribers.retain(|sub| !sub.is_closed());
        }

        topics.retain(|_topic, subs| !subs.is_empty());
    }

    /// Returns statistics about the current state of the bus.
    ///
    /// Note: subscriber counts include disconnected subscribers that
    /// have not yet been cleaned up.
    pub async fn stats(&self) -> BusStats {
        let topics = self.topics.read().await;

        let topic_count = topics.len();
        let subscriber_count: usize = topics.values().map(|v| v.len()).sum();

        BusStats {
            topics: topic_count,
            subscribers: subscriber_count,
        }
    }

    /// Returns the list of topic names that currently have subscribers.
    pub async fn topic_names(&self) -> Vec<String> {
        let topics = self.topics.read().await;
        topics.keys().cloned().collect()
    }
}

impl Default for MessageBus {
    fn default() -> Self {
        Self::new()
    }
}
