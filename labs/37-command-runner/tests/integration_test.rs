// Integration tests for Lab 37: Command Runner
//
// Tests command execution, output capture, environment variables,
// pipeline piping, the builder pattern, and the task runner.
//
// Note: These tests use platform commands (echo, true, false, sh, pwd)
// that are available on Unix/macOS. They may need adjustment on Windows.

use command_runner::{CommandBuilder, CommandRunner, Task, TaskRunner};
use std::time::Duration;

// ============================================================================
// BASIC COMMAND EXECUTION
// ============================================================================

#[test]
fn test_run_echo() {
    let result = CommandRunner::run("echo", &["hello world"]).unwrap();
    assert!(result.success);
    assert_eq!(result.stdout.trim(), "hello world");
    assert_eq!(result.exit_code, Some(0));
}

#[test]
fn test_run_echo_multiple_args() {
    let result = CommandRunner::run("echo", &["hello", "world"]).unwrap();
    assert!(result.success);
    assert_eq!(result.stdout.trim(), "hello world");
}

#[test]
fn test_run_echo_empty_string() {
    let result = CommandRunner::run("echo", &[""]).unwrap();
    assert!(result.success);
    // echo with empty string still produces a newline
    assert_eq!(result.stdout.trim(), "");
}

#[test]
fn test_run_true_command() {
    let result = CommandRunner::run("true", &[]).unwrap();
    assert!(result.success);
    assert_eq!(result.exit_code, Some(0));
}

#[test]
fn test_run_false_command() {
    let result = CommandRunner::run("false", &[]).unwrap();
    assert!(!result.success);
    // false exits with 1 on Unix
    assert_ne!(result.exit_code, Some(0));
}

#[test]
fn test_run_nonexistent_command() {
    let result = CommandRunner::run("nonexistent_command_xyz_12345", &[]);
    assert!(result.is_err());
}

// ============================================================================
// CAPTURE OUTPUT
// ============================================================================

#[test]
fn test_capture_stdout() {
    let result = CommandRunner::run("echo", &["captured output"]).unwrap();
    assert!(result.stdout.contains("captured output"));
}

#[test]
fn test_capture_stderr() {
    // Use sh to write to stderr
    let result = CommandRunner::run("sh", &["-c", "echo error >&2"]).unwrap();
    assert!(result.stderr.contains("error"));
}

#[test]
fn test_capture_exit_code_success() {
    let result = CommandRunner::run("sh", &["-c", "exit 0"]).unwrap();
    assert_eq!(result.exit_code, Some(0));
    assert!(result.success);
}

#[test]
fn test_capture_exit_code_failure() {
    let result = CommandRunner::run("sh", &["-c", "exit 42"]).unwrap();
    assert_eq!(result.exit_code, Some(42));
    assert!(!result.success);
}

// ============================================================================
// WORKING DIRECTORY
// ============================================================================

#[test]
fn test_run_in_dir() {
    let result = CommandRunner::run_in_dir("pwd", &[], "/tmp").unwrap();
    assert!(result.success);
    // On macOS, /tmp may resolve to /private/tmp
    let output = result.stdout.trim();
    assert!(output == "/tmp" || output == "/private/tmp");
}

#[test]
fn test_run_in_dir_nonexistent() {
    let result = CommandRunner::run_in_dir("pwd", &[], "/nonexistent_dir_xyz");
    assert!(result.is_err());
}

// ============================================================================
// ENVIRONMENT VARIABLES
// ============================================================================

#[test]
fn test_run_with_env() {
    let result = CommandRunner::run_with_env(
        "sh",
        &["-c", "echo $MY_TEST_VAR"],
        &[("MY_TEST_VAR", "hello_from_env")],
    )
    .unwrap();
    assert!(result.success);
    assert_eq!(result.stdout.trim(), "hello_from_env");
}

#[test]
fn test_run_with_multiple_env_vars() {
    let result = CommandRunner::run_with_env(
        "sh",
        &["-c", "echo ${VAR_A}_${VAR_B}"],
        &[("VAR_A", "foo"), ("VAR_B", "bar")],
    )
    .unwrap();
    assert!(result.success);
    assert_eq!(result.stdout.trim(), "foo_bar");
}

// ============================================================================
// PIPELINE (PIPING)
// ============================================================================

#[test]
fn test_pipe_echo_to_wc() {
    // echo "hello world" | wc -w
    let result = CommandRunner::pipe("echo", &["hello world"], "wc", &["-w"]).unwrap();
    assert!(result.success);
    assert_eq!(result.stdout.trim(), "2");
}

#[test]
fn test_pipe_echo_to_cat() {
    // echo "piped data" | cat
    let result = CommandRunner::pipe("echo", &["piped data"], "cat", &[]).unwrap();
    assert!(result.success);
    assert_eq!(result.stdout.trim(), "piped data");
}

#[test]
fn test_pipe_multiline() {
    // echo "line1\nline2\nline3" | wc -l
    let result =
        CommandRunner::pipe("printf", &["line1\nline2\nline3\n"], "wc", &["-l"]).unwrap();
    assert!(result.success);
    assert_eq!(result.stdout.trim(), "3");
}

// ============================================================================
// TIMEOUT
// ============================================================================

#[test]
fn test_timeout_completes_in_time() {
    // echo should complete well within 5 seconds
    let result =
        CommandRunner::run_with_timeout("echo", &["fast"], Duration::from_secs(5)).unwrap();
    assert!(result.success);
    assert_eq!(result.stdout.trim(), "fast");
}

