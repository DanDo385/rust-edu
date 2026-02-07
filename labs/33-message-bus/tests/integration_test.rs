// Lab 33: Message Bus - Integration Tests
//
// Tests for the async publish-subscribe message bus.
// Uses #[tokio::test] for async test functions.
// Covers: basic pub/sub, multiple subscribers, multiple topics,
// unsubscribe, cleanup, statistics, and edge cases.

use message_bus::{BusStats, MessageBus};

// ============================================================================
// BASIC PUB/SUB
// ============================================================================

#[tokio::test]
async fn test_basic_publish_subscribe() {
    let bus = MessageBus::new();
    let mut sub = bus.subscribe("news").await;

    bus.publish("news", "Hello, world!".to_string()).await;

    let msg = sub.recv().await;
    assert_eq!(msg, Some("Hello, world!".to_string()));
}

#[tokio::test]
async fn test_publish_multiple_messages() {
    let bus = MessageBus::new();
    let mut sub = bus.subscribe("events").await;

    bus.publish("events", "Event 1".to_string()).await;
    bus.publish("events", "Event 2".to_string()).await;
    bus.publish("events", "Event 3".to_string()).await;

    assert_eq!(sub.recv().await, Some("Event 1".to_string()));
    assert_eq!(sub.recv().await, Some("Event 2".to_string()));
    assert_eq!(sub.recv().await, Some("Event 3".to_string()));
}

#[tokio::test]
async fn test_publish_returns_delivery_count() {
    let bus = MessageBus::new();
    let _sub1 = bus.subscribe("topic").await;
    let _sub2 = bus.subscribe("topic").await;

    let count = bus.publish("topic", "msg".to_string()).await;
    assert_eq!(count, 2);
}

#[tokio::test]
async fn test_publish_to_nonexistent_topic() {
    let bus = MessageBus::new();

    // No subscribers on this topic
    let count = bus.publish("ghost", "nobody listens".to_string()).await;
    assert_eq!(count, 0);
}

#[tokio::test]
async fn test_publish_empty_message() {
    let bus = MessageBus::new();
    let mut sub = bus.subscribe("topic").await;

    bus.publish("topic", "".to_string()).await;

    assert_eq!(sub.recv().await, Some("".to_string()));
}

// ============================================================================
// MULTIPLE SUBSCRIBERS (BROADCAST)
// ============================================================================

#[tokio::test]
async fn test_broadcast_to_multiple_subscribers() {
    let bus = MessageBus::new();

    let mut sub1 = bus.subscribe("alerts").await;
    let mut sub2 = bus.subscribe("alerts").await;
    let mut sub3 = bus.subscribe("alerts").await;

    bus.publish("alerts", "Alert!".to_string()).await;

    // All subscribers should receive the same message
    assert_eq!(sub1.recv().await, Some("Alert!".to_string()));
    assert_eq!(sub2.recv().await, Some("Alert!".to_string()));
    assert_eq!(sub3.recv().await, Some("Alert!".to_string()));
}

#[tokio::test]
async fn test_subscriber_only_receives_after_subscribe() {
    let bus = MessageBus::new();

    // Publish BEFORE subscribing
    bus.publish("topic", "before".to_string()).await;

    let mut sub = bus.subscribe("topic").await;

    // Publish AFTER subscribing
    bus.publish("topic", "after".to_string()).await;

    // Subscriber should only receive the message published after subscribing
    let msg = sub.recv().await;
    assert_eq!(msg, Some("after".to_string()));
}

// ============================================================================
// MULTIPLE TOPICS
// ============================================================================

#[tokio::test]
async fn test_multiple_topics_isolation() {
    let bus = MessageBus::new();

    let mut orders_sub = bus.subscribe("orders").await;
    let mut payments_sub = bus.subscribe("payments").await;

    bus.publish("orders", "Order #1".to_string()).await;
    bus.publish("payments", "Payment $100".to_string()).await;

    // Each subscriber only gets messages from their topic
    assert_eq!(orders_sub.recv().await, Some("Order #1".to_string()));
    assert_eq!(payments_sub.recv().await, Some("Payment $100".to_string()));
}

#[tokio::test]
async fn test_subscriber_on_one_topic_ignores_other() {
    let bus = MessageBus::new();

    let mut sub_a = bus.subscribe("topic_a").await;

    // Publish to a different topic
    bus.publish("topic_b", "wrong topic".to_string()).await;
    // Publish to the subscribed topic
    bus.publish("topic_a", "right topic".to_string()).await;

    // sub_a should only get its own topic's messages
    assert_eq!(sub_a.recv().await, Some("right topic".to_string()));
}

#[tokio::test]
async fn test_subscriber_to_multiple_topics() {
    let bus = MessageBus::new();

    // One logical subscriber can subscribe to multiple topics
    let mut sub_orders = bus.subscribe("orders").await;
    let mut sub_users = bus.subscribe("users").await;

    bus.publish("orders", "New order".to_string()).await;
    bus.publish("users", "New user".to_string()).await;

    assert_eq!(sub_orders.recv().await, Some("New order".to_string()));
    assert_eq!(sub_users.recv().await, Some("New user".to_string()));
}

