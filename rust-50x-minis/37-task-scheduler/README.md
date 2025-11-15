# Project 37: Task Scheduler

## Overview
Build a cron-like task scheduler that runs jobs at specified times or intervals. Learn about time handling, delayed execution, scheduling algorithms, and basic async programming in Rust.

## Concepts Taught
- **Time handling** with std::time
- **Scheduling algorithms** (interval, delayed, cron-like)
- **Closures** as scheduled tasks
- **Trait objects** (Box<dyn Fn()>)
- **Duration and Instant**
- **Thread::sleep** for timing
- **Priority queues** with BinaryHeap
- **Date/time parsing**
- **Background task execution**

## Why Task Schedulers?

Task schedulers are essential for:
- **Cron jobs**: Running periodic maintenance tasks
- **Background workers**: Processing queues, sending emails
- **Job orchestration**: ETL pipelines, data processing
- **System automation**: Backups, cleanups, monitoring

### Real-World Examples

- **Linux cron**: System task scheduler
- **Kubernetes CronJobs**: Container-based scheduled tasks
- **AWS EventBridge**: Cloud-based scheduler
- **APScheduler (Python)**: Application-level scheduler
- **Sidekiq (Ruby)**: Background job processor

## Scheduling Types

### 1. Interval-Based
Run every N seconds/minutes/hours:
```
Every 5 minutes: Check API
Every hour: Clean cache
Every day: Backup database
```

### 2. Delayed Execution
Run once after delay:
```
In 30 seconds: Send confirmation email
In 1 hour: Mark session as expired
```

### 3. Cron-Style
Run at specific times (minute, hour, day, month):
```
0 0 * * * - Daily at midnight
0 9 * * 1 - Every Monday at 9 AM
*/15 * * * * - Every 15 minutes
```

## Running This Project

```bash
cd 37-task-scheduler
cargo run
```

## How It Works

The scheduler maintains a priority queue of scheduled tasks:

1. **Task Registration**: Add tasks with execution time
2. **Scheduling Loop**: Continuously check for due tasks
3. **Execution**: Run tasks when their time arrives
4. **Rescheduling**: For recurring tasks, calculate next run time

## Performance Considerations

**Scheduling Overhead**:
- BinaryHeap operations: O(log n) for insert/pop
- Peek next task: O(1)
- Memory: O(n) for n scheduled tasks

**Timing Accuracy**:
- Thread::sleep has millisecond precision on most systems
- For microsecond precision, use tokio or async-std
- System load affects actual execution time

**Scalability**:
- Single-threaded: ~100-1000 tasks/second
- Multi-threaded: Use tokio for true concurrency
- For millions of tasks, use distributed systems (Kubernetes, AWS Lambda)

## Comparison: Rust vs Other Languages

| Feature | Rust | Python (APScheduler) | Node.js (node-cron) |
|---------|------|---------------------|---------------------|
| Performance | Excellent | Moderate | Good |
| Memory usage | Low | High (GC overhead) | Moderate |
| Concurrency | Excellent (tokio) | Limited (GIL) | Good (event loop) |
| Type safety | Compile-time | Runtime | Runtime |
| Learning curve | Steep | Easy | Easy |

## Additional Challenges

1. **Cron Expression Parser**: Parse and execute cron expressions ("*/5 * * * *")

2. **Persistent Scheduler**: Save scheduled tasks to disk, reload on restart

3. **Async Scheduler**: Use tokio for true concurrent task execution

4. **Task Cancellation**: Allow tasks to be removed before execution

5. **Task History**: Track execution times, failures, and results

6. **Priority Scheduling**: High-priority tasks run before low-priority ones

7. **Retry Logic**: Automatically retry failed tasks with backoff

8. **Web Dashboard**: HTTP API to view/add/remove scheduled tasks

## Future Directions

- **Next**: CLI To-Do App with argument parsing (Project 38)
- **Later**: Build a concurrent web crawler with scheduling (Project 48)
- **Advanced**: Distributed task queue with message bus (Project 30)

## Expected Output

You should see:
- Tasks being scheduled at specific times
- Countdown to next task execution
- Tasks running at their scheduled times
- Recurring tasks being rescheduled
- Clean scheduling output with timestamps
