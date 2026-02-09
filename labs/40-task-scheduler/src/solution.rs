// Lab 40: Task Scheduler
//
// This module implements a simplified, testable task scheduler.
// Unlike the main.rs version which uses Instant (hard to test),
// this version uses Duration-based delays and explicit state tracking
// so that all behavior can be verified in unit and integration tests.
//
// Key Concepts:
// - Priority queues (BinaryHeap) with custom ordering
// - Structs with rich metadata for scheduling
// - The Ord/PartialOrd trait pair for custom comparison
// - Builder-like patterns for task configuration
// - Separation of scheduling logic from time-dependent execution

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::time::Duration;

// ============================================================================
// TASK INFO
// ============================================================================

/// Represents a scheduled task with metadata suitable for testing.
///
/// Instead of using Instant (which depends on wall-clock time and is
/// difficult to control in tests), TaskInfo stores a Duration-based
/// delay and an interval for recurring tasks.
///
/// # Memory Model
/// TaskInfo is stored on the heap inside BinaryHeap's backing Vec.
/// Fields:
///   - id (u64): 8 bytes on stack
///   - name (String): 24 bytes on stack + heap-allocated chars
///   - delay (Duration): 12 bytes (u64 secs + u32 nanos)
///   - interval (Option<Duration>): 12 + tag bytes
///   - is_recurring (bool): 1 byte + padding
///   - execution_count (usize): 8 bytes
#[derive(Debug, Clone)]
pub struct TaskInfo {
    id: u64,
    name: String,
    delay: Duration,
    interval: Option<Duration>,
    is_recurring: bool,
    execution_count: usize,
}

impl TaskInfo {
    /// Create a new one-time task.
    pub fn new_once(id: u64, name: &str, delay: Duration) -> Self {
        TaskInfo {
            id,
            name: name.to_string(),
            delay,
            interval: None,
            is_recurring: false,
            execution_count: 0,
        }
    }

    /// Create a new recurring task.
    pub fn new_recurring(id: u64, name: &str, interval: Duration) -> Self {
        TaskInfo {
            id,
            name: name.to_string(),
            delay: interval,
            interval: Some(interval),
            is_recurring: true,
            execution_count: 0,
        }
    }

    /// Get the task's unique ID.
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Get the task's name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the task's delay (time until first execution, or interval for recurring).
    pub fn delay(&self) -> Duration {
        self.delay
    }

    /// Get the task's interval if it is recurring, or None for one-time tasks.
    pub fn interval(&self) -> Option<Duration> {
        self.interval
    }

    /// Check whether the task is recurring.
    pub fn is_recurring(&self) -> bool {
        self.is_recurring
    }

    /// Get how many times this task has been executed.
    pub fn execution_count(&self) -> usize {
        self.execution_count
    }

    /// Mark this task as executed (increments execution count).
    pub fn mark_executed(&mut self) {
        self.execution_count += 1;
    }
}

// ============================================================================
// ORDERING FOR PRIORITY QUEUE
// ============================================================================
// BinaryHeap is a max-heap by default. We reverse ordering so that
// tasks with the shortest delay have the highest priority (min-heap behavior).

impl Ord for TaskInfo {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse: shorter delay = higher priority
        other.delay.cmp(&self.delay)
            .then_with(|| self.id.cmp(&other.id)) // tie-break by ID
    }
}

impl PartialOrd for TaskInfo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for TaskInfo {}

impl PartialEq for TaskInfo {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

// ============================================================================
// SCHEDULER
// ============================================================================

/// A task scheduler that manages a priority queue of TaskInfo items.
///
/// The scheduler provides methods to add one-time and recurring tasks,
/// query the task queue, and simulate execution (marking tasks as executed).
///
/// # Memory Model
/// The BinaryHeap stores TaskInfo values in a contiguous Vec on the heap.
/// The Scheduler struct itself is 24 bytes on the stack (heap pointer, len, cap)
/// plus 8 bytes for next_id.
pub struct Scheduler {
    tasks: BinaryHeap<TaskInfo>,
    next_id: u64,
    executed: Vec<TaskInfo>,
}

impl Scheduler {
    /// Create a new empty scheduler.
    pub fn new() -> Self {
        Scheduler {
            tasks: BinaryHeap::new(),
            next_id: 1,
            executed: Vec::new(),
        }
    }

    /// Schedule a one-time task that runs after the given delay.
    /// Returns the assigned task ID.
    pub fn schedule_once(&mut self, name: &str, delay: Duration) -> u64 {
        let id = self.next_id;
        let task = TaskInfo::new_once(id, name, delay);
        self.tasks.push(task);
        self.next_id += 1;
        id
    }

