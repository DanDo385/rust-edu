// Lab 40: Task Scheduler - Integration Tests
//
// These tests verify the Scheduler and TaskInfo types without relying
// on wall-clock time. All tests are deterministic and fast.

use std::time::Duration;
use task_scheduler::solution::{Scheduler, TaskInfo};

// ============================================================================
// TASK INFO CONSTRUCTION
// ============================================================================

#[test]
fn test_task_info_once_properties() {
    let task = TaskInfo::new_once(1, "cleanup", Duration::from_secs(30));
    assert_eq!(task.id(), 1);
    assert_eq!(task.name(), "cleanup");
    assert_eq!(task.delay(), Duration::from_secs(30));
    assert_eq!(task.interval(), None);
    assert!(!task.is_recurring());
    assert_eq!(task.execution_count(), 0);
}

#[test]
fn test_task_info_recurring_properties() {
    let task = TaskInfo::new_recurring(5, "heartbeat", Duration::from_millis(500));
    assert_eq!(task.id(), 5);
    assert_eq!(task.name(), "heartbeat");
    assert_eq!(task.delay(), Duration::from_millis(500));
    assert_eq!(task.interval(), Some(Duration::from_millis(500)));
    assert!(task.is_recurring());
    assert_eq!(task.execution_count(), 0);
}

#[test]
fn test_task_info_mark_executed_increments() {
    let mut task = TaskInfo::new_once(1, "test", Duration::from_secs(1));
    for i in 0..5 {
        assert_eq!(task.execution_count(), i);
        task.mark_executed();
    }
    assert_eq!(task.execution_count(), 5);
}

#[test]
fn test_task_info_clone_independence() {
    let mut task = TaskInfo::new_once(1, "original", Duration::from_secs(1));
    let clone = task.clone();
    task.mark_executed();

    // Clone should not be affected
    assert_eq!(task.execution_count(), 1);
    assert_eq!(clone.execution_count(), 0);
}

// ============================================================================
// SCHEDULER CREATION AND BASIC OPERATIONS
// ============================================================================

#[test]
fn test_scheduler_new_is_empty() {
    let sched = Scheduler::new();
    assert!(sched.is_empty());
    assert_eq!(sched.task_count(), 0);
    assert!(sched.peek_next().is_none());
}

#[test]
fn test_scheduler_default_is_empty() {
    let sched = Scheduler::default();
    assert!(sched.is_empty());
    assert_eq!(sched.task_count(), 0);
}

// ============================================================================
// SCHEDULE ONCE
// ============================================================================

#[test]
fn test_schedule_once_returns_unique_ids() {
    let mut sched = Scheduler::new();
    let id1 = sched.schedule_once("task-a", Duration::from_secs(1));
    let id2 = sched.schedule_once("task-b", Duration::from_secs(2));
    let id3 = sched.schedule_once("task-c", Duration::from_secs(3));

    assert_ne!(id1, id2);
    assert_ne!(id2, id3);
    assert_ne!(id1, id3);
}

#[test]
fn test_schedule_once_increments_count() {
    let mut sched = Scheduler::new();
    assert_eq!(sched.task_count(), 0);

    sched.schedule_once("a", Duration::from_secs(1));
    assert_eq!(sched.task_count(), 1);

    sched.schedule_once("b", Duration::from_secs(2));
    assert_eq!(sched.task_count(), 2);

    sched.schedule_once("c", Duration::from_secs(3));
    assert_eq!(sched.task_count(), 3);
}

#[test]
fn test_schedule_once_not_recurring() {
    let mut sched = Scheduler::new();
    let id = sched.schedule_once("one-shot", Duration::from_secs(5));
    let task = sched.find_task(id).unwrap();
    assert!(!task.is_recurring());
    assert_eq!(task.interval(), None);
}

// ============================================================================
// SCHEDULE RECURRING
// ============================================================================

#[test]
fn test_schedule_recurring_returns_unique_ids() {
    let mut sched = Scheduler::new();
    let id1 = sched.schedule_recurring("rec-a", Duration::from_secs(1));
    let id2 = sched.schedule_recurring("rec-b", Duration::from_secs(2));
    assert_ne!(id1, id2);
}

#[test]
fn test_schedule_recurring_properties() {
    let mut sched = Scheduler::new();
    let id = sched.schedule_recurring("heartbeat", Duration::from_secs(10));
    let task = sched.find_task(id).unwrap();

    assert!(task.is_recurring());
    assert_eq!(task.interval(), Some(Duration::from_secs(10)));
    assert_eq!(task.name(), "heartbeat");
}

#[test]
fn test_mixed_scheduling() {
    let mut sched = Scheduler::new();
    sched.schedule_once("one-shot", Duration::from_secs(5));
    sched.schedule_recurring("periodic", Duration::from_secs(3));
    sched.schedule_once("delayed", Duration::from_secs(10));

    assert_eq!(sched.task_count(), 3);
}

// ============================================================================
// PEEK AND PRIORITY ORDERING
// ============================================================================

