//! # A Task Scheduler - Demo

use std::time::Duration;
use task_scheduler::solution::Scheduler;

fn main() {
    println!("=== Task Scheduler Demo ===");

    let mut scheduler = Scheduler::new();

    scheduler.schedule_once("Task A (one-time)", Duration::from_secs(3));
    scheduler.schedule_recurring("Task B (recurring)", Duration::from_secs(2));
    scheduler.schedule_once("Task C (one-time)", Duration::from_secs(1));

    println!("scheduled {} tasks", scheduler.task_count());

    for _ in 0..5 {
        if let Some(task) = scheduler.execute_next() {
            println!("executed: {} (id={})", task.name(), task.id());
        }
    }

    println!("total executions: {}", scheduler.total_executions());
}
