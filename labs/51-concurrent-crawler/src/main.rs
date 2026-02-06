// Project 48: Concurrent Web Crawler (CAPSTONE)
//
// Multi-threaded web crawler with URL queue, visited set, rate limiting,
// and respectful crawling. Demonstrates async/await, concurrency, and
// network programming in Rust.

use std::collections::{HashSet, VecDeque};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::time::sleep;

// Note: For compilation, add these dependencies to Cargo.toml
// This is a demonstration of the architecture and patterns

fn main() {
    println!("=== Concurrent Web Crawler ===\n");

    // In real implementation, would use #[tokio::main]
    // For this educational example, we'll demonstrate the concepts synchronously

    // Create crawler configuration
    let config = CrawlerConfig {
        max_depth: 3,
        max_pages: 50,
        worker_threads: 8,
        rate_limit_ms: 100, // 10 requests per second
        respect_robots_txt: true,
        user_agent: "RustEduCrawler/1.0".to_string(),
    };

    println!("Configuration:");
    println!("  Max depth: {}", config.max_depth);
    println!("  Max pages: {}", config.max_pages);
    println!("  Worker threads: {}", config.worker_threads);
    println!("  Rate limit: {} ms between requests", config.rate_limit_ms);
    println!();

    // Demonstrate crawler components
    demonstrate_url_queue();
    demonstrate_visited_set();
    demonstrate_rate_limiting();
    demonstrate_concurrent_fetching();
    demonstrate_link_extraction();
}

// ============================================================================
// CRAWLER CONFIGURATION
// ============================================================================

#[derive(Clone)]
struct CrawlerConfig {
    max_depth: u32,
    max_pages: usize,
    worker_threads: usize,
    rate_limit_ms: u64,
    respect_robots_txt: bool,
    user_agent: String,
}

// ============================================================================
// CRAWLER STATE
// ============================================================================

struct Crawler {
    config: CrawlerConfig,
    queue: Arc<Mutex<UrlQueue>>,
    visited: Arc<Mutex<HashSet<String>>>,
    pages_crawled: Arc<Mutex<usize>>,
    stats: Arc<Mutex<CrawlerStats>>,
}

struct UrlQueue {
    queue: VecDeque<UrlEntry>,
}

#[derive(Clone)]
struct UrlEntry {
    url: String,
    depth: u32,
    parent: Option<String>,
}

#[derive(Default)]
struct CrawlerStats {
    pages_crawled: usize,
    links_found: usize,
    errors: usize,
    total_fetch_time: Duration,
}

impl Crawler {
    fn new(config: CrawlerConfig, seed_urls: Vec<String>) -> Self {
        let mut queue = UrlQueue {
            queue: VecDeque::new(),
        };

        // Add seed URLs to queue
        for url in seed_urls {
            queue.queue.push_back(UrlEntry {
                url,
                depth: 0,
                parent: None,
            });
        }

        Crawler {
            config,
            queue: Arc::new(Mutex::new(queue)),
            visited: Arc::new(Mutex::new(HashSet::new())),
            pages_crawled: Arc::new(Mutex::new(0)),
            stats: Arc::new(Mutex::new(CrawlerStats::default())),
        }
    }

    fn next_url(&self) -> Option<UrlEntry> {
        let mut queue = self.queue.lock().unwrap();
        queue.queue.pop_front()
    }

    fn add_urls(&self, urls: Vec<UrlEntry>) {
        let mut queue = self.queue.lock().unwrap();
        let visited = self.visited.lock().unwrap();

        for entry in urls {
            // Skip if already visited or exceeds max depth
            if visited.contains(&entry.url) || entry.depth > self.config.max_depth {
                continue;
            }

            queue.queue.push_back(entry);
        }
    }

    fn mark_visited(&self, url: String) {
        let mut visited = self.visited.lock().unwrap();
        visited.insert(url);
    }

    fn is_visited(&self, url: &str) -> bool {
        let visited = self.visited.lock().unwrap();
        visited.contains(url)
    }

    fn should_stop(&self) -> bool {
        let pages = self.pages_crawled.lock().unwrap();
        *pages >= self.config.max_pages
    }
}

// ============================================================================
// URL QUEUE DEMONSTRATION
// ============================================================================

