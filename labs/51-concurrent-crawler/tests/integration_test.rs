// Lab 51: Concurrent Web Crawler - Integration Tests
//
// Tests for URL queue, visited set, link extraction, URL normalization,
// robots.txt parsing, and BFS crawl simulation. All tests are pure
// (no network access required).

use concurrent_crawler::*;

// ============================================================================
// CRAWLER CONFIG TESTS
// ============================================================================

#[test]
fn test_default_config() {
    let config = CrawlerConfig::default();
    assert_eq!(config.max_depth, 3);
    assert_eq!(config.max_pages, 50);
    assert_eq!(config.worker_threads, 8);
    assert_eq!(config.rate_limit_ms, 100);
    assert!(config.respect_robots_txt);
    assert_eq!(config.user_agent, "RustEduCrawler/1.0");
}

#[test]
fn test_custom_config() {
    let config = CrawlerConfig {
        max_depth: 5,
        max_pages: 100,
        worker_threads: 16,
        rate_limit_ms: 50,
        respect_robots_txt: false,
        user_agent: "CustomBot/2.0".into(),
    };
    assert_eq!(config.max_depth, 5);
    assert_eq!(config.max_pages, 100);
}

// ============================================================================
// URL ENTRY TESTS
// ============================================================================

#[test]
fn test_url_entry_new() {
    let entry = UrlEntry::new(
        "https://example.com/page".into(),
        2,
        Some("https://example.com".into()),
    );
    assert_eq!(entry.url, "https://example.com/page");
    assert_eq!(entry.depth, 2);
    assert_eq!(entry.parent, Some("https://example.com".into()));
}

#[test]
fn test_url_entry_seed() {
    let entry = UrlEntry::seed("https://example.com".into());
    assert_eq!(entry.url, "https://example.com");
    assert_eq!(entry.depth, 0);
    assert_eq!(entry.parent, None);
}

// ============================================================================
// URL QUEUE TESTS
// ============================================================================

#[test]
fn test_url_queue_empty() {
    let queue = UrlQueue::new();
    assert!(queue.is_empty());
    assert_eq!(queue.len(), 0);
}

#[test]
fn test_url_queue_push_pop() {
    let mut queue = UrlQueue::new();
    queue.push(UrlEntry::seed("https://a.com".into()));
    queue.push(UrlEntry::seed("https://b.com".into()));

    assert_eq!(queue.len(), 2);

    let first = queue.pop().unwrap();
    assert_eq!(first.url, "https://a.com");

    let second = queue.pop().unwrap();
    assert_eq!(second.url, "https://b.com");

    assert!(queue.pop().is_none());
}

#[test]
fn test_url_queue_fifo_order() {
    let mut queue = UrlQueue::new();
    for i in 0..5 {
        queue.push(UrlEntry::seed(format!("https://example.com/page{}", i)));
    }

    for i in 0..5 {
        let entry = queue.pop().unwrap();
        assert_eq!(entry.url, format!("https://example.com/page{}", i));
    }
}

#[test]
fn test_url_queue_from_seeds() {
    let seeds = vec![
        "https://a.com".into(),
        "https://b.com".into(),
        "https://c.com".into(),
    ];
    let mut queue = UrlQueue::from_seeds(seeds);

    assert_eq!(queue.len(), 3);
    assert_eq!(queue.pop().unwrap().url, "https://a.com");
    assert_eq!(queue.pop().unwrap().url, "https://b.com");
    assert_eq!(queue.pop().unwrap().url, "https://c.com");
}

// ============================================================================
// VISITED SET TESTS
// ============================================================================

#[test]
fn test_visited_set_empty() {
    let visited = VisitedSet::new();
    assert_eq!(visited.count(), 0);
    assert!(!visited.is_visited("https://example.com"));
}

#[test]
fn test_visited_set_mark_and_check() {
    let mut visited = VisitedSet::new();
    let was_new = visited.mark_visited("https://example.com".into());
    assert!(was_new);
    assert!(visited.is_visited("https://example.com"));
    assert_eq!(visited.count(), 1);
}

