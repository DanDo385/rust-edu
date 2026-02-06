# Project 48: Concurrent Web Crawler (CAPSTONE)

## Overview
Build a high-performance multi-threaded web crawler that discovers and fetches web pages concurrently. Implement URL queue management, visited set tracking, rate limiting, and respectful crawling practices.

## Concepts Taught
- **Concurrent programming**: spawning multiple worker threads
- **Async I/O**: non-blocking network operations
- **Thread synchronization**: Arc, Mutex, channels
- **URL queue management**: work distribution across threads
- **Visited set**: preventing duplicate crawling with HashSet
- **Rate limiting**: respecting server resources
- **Robots.txt**: following crawler etiquette
- **Error handling**: dealing with network failures gracefully
- **Backpressure**: preventing resource exhaustion

## Why Web Crawlers Matter

### Real-World Applications
- **Search engines**: Google, Bing crawl the entire web
- **Price monitoring**: Track product prices across sites
- **SEO analysis**: Analyze website structure and links
- **Data collection**: Gather information for research
- **Website testing**: Check for broken links
- **Content monitoring**: Track changes to web pages

### The Challenge
Crawling millions of pages requires:
- **Concurrency**: Single-threaded crawlers are too slow
- **Deduplication**: Don't crawl same URL twice
- **Politeness**: Don't overwhelm servers
- **Resilience**: Handle failures gracefully
- **Scalability**: Efficiently manage state

## Crawler Architecture

### Components
```
┌─────────────────┐
│  URL Frontier   │ ← Queue of URLs to visit
└────────┬────────┘
         │
    ┌────▼─────┐
    │ Scheduler │ ← Distributes work to threads
    └────┬─────┘
         │
    ┌────▼─────────────────┐
    │  Worker Threads      │ ← Fetch and parse pages
    │  (Thread Pool)       │
    └────┬─────────────────┘
         │
    ┌────▼──────┐
    │ Visited   │ ← Track crawled URLs
    │    Set    │
    └───────────┘
```

### Data Flow
1. Start with seed URLs in queue
2. Scheduler assigns URL to available worker
3. Worker fetches page content
4. Worker extracts new links from page
5. New links added to queue (if not visited)
6. Mark current URL as visited
7. Repeat until queue empty or limit reached

## Concurrency Patterns

### Thread Pool
- Fixed number of worker threads
- Prevents resource exhaustion
- Reuses threads (avoids spawn overhead)
- Typical size: CPU cores × 2-4

### Work Stealing
- Idle threads steal work from busy ones
- Better load balancing
- Rayon and Tokio use this internally

### Message Passing
- Channels for thread communication
- Send URLs to workers
- Receive results from workers
- Safe, no shared mutable state

### Shared State
- Arc<Mutex<HashSet>> for visited URLs
- Arc<Mutex<VecDeque>> for URL queue
- Minimizes lock contention
- Consider lock-free structures for extreme performance

## Rate Limiting

### Why It Matters
- Prevents overwhelming servers
- Avoids IP bans
- Respects robots.txt crawl-delay
- Good internet citizenship

### Strategies

#### 1. Fixed Delay
```rust
sleep(Duration::from_millis(100)); // 10 req/sec
```

#### 2. Token Bucket
```rust
// Allow bursts, average rate
bucket.wait_for_token();
fetch_url(url);
```

#### 3. Per-Domain Rate Limiting
```rust
// Different limits per domain
rate_limiter.get_or_insert(domain).wait();
```

#### 4. Adaptive Rate Limiting
```rust
// Slow down on errors, speed up on success
if response.status().is_success() {
    rate *= 1.1; // Increase rate
} else {
    rate *= 0.5; // Decrease rate
}
```

## URL Management

### Normalization
Before adding to queue:
```
http://example.com/page?a=1&b=2
http://example.com/page?b=2&a=1  ← Same page!

→ Normalize: sort query parameters
→ http://example.com/page?a=1&b=2
```

### Deduplication
- Use HashSet to track visited URLs
- Check before fetching
- Prevents infinite loops
- Saves bandwidth and time

### Frontier Prioritization
- **BFS**: Queue (FIFO) - crawl by depth
- **DFS**: Stack (LIFO) - deep dive first
- **Priority**: Heap - crawl important pages first
- **Politeness**: Delay queue per domain

## Parsing Strategies

### HTML Parsing
- Extract `<a href="...">` links
- Handle relative URLs (`/path` → `http://example.com/path`)
- Extract metadata (title, description)
- Find images, scripts, stylesheets

### Link Filtering
```rust
if url.starts_with("http") &&
   !url.contains("logout") &&
   !url.ends_with(".pdf") &&
   same_domain(url, base_url) {
    queue.push(url);
}
```

## Robots.txt Compliance

### Format
```
User-agent: *
Disallow: /private/
Disallow: /admin/
Crawl-delay: 1

User-agent: Googlebot
Crawl-delay: 0.5
```

