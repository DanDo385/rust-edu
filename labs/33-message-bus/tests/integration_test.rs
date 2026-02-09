//! Integration tests for Lab 33: Message Bus
//!
//! These tests verify the async publish-subscribe message bus.
//! Uses #[tokio::test] for async test functions.

use message_bus::solution::{MessageBus, Message};
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn test_basic_publish_subscribe() {
    let bus = MessageBus::new();
    let mut rx = bus.subscribe("topic1".to_string()).await;

    let msg: Message = "hello".into();
    bus.publish("topic1".to_string(), msg.clone()).await;

    let received = rx.recv().await.unwrap();
    assert_eq!(received, msg);
}

#[tokio::test]
async fn test_publish_multiple_messages() {
    let bus = MessageBus::new();
    let mut rx = bus.subscribe("topic1".to_string()).await;

    bus.publish("topic1".to_string(), "msg1".into()).await;
    bus.publish("topic1".to_string(), "msg2".into()).await;

    assert_eq!(rx.recv().await.unwrap(), Message::from("msg1"));
    assert_eq!(rx.recv().await.unwrap(), Message::from("msg2"));
}

#[tokio::test]
async fn test_publish_returns_delivery_count() {
    let bus = MessageBus::new();
    let _rx1 = bus.subscribe("topic1".to_string()).await;
    let _rx2 = bus.subscribe("topic1".to_string()).await;

    let count = bus.publish("topic1".to_string(), "hello".into()).await;
    assert_eq!(count, 2);
}

#[tokio::test]
async fn test_publish_to_nonexistent_topic() {
    let bus = MessageBus::new();
    let count = bus.publish("topic1".to_string(), "hello".into()).await;
    assert_eq!(count, 0);
}

#[tokio::test]
async fn test_broadcast_to_multiple_subscribers() {
    let bus = MessageBus::new();
    let mut rx1 = bus.subscribe("topic1".to_string()).await;
    let mut rx2 = bus.subscribe("topic1".to_string()).await;

    bus.publish("topic1".to_string(), "hello".into()).await;

    assert_eq!(rx1.recv().await.unwrap(), Message::from("hello"));
    assert_eq!(rx2.recv().await.unwrap(), Message::from("hello"));
}

#[tokio::test]
async fn test_multiple_topics_isolation() {
    let bus = MessageBus::new();
    let mut rx1 = bus.subscribe("topic1".to_string()).await;
    let mut rx2 = bus.subscribe("topic2".to_string()).await;

    bus.publish("topic1".to_string(), "msg1".into()).await;
    bus.publish("topic2".to_string(), "msg2".into()).await;

    assert_eq!(rx1.recv().await.unwrap(), Message::from("msg1"));
    assert_eq!(rx2.recv().await.unwrap(), Message::from("msg2"));
}

#[tokio::test]
async fn test_unsubscribe_by_dropping_receiver() {
    let bus = MessageBus::new();
    let rx1 = bus.subscribe("topic1".to_string()).await;
    
    let count1 = bus.publish("topic1".to_string(), "msg1".into()).await;
    assert_eq!(count1, 1);
    
    drop(rx1);
    
    // Allow some time for the drop to be potentially processed, though with broadcast
    // the sender doesn't immediately know about dropped receivers.
    sleep(Duration::from_millis(10)).await;

    // The sender will error because the last receiver was dropped, so count is 0.
    let count2 = bus.publish("topic1".to_string(), "msg2".into()).await;
    assert_eq!(count2, 0);
}

#[tokio::test]
async fn test_cloned_bus_shares_state() {
    let bus1 = MessageBus::new();
    let bus2 = bus1.clone();

    let mut rx = bus1.subscribe("topic1".to_string()).await;
    bus2.publish("topic1".to_string(), "hello".into()).await;

    assert_eq!(rx.recv().await.unwrap(), Message::from("hello"));
}

#[tokio::test]
async fn test_messages_received_in_order() {
    let bus = MessageBus::new();
    let mut rx = bus.subscribe("topic1".to_string()).await;

    for i in 0..10 {
        bus.publish("topic1".to_string(), format!("msg{}", i).into()).await;
    }

    for i in 0..10 {
        assert_eq!(rx.recv().await.unwrap(), Message::from(format!("msg{}", i)));
    }
}