#[test]
fn test_peek_returns_shortest_delay() {
    let mut sched = Scheduler::new();
    sched.schedule_once("slow", Duration::from_secs(100));
    sched.schedule_once("fast", Duration::from_secs(1));
    sched.schedule_once("medium", Duration::from_secs(50));

    let next = sched.peek_next().unwrap();
    assert_eq!(next.name(), "fast");
    assert_eq!(next.delay(), Duration::from_secs(1));
}

#[test]
fn test_peek_does_not_remove_task() {
    let mut sched = Scheduler::new();
    sched.schedule_once("task", Duration::from_secs(5));

    let _ = sched.peek_next();
    assert_eq!(sched.task_count(), 1);

    let _ = sched.peek_next();
    assert_eq!(sched.task_count(), 1);
}

#[test]
fn test_peek_on_empty_scheduler_returns_none() {
    let sched = Scheduler::new();
    assert!(sched.peek_next().is_none());
}

// ============================================================================
// EXECUTE NEXT
// ============================================================================

#[test]
fn test_execute_next_removes_one_time_task() {
    let mut sched = Scheduler::new();
    sched.schedule_once("task", Duration::from_secs(5));
    assert_eq!(sched.task_count(), 1);

    let executed = sched.execute_next().unwrap();
    assert_eq!(executed.name(), "task");
    assert_eq!(executed.execution_count(), 1);
    assert_eq!(sched.task_count(), 0);
}

#[test]
fn test_execute_next_keeps_recurring_task() {
    let mut sched = Scheduler::new();
    sched.schedule_recurring("periodic", Duration::from_secs(5));
    assert_eq!(sched.task_count(), 1);

    let executed = sched.execute_next().unwrap();
    assert_eq!(executed.name(), "periodic");
    assert!(executed.is_recurring());
    assert_eq!(executed.execution_count(), 1);

    // Recurring task should be re-enqueued
    assert_eq!(sched.task_count(), 1);
}

#[test]
fn test_execute_next_recurring_multiple_times() {
    let mut sched = Scheduler::new();
    sched.schedule_recurring("repeater", Duration::from_secs(1));

    for _ in 0..5 {
        let executed = sched.execute_next().unwrap();
        assert_eq!(executed.name(), "repeater");
    }

    // Still in the queue after 5 executions
    assert_eq!(sched.task_count(), 1);
    assert_eq!(sched.total_executions(), 5);
}

#[test]
fn test_execute_next_on_empty_returns_none() {
    let mut sched = Scheduler::new();
    assert!(sched.execute_next().is_none());
}

#[test]
fn test_execute_next_priority_order() {
    let mut sched = Scheduler::new();
    sched.schedule_once("slow", Duration::from_secs(100));
    sched.schedule_once("fast", Duration::from_secs(1));
    sched.schedule_once("medium", Duration::from_secs(50));

    let first = sched.execute_next().unwrap();
    assert_eq!(first.name(), "fast");

    let second = sched.execute_next().unwrap();
    assert_eq!(second.name(), "medium");

    let third = sched.execute_next().unwrap();
    assert_eq!(third.name(), "slow");

    assert!(sched.is_empty());
}

// ============================================================================
// EXECUTED TASKS LOG
// ============================================================================

#[test]
fn test_executed_tasks_initially_empty() {
    let sched = Scheduler::new();
    assert!(sched.executed_tasks().is_empty());
    assert_eq!(sched.total_executions(), 0);
}

#[test]
fn test_executed_tasks_records_executions() {
    let mut sched = Scheduler::new();
    sched.schedule_once("first", Duration::from_secs(1));
    sched.schedule_once("second", Duration::from_secs(2));

    sched.execute_next();
    assert_eq!(sched.total_executions(), 1);
    assert_eq!(sched.executed_tasks()[0].name(), "first");

    sched.execute_next();
    assert_eq!(sched.total_executions(), 2);
    assert_eq!(sched.executed_tasks()[1].name(), "second");
}

#[test]
fn test_executed_tasks_records_recurring_executions() {
    let mut sched = Scheduler::new();
    sched.schedule_recurring("repeat", Duration::from_secs(1));

    sched.execute_next();
    sched.execute_next();
    sched.execute_next();

    assert_eq!(sched.total_executions(), 3);
    for task in sched.executed_tasks() {
        assert_eq!(task.name(), "repeat");
    }
}

// ============================================================================
// FIND TASK
// ============================================================================

#[test]
fn test_find_task_by_id() {
    let mut sched = Scheduler::new();
    let id1 = sched.schedule_once("alpha", Duration::from_secs(1));
    let id2 = sched.schedule_once("beta", Duration::from_secs(2));

    let task1 = sched.find_task(id1).unwrap();
    assert_eq!(task1.name(), "alpha");

    let task2 = sched.find_task(id2).unwrap();
    assert_eq!(task2.name(), "beta");
}

#[test]
fn test_find_task_nonexistent_returns_none() {
    let sched = Scheduler::new();
    assert!(sched.find_task(999).is_none());
}