fn demonstrate_url_queue() {
    println!("--- URL Queue Management ---");

    let mut queue = VecDeque::new();

    // Add initial URLs
    let seed_urls = vec![
        "https://example.com/",
        "https://example.com/about",
        "https://example.com/contact",
    ];

    println!("Adding seed URLs:");
    for url in seed_urls {
        queue.push_back(url.to_string());
        println!("  + {}", url);
    }
    println!("Queue size: {}", queue.len());
    println!();

    // Process URLs (FIFO - breadth-first)
    println!("Processing URLs (BFS order):");
    let mut processed = 0;
    while let Some(url) = queue.pop_front() {
        processed += 1;
        println!("  [{}] Processing: {}", processed, url);

        // Simulate finding new links
        if url.ends_with('/') {
            let new_url = format!("{}page{}", url, processed);
            queue.push_back(new_url.clone());
            println!("      Found: {}", new_url);
        }

        if processed >= 5 {
            break; // Limit for demonstration
        }
    }
    println!();
}

// ============================================================================
// VISITED SET DEMONSTRATION
// ============================================================================

fn demonstrate_visited_set() {
    println!("--- Visited Set (Deduplication) ---");

    let mut visited = HashSet::new();

    let urls = vec![
        "https://example.com/",
        "https://example.com/about",
        "https://example.com/", // Duplicate!
        "https://example.com/contact",
        "https://example.com/about", // Duplicate!
    ];

    println!("Processing URLs with deduplication:");
    let mut crawled = 0;
    let mut skipped = 0;

    for url in urls {
        if visited.contains(url) {
            println!("  ⏭️  SKIP: {} (already visited)", url);
            skipped += 1;
        } else {
            println!("  ✅ CRAWL: {}", url);
            visited.insert(url.to_string());
            crawled += 1;
        }
    }

    println!();
    println!("Summary:");
    println!("  Crawled: {}", crawled);
    println!("  Skipped: {}", skipped);
    println!("  Visited set size: {}", visited.len());
    println!();
}

// ============================================================================
// RATE LIMITING DEMONSTRATION
// ============================================================================

fn demonstrate_rate_limiting() {
    println!("--- Rate Limiting ---");

    let rate_limit = Duration::from_millis(200); // 5 requests per second
    let urls = vec!["url1", "url2", "url3", "url4", "url5"];

    println!("Fetching with rate limit: {} ms between requests", rate_limit.as_millis());
    println!();

    let start = Instant::now();

    for (i, url) in urls.iter().enumerate() {
        let request_start = Instant::now();

        // Simulate HTTP request
        simulate_fetch(url);

        let request_time = request_start.elapsed();
        println!("  [{}] Fetched {} in {:?}", i + 1, url, request_time);

        // Rate limiting: wait before next request
        if i < urls.len() - 1 {
            std::thread::sleep(rate_limit);
        }
    }

    let total_time = start.elapsed();
    println!();
    println!("Total time: {:?}", total_time);
    println!("Average rate: {:.1} req/sec",
        urls.len() as f64 / total_time.as_secs_f64());
    println!();
}

fn simulate_fetch(_url: &str) {
    // Simulate network latency
    std::thread::sleep(Duration::from_millis(50));
}

// ============================================================================
// CONCURRENT FETCHING DEMONSTRATION
// ============================================================================

