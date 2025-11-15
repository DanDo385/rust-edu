// Project 37: Task Scheduler
//
// This program implements a task scheduler similar to cron.
// It demonstrates time handling, scheduling algorithms, closures,
// and priority queues for managing delayed and recurring tasks.

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::time::{Duration, Instant};
use std::thread;

fn main() {
    println!("=== Task Scheduler ===\n");

    // ============================================================================
    // WHAT IS A TASK SCHEDULER?
    // ============================================================================
    // A task scheduler runs functions at specified times or intervals.
    // It's like cron on Linux or Task Scheduler on Windows.
    //
    // Types of tasks:
    // 1. One-time: Run once after delay
    // 2. Recurring: Run every N seconds/minutes/hours
    // 3. Scheduled: Run at specific times (cron-style)

    let mut scheduler = Scheduler::new();

    println!("=== Scheduling Tasks ===\n");

    // Schedule a one-time task
    scheduler.schedule_once(
        Duration::from_secs(2),
        Box::new(|| println!("  [TASK] One-time task executed!")),
        "One-time task (2 seconds)".to_string(),
    );

    // Schedule recurring tasks
    scheduler.schedule_recurring(
        Duration::from_secs(3),
        Box::new(|| println!("  [TASK] Recurring task A - Hello every 3 seconds!")),
        "Recurring task A (every 3 seconds)".to_string(),
    );

    scheduler.schedule_recurring(
        Duration::from_secs(5),
        Box::new(|| println!("  [TASK] Recurring task B - Running every 5 seconds!")),
        "Recurring task B (every 5 seconds)".to_string(),
    );

    scheduler.schedule_once(
        Duration::from_secs(4),
        Box::new(|| {
            println!("  [TASK] Simulating database backup...");
            thread::sleep(Duration::from_millis(500));
            println!("  [TASK] Database backup complete!");
        }),
        "Database backup (4 seconds)".to_string(),
    );

    scheduler.schedule_once(
        Duration::from_secs(7),
        Box::new(|| println!("  [TASK] Sending scheduled email notification")),
        "Email notification (7 seconds)".to_string(),
    );

    println!("Scheduled {} tasks\n", scheduler.task_count());
    println!("Starting scheduler (will run for 15 seconds)...\n");

    // ============================================================================
    // RUN THE SCHEDULER
    // ============================================================================
    // The scheduler will run for 15 seconds, executing tasks as they become due

    let start = Instant::now();
    let run_duration = Duration::from_secs(15);

    while start.elapsed() < run_duration {
        scheduler.tick();
        thread::sleep(Duration::from_millis(100)); // Check every 100ms
    }

    println!("\n=== Scheduler Finished ===");
    println!("Total runtime: {:.2}s", start.elapsed().as_secs_f64());
    println!("Remaining scheduled tasks: {}", scheduler.task_count());

    println!();
}

// ============================================================================
// TASK STRUCTURE
// ============================================================================

/// A scheduled task with execution time and callback
struct Task {
    id: u64,
    name: String,
    execute_at: Instant,
    interval: Option<Duration>, // Some for recurring, None for one-time
    callback: Box<dyn Fn()>,
}

impl Task {
    fn new(
        id: u64,
        name: String,
        execute_at: Instant,
        interval: Option<Duration>,
        callback: Box<dyn Fn()>,
    ) -> Self {
        Task {
            id,
            name,
            execute_at,
            interval,
            callback,
        }
    }

    /// Execute this task
    fn execute(&self) {
        (self.callback)();
    }

    /// Check if this task is due
    fn is_due(&self) -> bool {
        Instant::now() >= self.execute_at
    }

    /// Reschedule this task if it's recurring
    fn reschedule(&mut self) {
        if let Some(interval) = self.interval {
            self.execute_at = Instant::now() + interval;
        }
    }
}

