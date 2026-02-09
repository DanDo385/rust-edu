// Lab 51: Concurrent Web Crawler
//
// Multi-threaded web crawler architecture with URL queue, visited set,
// rate limiting, and respectful crawling. Demonstrates async/await
// concepts, concurrency patterns, and network programming in Rust.
//
// Key concepts:
// - BFS (breadth-first) crawling with a URL queue
// - Deduplication via visited set (HashSet)
// - URL normalization and link extraction
// - robots.txt compliance
// - Rate limiting to avoid overwhelming servers
// - Arc<Mutex<T>> for safe shared state across threads

use std::collections::{HashSet, VecDeque};

// ============================================================================
// CRAWLER CONFIGURATION
// ============================================================================

/// Configuration for the web crawler.
#[derive(Clone, Debug)]
pub struct CrawlerConfig {
    pub max_depth: u32,
    pub max_pages: usize,
    pub worker_threads: usize,
    pub rate_limit_ms: u64,
    pub respect_robots_txt: bool,
    pub user_agent: String,
}

impl Default for CrawlerConfig {
    fn default() -> Self {
        CrawlerConfig {
            max_depth: 3,
            max_pages: 50,
            worker_threads: 8,
            rate_limit_ms: 100,
            respect_robots_txt: true,
            user_agent: "RustEduCrawler/1.0".to_string(),
        }
    }
}

// ============================================================================
// URL ENTRY
// ============================================================================

/// A URL queued for crawling, with depth and parent tracking.
#[derive(Clone, Debug, PartialEq)]
pub struct UrlEntry {
    pub url: String,
    pub depth: u32,
    pub parent: Option<String>,
}

impl UrlEntry {
    /// Create a new URL entry.
    pub fn new(url: String, depth: u32, parent: Option<String>) -> Self {
        UrlEntry { url, depth, parent }
    }

    /// Create a seed URL entry (depth 0, no parent).
    pub fn seed(url: String) -> Self {
        UrlEntry {
            url,
            depth: 0,
            parent: None,
        }
    }
}

// ============================================================================
// URL QUEUE
// ============================================================================

/// A FIFO queue of URLs to crawl (breadth-first order).
///
/// Ownership: UrlQueue owns the VecDeque of entries. In a concurrent
/// crawler, this would be wrapped in Arc<Mutex<UrlQueue>>.
pub struct UrlQueue {
    queue: VecDeque<UrlEntry>,
}

impl UrlQueue {
    /// Create an empty URL queue.
    pub fn new() -> Self {
        UrlQueue {
            queue: VecDeque::new(),
        }
    }

    /// Create a queue pre-populated with seed URLs.
    pub fn from_seeds(seed_urls: Vec<String>) -> Self {
        let mut queue = UrlQueue::new();
        for url in seed_urls {
            queue.push(UrlEntry::seed(url));
        }
        queue
    }

    /// Add a URL entry to the back of the queue.
    pub fn push(&mut self, entry: UrlEntry) {
        self.queue.push_back(entry);
    }

    /// Remove and return the next URL from the front of the queue.
    pub fn pop(&mut self) -> Option<UrlEntry> {
        self.queue.pop_front()
    }

    /// Return the number of URLs in the queue.
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    /// Check if the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

impl Default for UrlQueue {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// VISITED SET
// ============================================================================

/// Tracks which URLs have been visited to avoid duplicate crawling.
///
/// Uses a HashSet for O(1) average-case membership checks.
pub struct VisitedSet {
    visited: HashSet<String>,
}

impl VisitedSet {
    /// Create an empty visited set.
    pub fn new() -> Self {
        VisitedSet {
            visited: HashSet::new(),
        }
    }

    /// Mark a URL as visited. Returns true if it was newly inserted.
    pub fn mark_visited(&mut self, url: String) -> bool {
        self.visited.insert(url)
    }

    /// Check if a URL has already been visited.
    pub fn is_visited(&self, url: &str) -> bool {
        self.visited.contains(url)
    }

    /// Return the number of visited URLs.
    pub fn count(&self) -> usize {
        self.visited.len()
    }
}

impl Default for VisitedSet {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// CRAWL RESULT
// ============================================================================

/// The result of crawling a single page.
#[derive(Debug, Clone)]
pub struct CrawlResult {
    pub url: String,
    pub links_found: Vec<String>,
    pub status: CrawlStatus,
}

/// Status of a crawl attempt.
#[derive(Debug, Clone, PartialEq)]
pub enum CrawlStatus {
    Success,
    Error(String),
    Skipped(String),
}

// ============================================================================
// CRAWLER ERROR
// ============================================================================

/// Errors that can occur during crawling.
#[derive(Debug, Clone, PartialEq)]
pub enum CrawlerError {
    NetworkError,
    ParseError,
    RateLimitExceeded,
}

impl std::fmt::Display for CrawlerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CrawlerError::NetworkError => write!(f, "Network error"),
            CrawlerError::ParseError => write!(f, "Parse error"),
            CrawlerError::RateLimitExceeded => write!(f, "Rate limit exceeded"),
        }
    }
}

