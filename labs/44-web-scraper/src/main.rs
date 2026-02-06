// Project 41: Web Scraper
//
// Demonstrates HTTP requests, HTML parsing, async networking, and data extraction.
// This is the foundation for building web crawlers, data collection systems, and automation tools.

use reqwest;
use scraper::{Html, Selector};
use std::error::Error;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("=== Web Scraper ===\n");

    // Example 1: Simple GET request
    println!("1. Fetching example.com...");
    simple_fetch("https://example.com").await?;

    println!("\n" + &"=".repeat(60) + "\n");

    // Example 2: Extract specific data from HTML
    println!("2. Extracting data from Rust homepage...");
    extract_rust_homepage_data().await?;

    println!("\n" + &"=".repeat(60) + "\n");

    // Example 3: Scrape multiple pages with rate limiting
    println!("3. Scraping multiple pages with rate limiting...");
    scrape_multiple_urls().await?;

    println!("\n" + &"=".repeat(60) + "\n");

    // Example 4: Error handling for failed requests
    println!("4. Demonstrating error handling...");
    demonstrate_error_handling().await;

    println!("\n=== Scraping Complete ===");

    Ok(())
}

// ============================================================================
// EXAMPLE 1: SIMPLE HTTP GET REQUEST
// ============================================================================

async fn simple_fetch(url: &str) -> Result<(), Box<dyn Error>> {
    // Create an HTTP client
    // reqwest::Client is reusable and maintains connection pools
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Educational Rust Scraper)")
        .timeout(Duration::from_secs(10))
        .build()?;

    // Make the request
    let response = client.get(url).send().await?;

    println!("  Status: {}", response.status());
    println!("  URL: {}", response.url());

    // Get headers
    println!("  Content-Type: {:?}", response.headers().get("content-type"));

    // Get the response body as text
    let body = response.text().await?;
    println!("  Body length: {} bytes", body.len());
    println!("  First 200 characters:");
    println!("  {}", &body.chars().take(200).collect::<String>());

    Ok(())
}

// ============================================================================
// EXAMPLE 2: HTML PARSING AND DATA EXTRACTION
// ============================================================================

async fn extract_rust_homepage_data() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Educational Rust Scraper)")
        .build()?;

    // Fetch the Rust homepage
    let url = "https://www.rust-lang.org/";
    println!("  Fetching: {}", url);

    let response = client.get(url).send().await?;
    let html_content = response.text().await?;

    // Parse HTML
    let document = Html::parse_document(&html_content);

    // Extract the page title
    if let Ok(title_selector) = Selector::parse("title") {
        if let Some(title) = document.select(&title_selector).next() {
            let title_text = title.text().collect::<String>();
            println!("  Page Title: {}", title_text.trim());
        }
    }

    // Extract all links
    if let Ok(link_selector) = Selector::parse("a[href]") {
        let links: Vec<_> = document
            .select(&link_selector)
            .take(10) // Just first 10 links
            .filter_map(|element| {
                let href = element.value().attr("href")?;
                let text = element.text().collect::<String>();
                Some((href.to_string(), text.trim().to_string()))
            })
            .collect();

        println!("\n  First 10 links:");
        for (i, (href, text)) in links.iter().enumerate() {
            println!("    {}. {} -> {}", i + 1, text, href);
        }
    }

    // Extract headings
    if let Ok(h1_selector) = Selector::parse("h1") {
        println!("\n  H1 Headings:");
        for heading in document.select(&h1_selector).take(5) {
            let text = heading.text().collect::<String>();
            println!("    - {}", text.trim());
        }
    }

    Ok(())
}

// ============================================================================
// EXAMPLE 3: SCRAPING MULTIPLE URLS WITH RATE LIMITING
// ============================================================================

async fn scrape_multiple_urls() -> Result<(), Box<dyn Error>> {
    // List of URLs to scrape
    let urls = vec![
        "https://example.com",
        "https://example.org",
        "https://example.net",
    ];

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Educational Rust Scraper)")
        .build()?;

    println!("  Scraping {} URLs with 1-second delays...\n", urls.len());

    for (i, url) in urls.iter().enumerate() {
        println!("  [{}/{}] Fetching: {}", i + 1, urls.len(), url);

        match fetch_and_extract_title(&client, url).await {
            Ok(title) => println!("    Title: {}", title),
            Err(e) => println!("    Error: {}", e),
        }

        // Rate limiting: wait 1 second between requests
        // This is CRUCIAL for ethical scraping!
        if i < urls.len() - 1 {
            sleep(Duration::from_secs(1)).await;
        }
    }

    Ok(())
}