// ============================================================================
// UNSUBSCRIBE (by dropping receiver)
// ============================================================================

#[tokio::test]
async fn test_unsubscribe_by_dropping_receiver() {
    let bus = MessageBus::new();

    let sub = bus.subscribe("events").await;

    // Drop the receiver to unsubscribe
    drop(sub);

    // Publish after unsubscribe - delivery count should be 0
    // (the send will fail because receiver is closed)
    let count = bus.publish("events", "orphan message".to_string()).await;
    assert_eq!(count, 0);
}

#[tokio::test]
async fn test_partial_unsubscribe() {
    let bus = MessageBus::new();

    let mut sub1 = bus.subscribe("topic").await;
    let sub2 = bus.subscribe("topic").await;
    let mut sub3 = bus.subscribe("topic").await;

    // Drop sub2 only
    drop(sub2);

    // sub1 and sub3 should still receive
    let count = bus.publish("topic", "Hello".to_string()).await;
    assert_eq!(count, 2); // 2 out of 3 delivered

    assert_eq!(sub1.recv().await, Some("Hello".to_string()));
    assert_eq!(sub3.recv().await, Some("Hello".to_string()));
}

#[tokio::test]
async fn test_receive_before_and_after_other_unsubscribes() {
    let bus = MessageBus::new();

    let mut sub1 = bus.subscribe("topic").await;
    let mut sub2 = bus.subscribe("topic").await;

    bus.publish("topic", "msg 1".to_string()).await;

    assert_eq!(sub1.recv().await, Some("msg 1".to_string()));
    assert_eq!(sub2.recv().await, Some("msg 1".to_string()));

    // sub2 unsubscribes
    drop(sub2);

    bus.publish("topic", "msg 2".to_string()).await;

    // sub1 still receives
    assert_eq!(sub1.recv().await, Some("msg 2".to_string()));
}

// ============================================================================
// CLEANUP
// ============================================================================

#[tokio::test]
async fn test_cleanup_removes_dead_subscribers() {
    let bus = MessageBus::new();

    let sub1 = bus.subscribe("topic").await;
    let _sub2 = bus.subscribe("topic").await;

    // Before cleanup: 2 subscribers
    let stats = bus.stats().await;
    assert_eq!(stats.subscribers, 2);

    // Drop sub1
    drop(sub1);

    // Run cleanup
    bus.cleanup().await;

    let stats = bus.stats().await;
    assert_eq!(stats.subscribers, 1);
}

#[tokio::test]
async fn test_cleanup_removes_empty_topics() {
    let bus = MessageBus::new();

    let sub = bus.subscribe("temp_topic").await;

    let stats = bus.stats().await;
    assert_eq!(stats.topics, 1);

    // Drop all subscribers for the topic
    drop(sub);

    bus.cleanup().await;

    let stats = bus.stats().await;
    assert_eq!(stats.topics, 0);
    assert_eq!(stats.subscribers, 0);
}

#[tokio::test]
async fn test_cleanup_preserves_live_subscribers() {
    let bus = MessageBus::new();

    let _live = bus.subscribe("topic").await;
    let dead = bus.subscribe("topic").await;
    drop(dead);

    bus.cleanup().await;

    let stats = bus.stats().await;
    assert_eq!(stats.topics, 1);
    assert_eq!(stats.subscribers, 1);
}

// ============================================================================
// STATISTICS
// ============================================================================

#[tokio::test]
async fn test_stats_empty_bus() {
    let bus = MessageBus::new();

    let stats = bus.stats().await;
    assert_eq!(stats, BusStats { topics: 0, subscribers: 0 });
}

#[tokio::test]
async fn test_stats_with_subscribers() {
    let bus = MessageBus::new();

    let _sub1 = bus.subscribe("topic_a").await;
    let _sub2 = bus.subscribe("topic_a").await;
    let _sub3 = bus.subscribe("topic_b").await;

    let stats = bus.stats().await;
    assert_eq!(stats.topics, 2);
    assert_eq!(stats.subscribers, 3);
}

#[tokio::test]
async fn test_stats_after_unsubscribe_without_cleanup() {
    let bus = MessageBus::new();

    let sub = bus.subscribe("topic").await;

    let stats = bus.stats().await;
    assert_eq!(stats.subscribers, 1);

    drop(sub);

    // Without cleanup, stats still show the dead subscriber
    let stats = bus.stats().await;
    assert_eq!(stats.subscribers, 1);
}

// ============================================================================
// TOPIC NAMES
// ============================================================================

#[tokio::test]
async fn test_topic_names_empty() {
    let bus = MessageBus::new();
    let names = bus.topic_names().await;
    assert!(names.is_empty());
}