#[test]
fn test_visited_set_duplicate() {
    let mut visited = VisitedSet::new();
    assert!(visited.mark_visited("https://example.com".into()));
    assert!(!visited.mark_visited("https://example.com".into())); // already exists
    assert_eq!(visited.count(), 1);
}

#[test]
fn test_visited_set_multiple_urls() {
    let mut visited = VisitedSet::new();
    visited.mark_visited("https://a.com".into());
    visited.mark_visited("https://b.com".into());
    visited.mark_visited("https://c.com".into());

    assert_eq!(visited.count(), 3);
    assert!(visited.is_visited("https://a.com"));
    assert!(visited.is_visited("https://b.com"));
    assert!(visited.is_visited("https://c.com"));
    assert!(!visited.is_visited("https://d.com"));
}

// ============================================================================
// URL NORMALIZATION TESTS
// ============================================================================

#[test]
fn test_normalize_absolute_url() {
    let result = normalize_url("https://external.com/page", "https://example.com");
    assert_eq!(result, Some("https://external.com/page".into()));
}

#[test]
fn test_normalize_relative_url() {
    let result = normalize_url("/about", "https://example.com");
    assert_eq!(result, Some("https://example.com/about".into()));
}

#[test]
fn test_normalize_relative_path() {
    let result = normalize_url("page.html", "https://example.com");
    assert_eq!(result, Some("https://example.com/page.html".into()));
}

#[test]
fn test_normalize_skips_mailto() {
    assert_eq!(normalize_url("mailto:test@example.com", "https://example.com"), None);
}

#[test]
fn test_normalize_skips_javascript() {
    assert_eq!(normalize_url("javascript:void(0)", "https://example.com"), None);
}

#[test]
fn test_normalize_skips_tel() {
    assert_eq!(normalize_url("tel:+1234567890", "https://example.com"), None);
}

#[test]
fn test_normalize_skips_fragment() {
    assert_eq!(normalize_url("#section", "https://example.com"), None);
}

#[test]
fn test_normalize_http_absolute() {
    let result = normalize_url("http://insecure.com", "https://example.com");
    assert_eq!(result, Some("http://insecure.com".into()));
}

// ============================================================================
// LINK EXTRACTION TESTS
// ============================================================================

#[test]
fn test_extract_links_basic() {
    let html = r#"
        <a href="/about">About</a>
        <a href="/contact">Contact</a>
    "#;

    let links = extract_links(html, "https://example.com");
    assert_eq!(links.len(), 2);
    assert!(links.contains(&"https://example.com/about".to_string()));
    assert!(links.contains(&"https://example.com/contact".to_string()));
}

#[test]
fn test_extract_links_filters_mailto() {
    let html = r#"
        <a href="/valid">Valid</a>
        <a href="mailto:test@example.com">Email</a>
    "#;

    let links = extract_links(html, "https://example.com");
    assert_eq!(links.len(), 1);
    assert!(links.contains(&"https://example.com/valid".to_string()));
}

#[test]
fn test_extract_links_filters_javascript() {
    let html = r#"
        <a href="/valid">Valid</a>
        <a href="javascript:void(0)">JS</a>
    "#;

    let links = extract_links(html, "https://example.com");
    assert_eq!(links.len(), 1);
}

#[test]
fn test_extract_links_deduplicates() {
    let html = r#"
        <a href="/page">Page</a>
        <a href="/page">Page Again</a>
    "#;

    let links = extract_links(html, "https://example.com");
    assert_eq!(links.len(), 1);
}

#[test]
fn test_extract_links_mixed() {
    let html = r#"
        <a href="/about">About</a>
        <a href="https://external.com">External</a>
        <a href="/products/item1">Product</a>
        <a href="mailto:a@b.com">Email</a>
        <a href="javascript:void(0)">JS</a>
    "#;

    let links = extract_links(html, "https://example.com");
    assert_eq!(links.len(), 3);
    assert!(links.contains(&"https://example.com/about".to_string()));
    assert!(links.contains(&"https://external.com".to_string()));
    assert!(links.contains(&"https://example.com/products/item1".to_string()));
}

