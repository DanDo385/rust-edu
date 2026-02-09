//! # Concurrent Crawler Demo

use concurrent_crawler::solution::{CrawlerConfig, UrlEntry, UrlQueue, VisitedSet};

fn main() {
    println!("=== Concurrent Crawler Demo ===");

    let config = CrawlerConfig::default();
    println!("max_depth={} max_pages={}", config.max_depth, config.max_pages);

    let mut queue = UrlQueue::from_seeds(vec!["https://example.com".to_string()]);
    queue.push(UrlEntry::new(
        "https://example.com/docs".to_string(),
        1,
        Some("https://example.com".to_string()),
    ));

    let mut visited = VisitedSet::new();
    while let Some(entry) = queue.pop() {
        if visited.mark_visited(entry.url.clone()) {
            println!("crawl depth={} url={}", entry.depth, entry.url);
        }
    }

    println!("visited pages: {}", visited.count());
}