// ============================================================================
// LINK EXTRACTION
// ============================================================================

/// Extract and normalize links from HTML content.
///
/// This is a simplified parser that looks for href attributes.
/// In production, use a proper HTML parser like `scraper`.
pub fn extract_links(html: &str, base_url: &str) -> Vec<String> {
    let mut links = Vec::new();

    for line in html.lines() {
        if line.contains("href=\"") {
            if let Some(start) = line.find("href=\"") {
                if let Some(end) = line[start + 6..].find('"') {
                    let href = &line[start + 6..start + 6 + end];
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

// ============================================================================
// URL NORMALIZATION
// ============================================================================

/// Normalize a URL relative to a base URL.
///
/// Handles:
/// - Absolute URLs (https://...)
/// - Relative URLs (/path)
/// - Filters out mailto:, javascript:, tel: links
/// - Filters out fragment-only links (#section)
pub fn normalize_url(href: &str, base_url: &str) -> Option<String> {
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

    // Handle relative URLs starting with /
    if href.starts_with('/') {
        return Some(format!("{}{}", base_url, href));
    }

    // Skip fragment-only URLs
    if href.starts_with('#') {
        return None;
    }

    // Relative path
    Some(format!("{}/{}", base_url, href))
}

/// Check if a URL is within the same domain as the base URL.
pub fn is_same_domain(url: &str, base_url: &str) -> bool {
    // Simple check: both start with the same scheme + host
    extract_domain(url) == extract_domain(base_url)
}

/// Extract the domain (scheme + host) from a URL.
pub fn extract_domain(url: &str) -> Option<&str> {
    // Find end of scheme://host
    if let Some(scheme_end) = url.find("://") {
        let after_scheme = &url[scheme_end + 3..];
        if let Some(path_start) = after_scheme.find('/') {
            Some(&url[..scheme_end + 3 + path_start])
        } else {
            Some(url)
        }
    } else {
        None
    }
}

// ============================================================================
// ROBOTS.TXT PARSING
// ============================================================================

/// Check if a path is disallowed by robots.txt rules.
///
/// Simplified parser: checks Disallow directives for the wildcard user-agent.
pub fn is_disallowed(path: &str, robots_txt: &str) -> bool {
    for line in robots_txt.lines() {
        let line = line.trim();
        if line.starts_with("Disallow:") {
            let disallowed = line.trim_start_matches("Disallow:").trim();
            if !disallowed.is_empty() && path.starts_with(disallowed) {
                return true;
            }
        }
    }
    false
}

/// Extract the Crawl-delay value from robots.txt.
pub fn parse_crawl_delay(robots_txt: &str) -> Option<f64> {
    for line in robots_txt.lines() {
        let line = line.trim();
        if line.starts_with("Crawl-delay:") {
            let value = line.trim_start_matches("Crawl-delay:").trim();
            return value.parse::<f64>().ok();
        }
    }
    None
}

// ============================================================================
// BFS CRAWL SIMULATION (pure logic, no network)
// ============================================================================

/// Simulate a BFS crawl over a mock website graph.
///
/// The `site_map` closure takes a URL and returns a list of links found on that page.
/// Returns all visited URLs in BFS order.
pub fn simulate_bfs_crawl<F>(
    seed_urls: Vec<String>,
    max_depth: u32,
    max_pages: usize,
    site_map: F,
) -> Vec<String>
where
    F: Fn(&str) -> Vec<String>,
{
    let mut queue = UrlQueue::from_seeds(seed_urls);
    let mut visited = VisitedSet::new();
    let mut crawled = Vec::new();

    while let Some(entry) = queue.pop() {
        if crawled.len() >= max_pages {
            break;
        }

        if visited.is_visited(&entry.url) {
            continue;
        }

        if entry.depth > max_depth {
            continue;
        }

        visited.mark_visited(entry.url.clone());
        crawled.push(entry.url.clone());

        // "Fetch" the page and discover links
        let links = site_map(&entry.url);
        for link in links {
            if !visited.is_visited(&link) {
                queue.push(UrlEntry::new(link, entry.depth + 1, Some(entry.url.clone())));
            }
        }
    }

    crawled
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_queue_basic() {
        let mut q = UrlQueue::new();
        q.push(UrlEntry::seed("https://example.com".into()));
        assert_eq!(q.len(), 1);
        assert!(!q.is_empty());
    }

    #[test]
    fn test_normalize_url_absolute() {
        let result = normalize_url("https://example.com/page", "https://base.com");
        assert_eq!(result, Some("https://example.com/page".into()));
    }

    #[test]
    fn test_normalize_url_skips_mailto() {
        let result = normalize_url("mailto:test@example.com", "https://base.com");
        assert_eq!(result, None);
    }
}