#[test]
fn test_timeout_expires() {
    // sleep 10 with 1 second timeout should fail
    let result = CommandRunner::run_with_timeout("sleep", &["10"], Duration::from_secs(1));
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("timed out"));
}

// ============================================================================
// COMMAND BUILDER
// ============================================================================

#[test]
fn test_builder_basic() {
    let result = CommandBuilder::new("echo")
        .arg("builder test")
        .run()
        .unwrap();
    assert!(result.success);
    assert_eq!(result.stdout.trim(), "builder test");
}

#[test]
fn test_builder_multiple_args() {
    let result = CommandBuilder::new("echo")
        .args(&["one", "two", "three"])
        .run()
        .unwrap();
    assert!(result.success);
    assert_eq!(result.stdout.trim(), "one two three");
}

#[test]
fn test_builder_with_env() {
    let result = CommandBuilder::new("sh")
        .arg("-c")
        .arg("echo $BUILD_VAR")
        .env("BUILD_VAR", "from_builder")
        .run()
        .unwrap();
    assert!(result.success);
    assert_eq!(result.stdout.trim(), "from_builder");
}

#[test]
fn test_builder_with_working_dir() {
    let result = CommandBuilder::new("pwd")
        .working_dir("/tmp")
        .run()
        .unwrap();
    assert!(result.success);
    let output = result.stdout.trim();
    assert!(output == "/tmp" || output == "/private/tmp");
}

#[test]
fn test_builder_with_timeout_succeeds() {
    let result = CommandBuilder::new("echo")
        .arg("quick")
        .timeout(Duration::from_secs(5))
        .run()
        .unwrap();
    assert!(result.success);
}

#[test]
fn test_builder_nonexistent_command() {
    let result = CommandBuilder::new("nonexistent_command_xyz_99999").run();
    assert!(result.is_err());
}

#[test]
fn test_builder_chaining() {
    // Verify the builder can chain all methods fluently
    let result = CommandBuilder::new("echo")
        .arg("chain")
        .env("KEY", "val")
        .timeout(Duration::from_secs(10))
        .run()
        .unwrap();
    assert!(result.success);
    assert_eq!(result.stdout.trim(), "chain");
}

// ============================================================================
// TASK RUNNER
// ============================================================================

#[test]
fn test_task_runner_empty() {
    let runner = TaskRunner::new();
    assert_eq!(runner.task_count(), 0);
    let results = runner.run_all();
    assert!(results.is_empty());
}

#[test]
fn test_task_runner_single_task() {
    let mut runner = TaskRunner::new();
    runner.add_task(Task {
        name: "Greet".to_string(),
        command: "echo".to_string(),
        args: vec!["hello".to_string()],
    });
    assert_eq!(runner.task_count(), 1);

    let results = runner.run_all();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name, "Greet");
    assert!(results[0].result.is_ok());
    let cmd_result = results[0].result.as_ref().unwrap();
    assert!(cmd_result.success);
    assert_eq!(cmd_result.stdout.trim(), "hello");
}

#[test]
fn test_task_runner_multiple_tasks() {
    let mut runner = TaskRunner::new();
    runner.add_task(Task {
        name: "Step 1".to_string(),
        command: "echo".to_string(),
        args: vec!["first".to_string()],
    });
    runner.add_task(Task {
        name: "Step 2".to_string(),
        command: "echo".to_string(),
        args: vec!["second".to_string()],
    });
    runner.add_task(Task {
        name: "Step 3".to_string(),
        command: "echo".to_string(),
        args: vec!["third".to_string()],
    });

    let results = runner.run_all();
    assert_eq!(results.len(), 3);

    for (i, expected) in ["first", "second", "third"].iter().enumerate() {
        let cmd_result = results[i].result.as_ref().unwrap();
        assert!(cmd_result.success);
        assert_eq!(cmd_result.stdout.trim(), *expected);
    }
}

#[test]
fn test_task_runner_failing_task() {
    let mut runner = TaskRunner::new();
    runner.add_task(Task {
        name: "Fail".to_string(),
        command: "false".to_string(),
        args: vec![],
    });

    let results = runner.run_all();
    assert_eq!(results.len(), 1);
    let cmd_result = results[0].result.as_ref().unwrap();
    assert!(!cmd_result.success);
}

#[test]
fn test_task_runner_records_duration() {
    let mut runner = TaskRunner::new();
    runner.add_task(Task {
        name: "Quick".to_string(),
        command: "echo".to_string(),
        args: vec!["fast".to_string()],
    });

    let results = runner.run_all();
    // Duration should be non-negative and reasonably small
    assert!(results[0].duration < Duration::from_secs(5));
}

#[test]
fn test_task_runner_nonexistent_command() {
    let mut runner = TaskRunner::new();
    runner.add_task(Task {
        name: "Bad".to_string(),
        command: "nonexistent_cmd_xyz_88888".to_string(),
        args: vec![],
    });

    let results = runner.run_all();
    assert_eq!(results.len(), 1);
    assert!(results[0].result.is_err());
}

// ============================================================================
// COMMAND RESULT STRUCT TESTS
// ============================================================================

#[test]
fn test_command_result_clone() {
    let result = CommandRunner::run("echo", &["clone me"]).unwrap();
    let cloned = result.clone();
    assert_eq!(result.stdout, cloned.stdout);
    assert_eq!(result.stderr, cloned.stderr);
    assert_eq!(result.exit_code, cloned.exit_code);
    assert_eq!(result.success, cloned.success);
}

#[test]
fn test_command_result_debug() {
    let result = CommandRunner::run("echo", &["debug"]).unwrap();
    let debug_str = format!("{:?}", result);
    assert!(debug_str.contains("CommandResult"));
}