    /// Schedule a recurring task that runs at the given interval.
    /// Returns the assigned task ID.
    pub fn schedule_recurring(&mut self, name: &str, interval: Duration) -> u64 {
        let id = self.next_id;
        let task = TaskInfo::new_recurring(id, name, interval);
        self.tasks.push(task);
        self.next_id += 1;
        id
    }

    /// Get the number of tasks currently in the queue.
    pub fn task_count(&self) -> usize {
        self.tasks.len()
    }

    /// Check if the scheduler has no tasks.
    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    /// Peek at the next task (highest priority / shortest delay) without removing it.
    pub fn peek_next(&self) -> Option<&TaskInfo> {
        self.tasks.peek()
    }

    /// Execute the next task: removes it from the queue, marks it as executed,
    /// and if it is recurring, re-enqueues it. Returns the executed task info (clone).
    pub fn execute_next(&mut self) -> Option<TaskInfo> {
        if let Some(mut task) = self.tasks.pop() {
            task.mark_executed();

            if task.is_recurring() {
                // Clone for the executed log, then re-enqueue the original
                let executed_copy = task.clone();
                self.tasks.push(task);
                self.executed.push(executed_copy.clone());
                Some(executed_copy)
            } else {
                self.executed.push(task.clone());
                Some(task)
            }
        } else {
            None
        }
    }

    /// Get a list of all tasks that have been executed (in execution order).
    pub fn executed_tasks(&self) -> &[TaskInfo] {
        &self.executed
    }

    /// Get the total number of task executions that have occurred.
    pub fn total_executions(&self) -> usize {
        self.executed.len()
    }

    /// Remove all tasks from the scheduler.
    pub fn clear(&mut self) {
        self.tasks.clear();
    }

    /// Get all task names currently in the queue (unordered).
    pub fn task_names(&self) -> Vec<String> {
        self.tasks.iter().map(|t| t.name().to_string()).collect()
    }

    /// Find a task by ID in the queue. Returns None if not found.
    pub fn find_task(&self, id: u64) -> Option<&TaskInfo> {
        self.tasks.iter().find(|t| t.id() == id)
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// WHAT RUST DOES UNDER THE HOOD
// ============================================================================
// 1. BINARYHEAP
//    BinaryHeap is backed by a Vec, maintaining the heap property on push/pop.
//    Peek is O(1), push and pop are O(log n).
//    We reverse Ord to get min-heap behavior (shortest delay first).
//
// 2. CUSTOM ORD
//    Implementing Ord + PartialOrd + Eq + PartialEq is required for BinaryHeap.
//    The compiler enforces consistency between these trait implementations.
//
// 3. DURATION
//    Duration stores time as u64 seconds + u32 nanoseconds.
//    It implements Ord, so it can be compared directly.
//    All arithmetic is checked in debug mode.
//
// 4. OPTION<DURATION>
//    Used to distinguish one-time tasks (None) from recurring tasks (Some).
//    Option<Duration> is 16 bytes (12 for Duration + 4 for discriminant + padding).
//
// 5. CLONE SEMANTICS
//    TaskInfo derives Clone so we can keep copies in the executed log.
//    String clone allocates new heap memory for the name.
//    Duration and other primitives are Copy (bitwise copy, no allocation).

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_info_one_time() {
        let task = TaskInfo::new_once(1, "backup", Duration::from_secs(10));
        assert_eq!(task.id(), 1);
        assert_eq!(task.name(), "backup");
        assert!(!task.is_recurring());
        assert_eq!(task.interval(), None);
        assert_eq!(task.execution_count(), 0);
    }

    #[test]
    fn test_task_info_recurring() {
        let task = TaskInfo::new_recurring(2, "heartbeat", Duration::from_secs(5));
        assert_eq!(task.id(), 2);
        assert!(task.is_recurring());
        assert_eq!(task.interval(), Some(Duration::from_secs(5)));
    }

    #[test]
    fn test_mark_executed() {
        let mut task = TaskInfo::new_once(1, "test", Duration::from_secs(1));
        assert_eq!(task.execution_count(), 0);
        task.mark_executed();
        assert_eq!(task.execution_count(), 1);
        task.mark_executed();
        assert_eq!(task.execution_count(), 2);
    }

    #[test]
    fn test_scheduler_default() {
        let sched = Scheduler::default();
        assert_eq!(sched.task_count(), 0);
        assert!(sched.is_empty());
    }
}