fn demonstrate_concurrent_fetching() {
    println!("--- Concurrent Fetching with Threads ---");

    let urls = vec![
        "https://example.com/page1",
        "https://example.com/page2",
        "https://example.com/page3",
        "https://example.com/page4",
        "https://example.com/page5",
        "https://example.com/page6",
        "https://example.com/page7",
        "https://example.com/page8",
    ];

    let num_workers = 4;
    println!("Using {} worker threads", num_workers);
    println!();

    // Shared queue
    let queue = Arc::new(Mutex::new(urls.clone()));
    let results = Arc::new(Mutex::new(Vec::new()));

    let start = Instant::now();

    // Spawn worker threads
    let mut handles = vec![];

    for worker_id in 0..num_workers {
        let queue = Arc::clone(&queue);
        let results = Arc::clone(&results);

        let handle = std::thread::spawn(move || {
            loop {
                // Get next URL from queue
                let url = {
                    let mut q = queue.lock().unwrap();
                    q.pop()
                };

                match url {
                    Some(url) => {
                        println!("  [Worker {}] Fetching: {}", worker_id, url);

                        // Simulate fetch
                        let fetch_start = Instant::now();
                        simulate_fetch(&url);
                        let fetch_time = fetch_start.elapsed();

                        // Store result
                        let mut r = results.lock().unwrap();
                        r.push(FetchResult {
                            url,
                            worker_id,
                            duration: fetch_time,
                            links_found: 5, // Simulated
                        });
                    }
                    None => break, // Queue empty, worker done
                }
            }
        });

        handles.push(handle);
    }

    // Wait for all workers to finish
    for handle in handles {
        handle.join().unwrap();
    }

    let total_time = start.elapsed();

    // Print results
    println!();
    println!("Results:");
    let results = results.lock().unwrap();
    for (i, result) in results.iter().enumerate() {
        println!("  [{}] {} - {:?} (worker {})",
            i + 1, result.url, result.duration, result.worker_id);
    }

    println!();
    println!("Summary:");
    println!("  Pages fetched: {}", results.len());
    println!("  Total time: {:?}", total_time);
    println!("  Average time per page: {:?}", total_time / results.len() as u32);
    println!("  Speedup vs sequential: ~{}x", urls.len() as f64 * 50.0 / total_time.as_millis() as f64);
    println!();
}

#[derive(Debug)]
struct FetchResult {
    url: String,
    worker_id: usize,
    duration: Duration,
    links_found: usize,
}

// ============================================================================
// LINK EXTRACTION DEMONSTRATION
// ============================================================================

fn demonstrate_link_extraction() {
    println!("--- Link Extraction and Normalization ---");

    // Simulated HTML content
    let html = r#"
        <html>
            <body>
                <a href="/about">About</a>
                <a href="/contact">Contact</a>
                <a href="https://external.com">External</a>
                <a href="/products/item1">Product 1</a>
                <a href="/products/item2">Product 2</a>
                <a href="mailto:test@example.com">Email</a>
                <a href="javascript:void(0)">JavaScript</a>
                <a href="/about#section">About (with fragment)</a>
            </body>
        </html>
    "#;

    let base_url = "https://example.com";

    println!("Extracting links from page: {}", base_url);
    println!();

    let links = extract_links_simple(html, base_url);

    println!("Found {} valid links:", links.len());
    for (i, link) in links.iter().enumerate() {
        println!("  [{}] {}", i + 1, link);
    }
    println!();

    // Demonstrate URL normalization
    demonstrate_url_normalization();
}

fn extract_links_simple(html: &str, base_url: &str) -> Vec<String> {
    let mut links = Vec::new();

    // Simple regex-style extraction (in production, use proper HTML parser)
    for line in html.lines() {
        if line.contains("href=\"") {
            if let Some(start) = line.find("href=\"") {
                if let Some(end) = line[start + 6..].find('"') {
                    let href = &line[start + 6..start + 6 + end];

                    // Filter and normalize
                    if let Some(normalized) = normalize_url(href, base_url) {
                        links.push(normalized);
                    }
                }
            }
        }
    }

    // Deduplicate
    links.sort();
    links.dedup();
    links
}

fn normalize_url(href: &str, base_url: &str) -> Option<String> {
    // Skip non-HTTP links
    if href.starts_with("mailto:")
        || href.starts_with("javascript:")
        || href.starts_with("tel:")
    {
        return None;
    }

    // Handle absolute URLs
    if href.starts_with("http://") || href.starts_with("https://") {
        return Some(href.to_string());
    }

    // Handle relative URLs
    if href.starts_with('/') {
        return Some(format!("{}{}", base_url, href));
    }

    // Handle fragment-only URLs
    if href.starts_with('#') {
        return None; // Skip fragments
    }

    // Relative path (simplified)
    Some(format!("{}/{}", base_url, href))
}

