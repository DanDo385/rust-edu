//! Integration tests for Lab 12: Enums & Pattern Matching

use enums_pattern_matching::solution::*;

#[test]
fn test_process_quit() {
    let msg = Message::Quit;
    let result = process_message(msg);
    assert!(result.to_lowercase().contains("quit"));
}

#[test]
fn test_process_move() {
    let msg = Message::Move { x: 10, y: 20 };
    let result = process_message(msg);
    assert!(result.contains("10"));
    assert!(result.contains("20"));
}

#[test]
fn test_process_write() {
    let msg = Message::Write("hello".to_string());
    let result = process_message(msg);
    assert!(result.contains("hello"));
}

#[test]
fn test_process_color() {
    let msg = Message::ChangeColor(255, 0, 128);
    let result = process_message(msg);
    assert!(result.contains("255"));
    assert!(result.contains("0"));
    assert!(result.contains("128"));
}

#[test]
fn test_variant_size() {
    assert_eq!(variant_size(&Message::Quit), 0);
    assert_eq!(variant_size(&Message::Move { x: 1, y: 2 }), 2);
    assert_eq!(variant_size(&Message::Write("test".to_string())), 1);
    assert_eq!(variant_size(&Message::ChangeColor(1, 2, 3)), 3);
}

#[test]
fn test_is_quit() {
    assert!(is_quit(&Message::Quit));
    assert!(!is_quit(&Message::Move { x: 1, y: 2 }));
    assert!(!is_quit(&Message::Write("test".to_string())));
}
