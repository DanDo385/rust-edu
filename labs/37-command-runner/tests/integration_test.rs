//! Integration tests for Lab 37: Command Runner

use command_runner::solution::{CommandBuilder, CommandError};
use std::time::Duration;
use tempfile::tempdir;

#[test]
fn test_run_echo() {
    let result = CommandBuilder::new("echo")
        .arg("hello")
        .run()
        .unwrap();
    assert_eq!(result.exit_code, 0);
    assert_eq!(result.stdout.trim(), "hello");
    assert_eq!(result.stderr, "");
}

#[test]
fn test_run_command_that_fails() {
    // `ls` on a nonexistent file will fail
    let result = CommandBuilder::new("ls")
        .arg("nonexistent_dir_12345")
        .run()
        .unwrap();
    assert_ne!(result.exit_code, 0);
    assert!(!result.stderr.is_empty());
}

#[test]
fn test_run_command_in_different_dir() {
    let temp_dir = tempdir().unwrap();
    let file_path = temp_dir.path().join("test_file.txt");
    std::fs::write(&file_path, "hello").unwrap();

    // Run `ls` inside the temporary directory
    let result = CommandBuilder::new("ls")
        .current_dir(temp_dir.path().to_str().unwrap())
        .run()
        .unwrap();

    assert_eq!(result.exit_code, 0);
    assert!(result.stdout.contains("test_file.txt"));
}

#[test]
fn test_command_with_env_var() {
    let result = CommandBuilder::new("sh")
        .arg("-c")
        .arg("echo $MY_TEST_VAR")
        .env("MY_TEST_VAR", "hello from env")
        .run()
        .unwrap();

    assert_eq!(result.exit_code, 0);
    assert_eq!(result.stdout.trim(), "hello from env");
}

#[test]
fn test_command_timeout_succeeds() {
    let result = CommandBuilder::new("sleep")
        .arg("0.1")
        .timeout(Duration::from_secs(1))
        .run();
    assert!(result.is_ok());
}

#[test]
fn test_command_timeout_fails() {
    let result = CommandBuilder::new("sleep")
        .arg("2")
        .timeout(Duration::from_millis(100))
        .run();

    match result {
        Err(CommandError::Timeout(_)) => {
            // This is the expected outcome
        }
        _ => panic!("Expected a timeout error"),
    }
}

#[test]
fn test_nonexistent_command() {
    let result = CommandBuilder::new("a_truly_nonexistent_command_123").run();
    assert!(matches!(result, Err(CommandError::Io(_))));
}