fn demonstrate_url_normalization() {
    println!("--- URL Normalization ---");

    let test_cases = vec![
        ("http://example.com/path?b=2&a=1", "http://example.com/path?a=1&b=2"),
        ("http://example.com/path/", "http://example.com/path/"),
        ("http://EXAMPLE.com/Path", "http://example.com/path"),
        ("http://example.com:80/path", "http://example.com/path"),
        ("http://example.com/./path", "http://example.com/path"),
    ];

    println!("Examples of URL normalization:");
    for (input, expected) in test_cases {
        println!("  {} →", input);
        println!("  {}", expected);
        println!();
    }
}

// ============================================================================
// ROBOTS.TXT DEMONSTRATION
// ============================================================================

#[allow(dead_code)]
fn demonstrate_robots_txt() {
    println!("--- Robots.txt Compliance ---");

    let robots_txt = r#"
User-agent: *
Disallow: /admin/
Disallow: /private/
Crawl-delay: 1

User-agent: Googlebot
Crawl-delay: 0.5
    "#;

    println!("Robots.txt content:");
    println!("{}", robots_txt);
    println!();

    let test_urls = vec![
        "/",
        "/about",
        "/admin/users",
        "/private/data",
        "/public/page",
    ];

    println!("Checking URLs against robots.txt:");
    for url in test_urls {
        let allowed = !is_disallowed_simple(url, robots_txt);
        let status = if allowed { "✅ ALLOWED" } else { "❌ BLOCKED" };
        println!("  {} - {}", status, url);
    }
    println!();
}

fn is_disallowed_simple(path: &str, robots_txt: &str) -> bool {
    for line in robots_txt.lines() {
        let line = line.trim();
        if line.starts_with("Disallow:") {
            let disallowed = line.trim_start_matches("Disallow:").trim();
            if path.starts_with(disallowed) {
                return true;
            }
        }
    }
    false
}

// ============================================================================
// ERROR HANDLING AND RETRY LOGIC
// ============================================================================

#[allow(dead_code)]
fn fetch_with_retry(url: &str, max_retries: u32) -> Result<String, CrawlerError> {
    let mut retries = 0;
    let mut backoff = Duration::from_millis(100);

    loop {
        match fetch_url_simulated(url) {
            Ok(content) => return Ok(content),
            Err(e) => {
                retries += 1;
                if retries >= max_retries {
                    return Err(e);
                }

                println!("  ⚠️  Retry {}/{} for {} after {:?}",
                    retries, max_retries, url, backoff);

                std::thread::sleep(backoff);
                backoff *= 2; // Exponential backoff
            }
        }
    }
}

fn fetch_url_simulated(_url: &str) -> Result<String, CrawlerError> {
    // Simulate occasional failures
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hash, Hasher};

    let mut hasher = RandomState::new().build_hasher();
    _url.hash(&mut hasher);

    if hasher.finish() % 5 == 0 {
        Err(CrawlerError::NetworkError)
    } else {
        Ok("<html><body>Content</body></html>".to_string())
    }
}

#[derive(Debug)]
enum CrawlerError {
    NetworkError,
    ParseError,
    RateLimitExceeded,
}

// ============================================================================
// WHAT RUST DOES UNDER THE HOOD
// ============================================================================
// 1. ARC (ATOMIC REFERENCE COUNTING)
//    Arc<Mutex<T>> allows sharing data across threads safely.
//    Arc uses atomic operations to track references (no locks for counting).
//    When last Arc is dropped, inner value is freed automatically.
//    Zero-cost compared to manual reference counting in C++.
//
// 2. MUTEX SYNCHRONIZATION
//    Mutex<T> ensures only one thread accesses data at a time.
//    lock() blocks until lock is acquired.
//    Lock is automatically released when MutexGuard goes out of scope.
//    No risk of forgetting to unlock (unlike pthread_mutex in C).
//
// 3. THREAD SPAWNING
//    std::thread::spawn creates OS thread (not green thread).
//    Each thread gets its own stack (~2MB on Linux).
//    Thread ownership rules prevent data races at compile time.
//    join() blocks until thread completes and returns result.
//
// 4. CHANNEL COMMUNICATION
//    Channels (mpsc) are lock-free for single producer.
//    Send/Recv use efficient algorithms (no mutex for each message).
//    Receiver can block or poll (try_recv).
//    Channel closed when all senders dropped (automatic cleanup).
//
// 5. ASYNC/AWAIT (Tokio)
//    Async functions compile to state machines.
//    Tokio runtime multiplexes thousands of tasks on few threads.
//    No stack per task (unlike OS threads) - very memory efficient.
//    I/O operations don't block threads - cooperative multitasking.
//
// 6. MEMORY EFFICIENCY
//    HashSet<String> stores owned strings, but they're deduplicated.
//    VecDeque is ring buffer - O(1) push/pop on both ends.
//    String is pointer + length + capacity (3 words on stack).
//    URLs stored once in visited set, referenced elsewhere.

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. Concurrent crawling is much faster than sequential
// 2. Use HashSet to track visited URLs (deduplication)
// 3. Rate limiting prevents overwhelming servers
// 4. Arc<Mutex<T>> enables safe shared state across threads
// 5. Thread pools prevent creating too many threads
// 6. Always respect robots.txt (legal and ethical)
// 7. Normalize URLs before checking if visited
// 8. Handle errors gracefully with retry logic
// 9. BFS (queue) crawls by depth, DFS (stack) goes deep first
// 10. Async I/O (Tokio) is more efficient than threads for I/O-bound tasks