#[tokio::test]
async fn test_topic_names_lists_all() {
    let bus = MessageBus::new();

    let _sub1 = bus.subscribe("alpha").await;
    let _sub2 = bus.subscribe("beta").await;
    let _sub3 = bus.subscribe("gamma").await;

    let mut names = bus.topic_names().await;
    names.sort();
    assert_eq!(names, vec!["alpha", "beta", "gamma"]);
}

// ============================================================================
// CLONE / SHARING
// ============================================================================

#[tokio::test]
async fn test_cloned_bus_shares_state() {
    let bus = MessageBus::new();
    let bus_clone = bus.clone();

    // Subscribe via original
    let mut sub = bus.subscribe("shared").await;

    // Publish via clone
    bus_clone.publish("shared", "from clone".to_string()).await;

    assert_eq!(sub.recv().await, Some("from clone".to_string()));
}

#[tokio::test]
async fn test_cloned_bus_subscribe_via_clone() {
    let bus = MessageBus::new();
    let bus_clone = bus.clone();

    // Subscribe via clone
    let mut sub = bus_clone.subscribe("topic").await;

    // Publish via original
    bus.publish("topic", "from original".to_string()).await;

    assert_eq!(sub.recv().await, Some("from original".to_string()));
}

// ============================================================================
// DEFAULT TRAIT
// ============================================================================

#[tokio::test]
async fn test_default_creates_empty_bus() {
    let bus = MessageBus::default();
    let stats = bus.stats().await;
    assert_eq!(stats, BusStats { topics: 0, subscribers: 0 });
}

// ============================================================================
// ORDERING / FIFO
// ============================================================================

#[tokio::test]
async fn test_messages_received_in_order() {
    let bus = MessageBus::new();
    let mut sub = bus.subscribe("ordered").await;

    for i in 0..10 {
        bus.publish("ordered", format!("msg-{}", i)).await;
    }

    for i in 0..10 {
        assert_eq!(sub.recv().await, Some(format!("msg-{}", i)));
    }
}

// ============================================================================
// EDGE CASES
// ============================================================================

#[tokio::test]
async fn test_subscribe_same_topic_twice() {
    let bus = MessageBus::new();

    // Same "logical subscriber" subscribing twice to same topic
    // gets two separate receivers
    let mut sub1 = bus.subscribe("topic").await;
    let mut sub2 = bus.subscribe("topic").await;

    bus.publish("topic", "msg".to_string()).await;

    // Both receive the message (they are independent subscriptions)
    assert_eq!(sub1.recv().await, Some("msg".to_string()));
    assert_eq!(sub2.recv().await, Some("msg".to_string()));
}

#[tokio::test]
async fn test_topic_with_special_characters() {
    let bus = MessageBus::new();
    let mut sub = bus.subscribe("topic/with.special-chars_and spaces!").await;

    bus.publish("topic/with.special-chars_and spaces!", "works".to_string()).await;

    assert_eq!(sub.recv().await, Some("works".to_string()));
}

#[tokio::test]
async fn test_empty_topic_name() {
    let bus = MessageBus::new();
    let mut sub = bus.subscribe("").await;

    bus.publish("", "empty topic".to_string()).await;

    assert_eq!(sub.recv().await, Some("empty topic".to_string()));
}

#[tokio::test]
async fn test_many_subscribers() {
    let bus = MessageBus::new();

    let mut subscribers = Vec::new();
    for _ in 0..50 {
        subscribers.push(bus.subscribe("mass").await);
    }

    let count = bus.publish("mass", "broadcast".to_string()).await;
    assert_eq!(count, 50);

    for sub in &mut subscribers {
        assert_eq!(sub.recv().await, Some("broadcast".to_string()));
    }
}

#[tokio::test]
async fn test_publish_after_all_unsubscribe() {
    let bus = MessageBus::new();

    let sub1 = bus.subscribe("topic").await;
    let sub2 = bus.subscribe("topic").await;

    drop(sub1);
    drop(sub2);

    // All subscribers gone
    let count = bus.publish("topic", "nobody home".to_string()).await;
    assert_eq!(count, 0);
}

// ============================================================================
// CONCURRENT USAGE (tokio::spawn)
// ============================================================================

#[tokio::test]
async fn test_publish_from_spawned_task() {
    let bus = MessageBus::new();
    let mut sub = bus.subscribe("async_topic").await;

    let bus_clone = bus.clone();
    tokio::spawn(async move {
        bus_clone.publish("async_topic", "from task".to_string()).await;
    });

    let msg = sub.recv().await;
    assert_eq!(msg, Some("from task".to_string()));
}

#[tokio::test]
async fn test_subscribe_from_spawned_task() {
    let bus = MessageBus::new();
    let bus_clone = bus.clone();

    let handle = tokio::spawn(async move {
        let mut sub = bus_clone.subscribe("spawned").await;
        sub.recv().await
    });

    // Give the spawned task a moment to subscribe
    tokio::task::yield_now().await;

    bus.publish("spawned", "to spawned subscriber".to_string()).await;

    let result = handle.await.unwrap();
    assert_eq!(result, Some("to spawned subscriber".to_string()));
}