// ============================================================================
// PRIORITY QUEUE ORDERING
// ============================================================================
// BinaryHeap is a MAX heap by default, but we want a MIN heap
// (earliest time first). So we reverse the ordering.

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering: earlier times have higher priority
        other.execute_at.cmp(&self.execute_at)
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Task {}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.execute_at == other.execute_at
    }
}

// ============================================================================
// SCHEDULER STRUCTURE
// ============================================================================

struct Scheduler {
    tasks: BinaryHeap<Task>,
    next_id: u64,
}

impl Scheduler {
    fn new() -> Self {
        Scheduler {
            tasks: BinaryHeap::new(),
            next_id: 1,
        }
    }

    /// Schedule a one-time task
    fn schedule_once(&mut self, delay: Duration, callback: Box<dyn Fn()>, name: String) {
        let execute_at = Instant::now() + delay;
        let task = Task::new(self.next_id, name.clone(), execute_at, None, callback);

        println!(
            "Scheduled: {} (ID: {}) - runs in {:.1}s",
            name,
            self.next_id,
            delay.as_secs_f64()
        );

        self.tasks.push(task);
        self.next_id += 1;
    }

    /// Schedule a recurring task
    fn schedule_recurring(&mut self, interval: Duration, callback: Box<dyn Fn()>, name: String) {
        let execute_at = Instant::now() + interval;
        let task = Task::new(
            self.next_id,
            name.clone(),
            execute_at,
            Some(interval),
            callback,
        );

        println!(
            "Scheduled: {} (ID: {}) - runs every {:.1}s",
            name,
            self.next_id,
            interval.as_secs_f64()
        );

        self.tasks.push(task);
        self.next_id += 1;
    }

    /// Process due tasks
    fn tick(&mut self) {
        let mut tasks_to_reschedule = Vec::new();

        // Process all due tasks
        while let Some(task) = self.tasks.peek() {
            if !task.is_due() {
                break; // Heap is ordered, so no more due tasks
            }

            // Remove and execute the task
            if let Some(mut task) = self.tasks.pop() {
                let elapsed = Instant::now()
                    .duration_since(task.execute_at)
                    .as_millis();

                print!("[{:>5}ms] Executing: {} ", elapsed, task.name);

                if task.interval.is_some() {
                    println!("(recurring)");
                } else {
                    println!("(one-time)");
                }

                task.execute();

                // Reschedule if recurring
                if task.interval.is_some() {
                    task.reschedule();
                    tasks_to_reschedule.push(task);
                }
            }
        }

        // Add rescheduled tasks back to the heap
        for task in tasks_to_reschedule {
            self.tasks.push(task);
        }
    }

    /// Get number of scheduled tasks
    fn task_count(&self) -> usize {
        self.tasks.len()
    }

    /// Get time until next task (if any)
    #[allow(dead_code)]
    fn next_task_in(&self) -> Option<Duration> {
        self.tasks.peek().map(|task| {
            let now = Instant::now();
            if task.execute_at > now {
                task.execute_at.duration_since(now)
            } else {
                Duration::from_secs(0)
            }
        })
    }
}

