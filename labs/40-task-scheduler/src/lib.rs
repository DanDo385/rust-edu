//! # A Task Scheduler - Your Implementation
//!
//! Build a scheduler that keeps tasks ordered by their next execution time.
//! Use a priority queue (`BinaryHeap`) and custom ordering to make "earliest"
//! tasks come out first.

use std::time::Duration;

/// Metadata for a scheduled task.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskInfo {
    _private: (),
}

impl TaskInfo {
    pub fn new_once(_id: u64, _name: &str, _delay: Duration) -> Self {
        todo!("Construct a one-time task")
    }

    pub fn new_recurring(_id: u64, _name: &str, _interval: Duration) -> Self {
        todo!("Construct a recurring task")
    }

    pub fn id(&self) -> u64 {
        todo!("Return task ID")
    }

    pub fn name(&self) -> &str {
        todo!("Return task name")
    }

    pub fn delay(&self) -> Duration {
        todo!("Return task delay")
    }

    pub fn interval(&self) -> Option<Duration> {
        todo!("Return interval for recurring tasks")
    }

    pub fn is_recurring(&self) -> bool {
        todo!("Return whether task recurs")
    }

    pub fn execution_count(&self) -> usize {
        todo!("Return execution count")
    }

    pub fn mark_executed(&mut self) {
        todo!("Increment execution count")
    }
}

/// Priority-queue-backed task scheduler.
pub struct Scheduler {
    _private: (),
}

impl Scheduler {
    pub fn new() -> Self {
        todo!("Initialize an empty scheduler")
    }

    pub fn schedule_once(&mut self, _name: &str, _delay: Duration) -> u64 {
        todo!("Schedule a one-time task")
    }

    pub fn schedule_recurring(&mut self, _name: &str, _interval: Duration) -> u64 {
        todo!("Schedule a recurring task")
    }

    pub fn task_count(&self) -> usize {
        todo!("Return task count")
    }

    pub fn is_empty(&self) -> bool {
        todo!("Return whether queue is empty")
    }

    pub fn peek_next(&self) -> Option<&TaskInfo> {
        todo!("Peek earliest task")
    }

    pub fn execute_next(&mut self) -> Option<TaskInfo> {
        todo!("Execute next task")
    }

    pub fn executed_tasks(&self) -> &[TaskInfo] {
        todo!("Return execution log")
    }

    pub fn total_executions(&self) -> usize {
        todo!("Return number of task executions")
    }

    pub fn clear(&mut self) {
        todo!("Clear scheduled tasks")
    }

    pub fn task_names(&self) -> Vec<String> {
        todo!("Return names of queued tasks")
    }

    pub fn find_task(&self, _id: u64) -> Option<&TaskInfo> {
        todo!("Find task by ID")
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}

#[doc(hidden)]
pub mod solution;