async fn fetch_and_extract_title(
    client: &reqwest::Client,
    url: &str,
) -> Result<String, Box<dyn Error>> {
    let response = client.get(url).send().await?;
    let html = response.text().await?;
    let document = Html::parse_document(&html);

    // Try to extract title
    if let Ok(selector) = Selector::parse("title") {
        if let Some(element) = document.select(&selector).next() {
            let title = element.text().collect::<String>();
            return Ok(title.trim().to_string());
        }
    }

    Ok("(No title found)".to_string())
}

// ============================================================================
// EXAMPLE 4: ERROR HANDLING
// ============================================================================

async fn demonstrate_error_handling() {
    let test_cases = vec![
        ("https://httpbin.org/status/200", "Should succeed"),
        ("https://httpbin.org/status/404", "Should get 404"),
        ("https://httpbin.org/delay/2", "Should succeed with delay"),
        ("https://this-domain-definitely-does-not-exist-12345.com", "Should fail (DNS)"),
    ];

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Educational Rust Scraper)")
        .timeout(Duration::from_secs(5))
        .build()
        .expect("Failed to build client");

    for (url, description) in test_cases {
        println!("\n  Testing: {} ({})", url, description);

        match client.get(url).send().await {
            Ok(response) => {
                let status = response.status();
                println!("    ✓ Response received: {}", status);

                if status.is_success() {
                    println!("    Status: Success (2xx)");
                } else if status.is_client_error() {
                    println!("    Status: Client Error (4xx)");
                } else if status.is_server_error() {
                    println!("    Status: Server Error (5xx)");
                }
            }
            Err(e) => {
                println!("    ✗ Request failed: {}", e);

                // Detailed error information
                if e.is_timeout() {
                    println!("    Error type: Timeout");
                } else if e.is_connect() {
                    println!("    Error type: Connection failed");
                } else if e.is_request() {
                    println!("    Error type: Request error");
                }
            }
        }
    }
}

// ============================================================================
// ADVANCED: EXTRACTING STRUCTURED DATA
// ============================================================================

#[allow(dead_code)]
struct Article {
    title: String,
    url: String,
    description: Option<String>,
}

#[allow(dead_code)]
async fn extract_articles(url: &str) -> Result<Vec<Article>, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;
    let html = response.text().await?;
    let document = Html::parse_document(&html);

    let mut articles = Vec::new();

    // Example: Extract articles from a blog
    // This selector would need to be adjusted for the actual website structure
    if let Ok(article_selector) = Selector::parse("article") {
        for article_el in document.select(&article_selector) {
            // Extract title
            let title = if let Ok(title_sel) = Selector::parse("h2") {
                article_el
                    .select(&title_sel)
                    .next()
                    .map(|el| el.text().collect::<String>())
                    .unwrap_or_default()
            } else {
                String::new()
            };

            // Extract URL
            let url = if let Ok(link_sel) = Selector::parse("a[href]") {
                article_el
                    .select(&link_sel)
                    .next()
                    .and_then(|el| el.value().attr("href"))
                    .unwrap_or("")
                    .to_string()
            } else {
                String::new()
            };

            // Extract description
            let description = if let Ok(desc_sel) = Selector::parse("p") {
                article_el
                    .select(&desc_sel)
                    .next()
                    .map(|el| el.text().collect::<String>())
            } else {
                None
            };

            articles.push(Article {
                title: title.trim().to_string(),
                url,
                description: description.map(|d| d.trim().to_string()),
            });
        }
    }

    Ok(articles)
}