// ============================================================================
// WHAT RUST DOES UNDER THE HOOD
// ============================================================================
// 1. BINARYHEAP
//    BinaryHeap is implemented as a binary tree stored in a Vec.
//    It provides O(1) peek and O(log n) insert/pop.
//    We use it as a priority queue for efficient task scheduling.
//
// 2. INSTANT VS SYSTEMTIME
//    Instant: Monotonic clock, never goes backwards
//    SystemTime: Wall clock, can jump (NTP, timezone changes)
//    For scheduling, Instant is better (immune to time changes).
//
// 3. DURATION
//    Duration represents a time span (seconds + nanoseconds).
//    Operations are checked for overflow in debug mode.
//    All math is done with u64, making it very efficient.
//
// 4. CLOSURES AND TRAIT OBJECTS
//    Box<dyn Fn()> is a trait object - dynamically dispatched closure.
//    It's stored on the heap and can be any type that implements Fn().
//    This allows different tasks to have different implementations.
//
// 5. THREAD::SLEEP
//    Yields the CPU to other threads for at least the specified duration.
//    Actual sleep time may be longer (scheduler overhead).
//    On Linux: Uses nanosleep syscall
//    On Windows: Uses SleepEx
//
// 6. MEMORY LAYOUT
//    - Task: ~80 bytes (Instant=16, Duration=16, Box=16, id=8, name=24)
//    - BinaryHeap: Vec + ordering overhead
//    - Total: O(n) where n = number of scheduled tasks
//
// 7. PERFORMANCE
//    - Schedule task: O(log n)
//    - Execute next task: O(log n)
//    - Check if task due: O(1)
//    - Reschedule: O(log n)
//
//    For 1000 scheduled tasks:
//    - Schedule: ~100 nanoseconds
//    - Pop next: ~100 nanoseconds
//    - Tick overhead: ~10 microseconds

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. BinaryHeap provides efficient priority queue operations
// 2. Instant is monotonic and doesn't jump (unlike SystemTime)
// 3. Duration represents time spans with nanosecond precision
// 4. Box<dyn Fn()> allows storing different closures together
// 5. Trait objects enable dynamic dispatch (runtime polymorphism)
// 6. Thread::sleep yields CPU to other threads
// 7. Reversing Ord turns max-heap into min-heap
// 8. Recurring tasks need rescheduling after execution

// ============================================================================
// SCHEDULING ALGORITHMS
// ============================================================================
// 1. PRIORITY QUEUE (this implementation)
//    - Uses BinaryHeap to maintain tasks sorted by execution time
//    - O(log n) to add/remove tasks
//    - O(1) to peek next task
//    - Best for: Variable intervals, one-time tasks
//
// 2. TIME WHEEL
//    - Circular buffer with slots for time ranges
//    - O(1) to schedule tasks in near future
//    - O(n) for distant future (overflow bucket)
//    - Best for: Fixed intervals, high throughput
//
// 3. TIMER HEAP (Tokio's approach)
//    - Hierarchical time wheel + heap for far future
//    - Combines benefits of both approaches
//    - Best for: General purpose async runtime

// ============================================================================
// WHY THIS MATTERS
// ============================================================================
// Task schedulers are fundamental to:
// - Web servers (cleanup, background jobs)
// - Databases (vacuum, backups, replication)
// - Operating systems (cron, systemd timers)
// - Distributed systems (leader election, heartbeats)
// - Game engines (update loops, animations)
//
// Understanding schedulers helps you:
// - Build background job systems
// - Implement game loops and animations
// - Create monitoring and alerting systems
// - Optimize periodic task execution

// ============================================================================
// IMPROVEMENTS FOR PRODUCTION
// ============================================================================
// 1. Use tokio for true async execution (no blocking)
// 2. Add task cancellation with Arc<AtomicBool>
// 3. Implement cron expression parser
// 4. Add error handling and task retry logic
// 5. Persist scheduled tasks to disk
// 6. Support task priorities beyond time
// 7. Add metrics (execution time, failure rate)
// 8. Implement graceful shutdown
// 9. Add task dependencies (run B after A completes)
// 10. Support distributed scheduling (across multiple nodes)

// ============================================================================
// COMMON BEGINNER MISTAKES
// ============================================================================
// ❌ Using SystemTime instead of Instant (can go backwards!)
// ❌ Forgetting to reschedule recurring tasks
// ❌ Not checking if task is due before executing
// ❌ Using busy-wait instead of sleep (wastes CPU)
// ❌ Wrong Ord implementation (max-heap instead of min-heap)
// ❌ Not handling task execution time (affects next execution)
// ❌ Blocking the scheduler with long-running tasks
// ❌ Memory leak: Not removing one-time tasks after execution