### Implementation
```rust
1. Fetch http://example.com/robots.txt
2. Parse rules for your user-agent
3. Check each URL against Disallow rules
4. Respect Crawl-delay directive
```

## Error Handling

### Common Errors
- **Network timeouts**: Server unreachable
- **404 Not Found**: Page doesn't exist
- **403 Forbidden**: Access denied
- **500 Server Error**: Server problems
- **DNS failures**: Domain doesn't exist
- **Connection refused**: Server not listening

### Strategies
- **Retry with backoff**: 1s, 2s, 4s, 8s...
- **Skip and continue**: Don't block on one failure
- **Log errors**: Track what went wrong
- **Circuit breaker**: Stop trying failing domains

## Running This Project

```bash
cd 48-concurrent-crawler
cargo run
```

**Dependencies** (add to `Cargo.toml`):
```toml
[dependencies]
tokio = { version = "1.35", features = ["full"] }
reqwest = "0.11"
scraper = "0.18"
url = "2.5"
```

## Expected Output
```
=== Concurrent Web Crawler ===

Configuration:
  Max depth: 3
  Max pages: 50
  Worker threads: 8
  Rate limit: 10 requests/second
  Starting URL: https://example.com

--- Crawling Started ---
[Worker 1] Fetching: https://example.com/
[Worker 2] Fetching: https://example.com/about
[Worker 3] Fetching: https://example.com/contact
✅ [1] https://example.com/ (12 links found)
✅ [2] https://example.com/about (8 links found)
✅ [3] https://example.com/contact (5 links found)
[Worker 4] Fetching: https://example.com/products
...

--- Crawling Complete ---
Pages crawled: 50
Total links found: 342
Unique domains: 3
Average fetch time: 234ms
Errors encountered: 2

Top pages by links:
  1. https://example.com/ (45 outbound links)
  2. https://example.com/sitemap (32 outbound links)
  3. https://example.com/blog (28 outbound links)
```

## Performance Optimization

### 1. Connection Pooling
- Reuse HTTP connections
- Reduces handshake overhead
- Built into reqwest::Client

### 2. DNS Caching
- Cache DNS lookups
- Prevents repeated resolution
- Significant speedup

### 3. Compression
- Accept gzip/brotli encoding
- Reduces bandwidth
- Faster downloads

### 4. Parallel DNS Resolution
- Resolve multiple domains concurrently
- Don't block on DNS

### 5. Lock-Free Data Structures
- Use DashMap instead of Mutex<HashMap>
- Crossbeam for channels
- Better scalability

## Respectful Crawling

### Best Practices
✅ **Identify yourself**: Use proper User-Agent
✅ **Respect robots.txt**: Follow the rules
✅ **Rate limit**: Don't overwhelm servers
✅ **Handle errors**: Retry gracefully
✅ **Follow redirects**: But limit depth
✅ **Cache DNS**: Reduce load on DNS servers

### What NOT to Do
❌ **Ignore robots.txt**: Disrespectful and potentially illegal
❌ **Crawl too fast**: Can be seen as DDoS attack
❌ **Infinite loops**: Follow circular links forever
❌ **Ignore 429 responses**: Server telling you to slow down
❌ **Fake User-Agent**: Pretending to be a browser

## Challenge Extensions
1. Implement distributed crawling (multiple machines)
2. Add content extraction (not just links)
3. Implement incremental crawling (only fetch changed pages)
4. Add bloom filter for URL deduplication (memory efficient)
5. Implement priority queue (crawl important pages first)
6. Add sitemap.xml parsing
7. Implement JavaScript rendering (headless browser)
8. Add content-type detection (skip images/videos)
9. Implement crawl budget per domain
10. Add analytics dashboard (visualize crawl progress)

## Advanced Topics

### Distributed Crawling
- Multiple machines coordinate
- Consistent hashing for URL assignment
- Redis/Kafka for shared queue
- Used by Google, Bing

### Incremental Crawling
- Only fetch changed pages
- Check Last-Modified, ETag headers
- Use If-Modified-Since requests
- Saves bandwidth and time

### Focused Crawling
- Prioritize relevant pages
- Machine learning to predict relevance
- Used for topic-specific search engines

### Deep Web Crawling
- Handle forms, logins
- Execute JavaScript (Selenium, Puppeteer)
- More complex but reaches more content

## Resources
- [Web Crawling Wikipedia](https://en.wikipedia.org/wiki/Web_crawler)
- [Robots Exclusion Protocol](https://www.robotstxt.org/)
- [Scrapy Documentation](https://docs.scrapy.org/) (Python, but good architecture ideas)
- [The Anatomy of a Large-Scale Hypertextual Web Search Engine (Google)](http://infolab.stanford.edu/~backrub/google.html)
- [Mercator: A Scalable, Extensible Web Crawler](https://www.semanticscholar.org/paper/Mercator%3A-A-Scalable%2C-Extensible-Web-Crawler-Heydon-Najork/c745c8ad46e09d0a8f6e4d768f6cf8677d8d1e41)