// ============================================================================
// WHAT RUST DOES UNDER THE HOOD
// ============================================================================
// 1. ASYNC RUNTIME (tokio)
//    - Tokio provides an event loop that manages async tasks
//    - Each .await point yields control back to the runtime
//    - The runtime can handle thousands of concurrent connections
//    - Unlike OS threads, async tasks are lightweight (few KB each)
//
// 2. HTTP CONNECTION POOLING
//    - reqwest::Client maintains a pool of TCP connections
//    - Reusing connections is MUCH faster than creating new ones
//    - Connection pooling is automatic and transparent
//
// 3. HTML PARSING
//    - scraper uses html5ever, a fast HTML5 parser written in Rust
//    - It builds a DOM tree in memory (like a browser does)
//    - CSS selectors are compiled to optimized matchers
//    - For a 1MB HTML file, parsing takes ~5-10ms
//
// 4. MEMORY MANAGEMENT
//    - No garbage collection! All memory is freed when variables go out of scope
//    - The Html document owns all parsed nodes
//    - When document is dropped, entire DOM tree is freed
//
// 5. ERROR HANDLING
//    - Result<T, E> forces you to handle errors
//    - The ? operator propagates errors up the call stack
//    - Box<dyn Error> allows any error type (trait object)

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. reqwest is the standard HTTP client for Rust (async by default)
// 2. scraper provides CSS selector-based HTML parsing
// 3. Always use User-Agent headers to identify your scraper
// 4. Rate limiting is essential for ethical scraping
// 5. Handle errors gracefully (network can fail!)
// 6. Respect robots.txt and Terms of Service
// 7. Async/await enables efficient concurrent scraping
// 8. CSS selectors are powerful: "div.class > p", "a[href^='http']", etc.

// ============================================================================
// CSS SELECTOR EXAMPLES
// ============================================================================
// "h1"              - All <h1> elements
// ".className"      - All elements with class="className"
// "#id"             - Element with id="id"
// "div > p"         - Direct <p> children of <div>
// "div p"           - All <p> descendants of <div>
// "a[href]"         - All <a> elements with href attribute
// "a[href^='http']" - Links starting with "http"
// "div.class1.class2" - <div> with both classes
// "p:first-child"   - First <p> child of its parent
// "li:nth-child(2)" - Second <li> child

// ============================================================================
// COMMON BEGINNER MISTAKES
// ============================================================================
// ❌ Not handling missing elements (unwrap on Option)
// ❌ Scraping too fast (no rate limiting)
// ❌ Ignoring HTTP status codes (not all responses are 200 OK)
// ❌ Not setting User-Agent header
// ❌ Using blocking reqwest in async context
// ❌ Not handling network errors (timeout, DNS failure)
// ❌ Parsing malformed HTML with strict parser
// ❌ Memory leaks when scraping millions of pages (store data, don't accumulate)

// ============================================================================
// PERFORMANCE TIPS
// ============================================================================
// 1. Reuse reqwest::Client (connection pooling)
// 2. Use async for I/O-bound operations
// 3. Limit concurrent requests (tokio::sync::Semaphore)
// 4. Stream large responses instead of loading into memory
// 5. Use compressed responses (gzip, deflate) - reqwest does this automatically
// 6. Cache responses when possible
// 7. Use HTTP/2 for multiplexing (reqwest supports this)

// ============================================================================
// REAL-WORLD SCRAPING CHALLENGES
// ============================================================================
// - JavaScript-rendered content: Use headless browser (fantoccini, headless_chrome)
// - CAPTCHAs: Use services like 2captcha, or avoid them (respect restrictions)
// - IP blocking: Rotate proxies, use residential IPs
// - Anti-bot detection: Mimic browser behavior (headers, delays, cookies)
// - Dynamic content: Handle AJAX requests, WebSockets
// - Authentication: Handle login, sessions, cookies
// - Pagination: Follow "next" links, handle infinite scroll
// - Data cleaning: Remove HTML entities, normalize whitespace

// ============================================================================
// ETHICAL AND LEGAL CONSIDERATIONS
// ============================================================================
// ✓ Check robots.txt before scraping
// ✓ Respect rate limits and server resources
// ✓ Don't scrape personal/private data
// ✓ Follow Terms of Service
// ✓ Cache responses to avoid repeated requests
// ✓ Identify yourself with User-Agent
// ✗ Don't DOS the server with too many requests
// ✗ Don't ignore copyright and data ownership
// ✗ Don't scrape authentication-protected content without permission