#[test]
fn test_find_task_after_execution_one_time_gone() {
    let mut sched = Scheduler::new();
    let id = sched.schedule_once("ephemeral", Duration::from_secs(1));

    assert!(sched.find_task(id).is_some());
    sched.execute_next();
    assert!(sched.find_task(id).is_none());
}

#[test]
fn test_find_task_after_execution_recurring_still_present() {
    let mut sched = Scheduler::new();
    let id = sched.schedule_recurring("persistent", Duration::from_secs(1));

    assert!(sched.find_task(id).is_some());
    sched.execute_next();
    assert!(sched.find_task(id).is_some());
}

// ============================================================================
// TASK NAMES
// ============================================================================

#[test]
fn test_task_names_lists_all() {
    let mut sched = Scheduler::new();
    sched.schedule_once("alpha", Duration::from_secs(1));
    sched.schedule_once("beta", Duration::from_secs(2));
    sched.schedule_recurring("gamma", Duration::from_secs(3));

    let mut names = sched.task_names();
    names.sort();
    assert_eq!(names, vec!["alpha", "beta", "gamma"]);
}

#[test]
fn test_task_names_empty() {
    let sched = Scheduler::new();
    assert!(sched.task_names().is_empty());
}

// ============================================================================
// CLEAR
// ============================================================================

#[test]
fn test_clear_removes_all_tasks() {
    let mut sched = Scheduler::new();
    sched.schedule_once("a", Duration::from_secs(1));
    sched.schedule_once("b", Duration::from_secs(2));
    sched.schedule_recurring("c", Duration::from_secs(3));

    assert_eq!(sched.task_count(), 3);
    sched.clear();
    assert_eq!(sched.task_count(), 0);
    assert!(sched.is_empty());
}

#[test]
fn test_clear_then_reschedule() {
    let mut sched = Scheduler::new();
    sched.schedule_once("old", Duration::from_secs(1));
    sched.clear();

    let id = sched.schedule_once("new", Duration::from_secs(5));
    assert_eq!(sched.task_count(), 1);
    let task = sched.find_task(id).unwrap();
    assert_eq!(task.name(), "new");
}

// ============================================================================
// COMPLEX SCENARIOS
// ============================================================================

#[test]
fn test_mixed_one_time_and_recurring_execution_order() {
    let mut sched = Scheduler::new();

    // One-time at 1s, recurring every 5s, one-time at 3s
    // Priority order by delay: early(1s) < late(3s) < repeat(5s)
    sched.schedule_once("early", Duration::from_secs(1));
    sched.schedule_recurring("repeat", Duration::from_secs(5));
    sched.schedule_once("late", Duration::from_secs(3));

    // First: early (1s delay)
    let first = sched.execute_next().unwrap();
    assert_eq!(first.name(), "early");

    // Second: late (3s delay) - shorter than repeat's 5s
    let second = sched.execute_next().unwrap();
    assert_eq!(second.name(), "late");

    // Third: repeat (5s delay)
    let third = sched.execute_next().unwrap();
    assert_eq!(third.name(), "repeat");

    // Repeat was re-enqueued (still 5s delay), only recurring task remains
    assert_eq!(sched.task_count(), 1);
    let fourth = sched.execute_next().unwrap();
    assert_eq!(fourth.name(), "repeat");
}

#[test]
fn test_many_tasks_priority_ordering() {
    let mut sched = Scheduler::new();

    // Schedule tasks with delays 10, 9, 8, ..., 1
    for i in (1..=10).rev() {
        sched.schedule_once(
            &format!("task-{}", i),
            Duration::from_secs(i),
        );
    }

    // Should execute in order 1, 2, 3, ..., 10
    for expected in 1..=10u64 {
        let task = sched.execute_next().unwrap();
        assert_eq!(task.name(), format!("task-{}", expected));
    }
}

#[test]
fn test_scheduler_stress_only_one_time_tasks() {
    let mut sched = Scheduler::new();

    // Schedule 100 one-time tasks with increasing delays
    for i in 0..100 {
        sched.schedule_once(&format!("task-{}", i), Duration::from_millis(i as u64 + 1));
    }

    assert_eq!(sched.task_count(), 100);

    // Execute all 100
    for _ in 0..100 {
        let task = sched.execute_next().unwrap();
        assert!(!task.is_recurring());
    }

    // All one-time tasks should be gone
    assert_eq!(sched.task_count(), 0);
    assert!(sched.is_empty());
    assert_eq!(sched.total_executions(), 100);
}

#[test]
fn test_scheduler_stress_recurring_tasks_persist() {
    let mut sched = Scheduler::new();

    // Schedule 10 recurring tasks
    for i in 0..10 {
        sched.schedule_recurring(&format!("rec-{}", i), Duration::from_millis(i as u64 + 1));
    }

    assert_eq!(sched.task_count(), 10);

    // Execute 50 times (each recurring task re-enqueues)
    for _ in 0..50 {
        let task = sched.execute_next().unwrap();
        assert!(task.is_recurring());
    }

    // All 10 recurring tasks should still be in the queue
    assert_eq!(sched.task_count(), 10);
    assert_eq!(sched.total_executions(), 50);
}