#[test]
fn test_extract_links_empty_html() {
    let links = extract_links("", "https://example.com");
    assert!(links.is_empty());
}

#[test]
fn test_extract_links_no_links() {
    let html = "<html><body><p>No links here</p></body></html>";
    let links = extract_links(html, "https://example.com");
    assert!(links.is_empty());
}

// ============================================================================
// DOMAIN EXTRACTION TESTS
// ============================================================================

#[test]
fn test_extract_domain() {
    assert_eq!(
        extract_domain("https://example.com/page"),
        Some("https://example.com")
    );
}

#[test]
fn test_extract_domain_no_path() {
    assert_eq!(
        extract_domain("https://example.com"),
        Some("https://example.com")
    );
}

#[test]
fn test_extract_domain_with_port() {
    assert_eq!(
        extract_domain("https://example.com:8080/page"),
        Some("https://example.com:8080")
    );
}

#[test]
fn test_extract_domain_no_scheme() {
    assert_eq!(extract_domain("example.com/page"), None);
}

#[test]
fn test_is_same_domain_true() {
    assert!(is_same_domain(
        "https://example.com/page1",
        "https://example.com/page2"
    ));
}

#[test]
fn test_is_same_domain_false() {
    assert!(!is_same_domain(
        "https://example.com/page",
        "https://other.com/page"
    ));
}

// ============================================================================
// ROBOTS.TXT TESTS
// ============================================================================

#[test]
fn test_robots_disallowed() {
    let robots = "User-agent: *\nDisallow: /admin/\nDisallow: /private/\n";

    assert!(is_disallowed("/admin/users", robots));
    assert!(is_disallowed("/private/data", robots));
    assert!(!is_disallowed("/public/page", robots));
    assert!(!is_disallowed("/about", robots));
}

#[test]
fn test_robots_root_allowed() {
    let robots = "User-agent: *\nDisallow: /admin/\n";
    assert!(!is_disallowed("/", robots));
}

#[test]
fn test_robots_empty_disallow() {
    // Empty Disallow means everything is allowed
    let robots = "User-agent: *\nDisallow:\n";
    assert!(!is_disallowed("/anything", robots));
}

#[test]
fn test_robots_no_rules() {
    let robots = "";
    assert!(!is_disallowed("/anything", robots));
}

#[test]
fn test_parse_crawl_delay() {
    let robots = "User-agent: *\nCrawl-delay: 1\nDisallow: /admin/\n";
    assert_eq!(parse_crawl_delay(robots), Some(1.0));
}

#[test]
fn test_parse_crawl_delay_fractional() {
    let robots = "Crawl-delay: 0.5\n";
    assert_eq!(parse_crawl_delay(robots), Some(0.5));
}

#[test]
fn test_parse_crawl_delay_missing() {
    let robots = "User-agent: *\nDisallow: /admin/\n";
    assert_eq!(parse_crawl_delay(robots), None);
}

// ============================================================================
// BFS CRAWL SIMULATION TESTS
// ============================================================================

#[test]
fn test_bfs_crawl_single_page() {
    let visited = simulate_bfs_crawl(
        vec!["https://example.com".into()],
        3,
        100,
        |_url| vec![], // No outgoing links
    );

    assert_eq!(visited, vec!["https://example.com"]);
}

#[test]
fn test_bfs_crawl_linear_chain() {
    // A -> B -> C -> D
    let visited = simulate_bfs_crawl(
        vec!["A".into()],
        10,
        100,
        |url| match url {
            "A" => vec!["B".into()],
            "B" => vec!["C".into()],
            "C" => vec!["D".into()],
            _ => vec![],
        },
    );

    assert_eq!(visited, vec!["A", "B", "C", "D"]);
}