// ============================================================================
// PRODUCTION CONSIDERATIONS
// ============================================================================
// In a real web crawler, you would add:
// 1. Proper HTML parsing (scraper, select.rs)
// 2. HTTP client with connection pooling (reqwest)
// 3. Async I/O for better performance (Tokio, async-std)
// 4. Persistent queue (RocksDB, Redis) for resume capability
// 5. Bloom filter for memory-efficient visited set
// 6. Per-domain rate limiting (separate limit per host)
// 7. Robots.txt caching and parsing
// 8. sitemap.xml support for efficient discovery
// 9. Content-Type detection (skip images, PDFs unless needed)
// 10. DNS caching to reduce lookup overhead
// 11. Compression support (gzip, brotli)
// 12. Proxy support and rotation (avoid IP bans)
// 13. JavaScript rendering (headless Chrome) for SPA sites
// 14. Distributed crawling (multiple machines coordinate)
// 15. Analytics and monitoring (Prometheus, Grafana)

// ============================================================================
// PERFORMANCE COMPARISON
// ============================================================================
// Sequential crawling:
//   - 50 pages × 100ms = 5 seconds
//   - Simple, predictable
//   - Low resource usage
//
// Threaded crawling (8 threads):
//   - 50 pages × 100ms / 8 threads = ~625ms
//   - 8x speedup (ideal case)
//   - Higher CPU and memory usage
//   - Limited by number of cores
//
// Async crawling (Tokio, 1000 concurrent):
//   - 50 pages × 100ms / 1000 tasks = ~5ms (if server can handle)
//   - Limited by I/O, not CPU
//   - Very memory efficient (no stack per task)
//   - Best for I/O-bound workloads like web crawling

// ============================================================================
// COMMON MISTAKES TO AVOID
// ============================================================================
// ❌ Not deduplicating URLs (infinite loops)
// ❌ Ignoring robots.txt (unethical and illegal)
// ❌ No rate limiting (overwhelming servers, getting banned)
// ❌ Not normalizing URLs (crawling same page multiple times)
// ❌ Blocking on I/O in async context (defeats the purpose)
// ❌ Creating unlimited threads (resource exhaustion)
// ❌ Not handling errors (crawler crashes on first failure)
// ❌ Following all links blindly (crawling logout, delete, etc.)
// ❌ Not respecting 429 (Too Many Requests) responses
// ❌ Crawling too deep (exponential link growth)

// ============================================================================
// ARCHITECTURE PATTERNS
// ============================================================================
// Producer-Consumer:
//   - URL frontier produces work
//   - Worker threads consume work
//   - Channels for communication
//
// Thread Pool:
//   - Fixed number of workers
//   - Prevents resource exhaustion
//   - Rayon, Tokio provide this built-in
//
// Actor Model:
//   - Each domain gets an actor
//   - Actor manages rate limiting for its domain
//   - Actix, Bastion implement this pattern
//
// Pipeline:
//   - Fetch → Parse → Extract → Filter → Queue
//   - Each stage can run in parallel
//   - Maximizes throughput