#[test]
fn test_bfs_crawl_handles_cycles() {
    // A -> B -> A (cycle)
    let visited = simulate_bfs_crawl(
        vec!["A".into()],
        10,
        100,
        |url| match url {
            "A" => vec!["B".into()],
            "B" => vec!["A".into()], // cycle back to A
            _ => vec![],
        },
    );

    assert_eq!(visited, vec!["A", "B"]);
}

#[test]
fn test_bfs_crawl_respects_max_depth() {
    // A(0) -> B(1) -> C(2) -> D(3) -> E(4)
    let visited = simulate_bfs_crawl(
        vec!["A".into()],
        2, // max depth 2
        100,
        |url| match url {
            "A" => vec!["B".into()],
            "B" => vec!["C".into()],
            "C" => vec!["D".into()],
            "D" => vec!["E".into()],
            _ => vec![],
        },
    );

    // Should visit A(depth 0), B(depth 1), C(depth 2), but not D(depth 3)
    assert_eq!(visited, vec!["A", "B", "C"]);
}

#[test]
fn test_bfs_crawl_respects_max_pages() {
    // Many pages, but limit to 3
    let visited = simulate_bfs_crawl(
        vec!["A".into()],
        10,
        3,
        |url| match url {
            "A" => vec!["B".into(), "C".into(), "D".into(), "E".into()],
            _ => vec![],
        },
    );

    assert_eq!(visited.len(), 3);
    assert_eq!(visited[0], "A");
}

#[test]
fn test_bfs_crawl_breadth_first_order() {
    // Tree:     A
    //          / \
    //         B   C
    //        / \
    //       D   E
    let visited = simulate_bfs_crawl(
        vec!["A".into()],
        10,
        100,
        |url| match url {
            "A" => vec!["B".into(), "C".into()],
            "B" => vec!["D".into(), "E".into()],
            _ => vec![],
        },
    );

    // BFS order: A, B, C, D, E
    assert_eq!(visited, vec!["A", "B", "C", "D", "E"]);
}

#[test]
fn test_bfs_crawl_deduplicates() {
    // A -> B, A -> C, B -> C (C should only be visited once)
    let visited = simulate_bfs_crawl(
        vec!["A".into()],
        10,
        100,
        |url| match url {
            "A" => vec!["B".into(), "C".into()],
            "B" => vec!["C".into()], // duplicate link to C
            _ => vec![],
        },
    );

    assert_eq!(visited, vec!["A", "B", "C"]);
}

#[test]
fn test_bfs_crawl_multiple_seeds() {
    let visited = simulate_bfs_crawl(
        vec!["A".into(), "B".into()],
        10,
        100,
        |url| match url {
            "A" => vec!["C".into()],
            "B" => vec!["D".into()],
            _ => vec![],
        },
    );

    assert_eq!(visited, vec!["A", "B", "C", "D"]);
}

// ============================================================================
// CRAWL STATUS TESTS
// ============================================================================

#[test]
fn test_crawl_result_success() {
    let result = CrawlResult {
        url: "https://example.com".into(),
        links_found: vec!["https://example.com/about".into()],
        status: CrawlStatus::Success,
    };

    assert_eq!(result.status, CrawlStatus::Success);
    assert_eq!(result.links_found.len(), 1);
}

#[test]
fn test_crawl_status_error() {
    let status = CrawlStatus::Error("404 Not Found".into());
    assert_eq!(status, CrawlStatus::Error("404 Not Found".into()));
}

#[test]
fn test_crawl_status_skipped() {
    let status = CrawlStatus::Skipped("robots.txt disallowed".into());
    assert_eq!(status, CrawlStatus::Skipped("robots.txt disallowed".into()));
}

// ============================================================================
// CRAWLER ERROR TESTS
// ============================================================================

#[test]
fn test_crawler_error_display() {
    assert_eq!(format!("{}", CrawlerError::NetworkError), "Network error");
    assert_eq!(format!("{}", CrawlerError::ParseError), "Parse error");
    assert_eq!(
        format!("{}", CrawlerError::RateLimitExceeded),
        "Rate limit exceeded"
    );
}
