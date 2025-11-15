# Project 41: Web Scraper

## Overview
Build a web scraper that fetches HTML pages and extracts structured data using HTTP requests and HTML parsing. This project demonstrates async networking, HTTP client usage, HTML parsing, error handling, and working with external APIs and web content.

## Concepts Taught
- **HTTP requests** with `reqwest` crate
- **Async/await** for non-blocking I/O
- **HTML parsing** with `scraper` crate
- **CSS selectors** for element extraction
- **Error handling** with `Result` and `?` operator
- **User-Agent headers** and HTTP headers
- **Rate limiting** and ethical scraping
- **Data extraction** and cleaning

## Why Rust for Web Scraping

### Performance
Rust's async runtime (tokio) is extremely efficient, allowing thousands of concurrent requests with minimal memory overhead. Unlike Python's GIL (Global Interpreter Lock), Rust can fully utilize multiple cores.

### Safety
Rust's type system prevents common scraping bugs:
- No null pointer exceptions when parsing HTML
- Memory safety when processing large documents
- Thread safety when scraping concurrently

**Comparison with other languages:**
- **Python**: Beautiful Soup is easy but slow; asyncio has GIL limitations
- **Go**: Fast concurrency but more verbose error handling
- **TypeScript**: Good with Cheerio, but slower than Rust

## Beginner Pitfalls & Best Practices

### Pitfall 1: Blocking in Async Context
```rust
// ❌ WRONG: Using blocking reqwest in async context
async fn fetch() {
    let res = reqwest::blocking::get("https://example.com"); // Blocks the executor!
}
```
**Fix**: Use the async client:
```rust
// ✅ CORRECT: Use async reqwest
async fn fetch() {
    let res = reqwest::get("https://example.com").await?;
}
```

### Pitfall 2: Not Handling Request Failures
```rust
// ❌ WRONG: Unwrapping can panic
let body = reqwest::get(url).await.unwrap();
```
**Fix**: Use `?` operator or handle errors:
```rust
// ✅ CORRECT: Propagate errors
let body = reqwest::get(url).await?;
```

### Pitfall 3: Scraping Too Fast (Rate Limiting)
Web scraping requires ethical practices:
- Respect `robots.txt`
- Add delays between requests (rate limiting)
- Set a proper User-Agent header
- Don't overload servers

### Pitfall 4: CSS Selector Mistakes
```rust
// ❌ WRONG: Selector might not exist
let title = document.select(&Selector::parse("h1").unwrap()).next().unwrap();
```
**Fix**: Handle missing elements gracefully:
```rust
// ✅ CORRECT: Check if element exists
if let Some(title) = document.select(&Selector::parse("h1").unwrap()).next() {
    println!("Title: {}", title.text().collect::<String>());
}
```

## Code Walkthrough

See `src/main.rs` for a complete implementation that demonstrates:
1. Making async HTTP requests with reqwest
2. Parsing HTML with scraper
3. Extracting data using CSS selectors
4. Error handling for network and parsing errors
5. Setting custom headers (User-Agent)
6. Processing multiple pages
7. Cleaning and formatting extracted data

## Ethical Web Scraping

### Respect robots.txt
Always check the website's `robots.txt` file:
```
https://example.com/robots.txt
```

### Rate Limiting
Add delays to avoid overwhelming servers:
```rust
use tokio::time::{sleep, Duration};
sleep(Duration::from_millis(500)).await; // 500ms delay
```

### User-Agent Header
Identify your scraper:
```rust
let client = reqwest::Client::builder()
    .user_agent("MyBot/1.0 (contact@example.com)")
    .build()?;
```

### Legal Considerations
- Check the website's Terms of Service
- Don't scrape private/authenticated content without permission
- Respect copyright and data ownership

## Performance Considerations

**Memory usage**: The `scraper` crate builds a DOM tree in memory. For large HTML documents (>10MB), this can use significant memory. Consider streaming parsers for huge documents.

**Concurrent scraping**: Tokio allows thousands of concurrent requests. However:
- Limit concurrency to avoid overwhelming target servers
- Use connection pooling (reqwest does this automatically)
- Set timeouts to prevent hanging requests

**Benchmarks** (approximate):
- Fetching 100 pages sequentially: ~30 seconds
- Fetching 100 pages concurrently (10 at a time): ~3 seconds
- Parsing a 1MB HTML page: ~5-10ms

## Comparison: Rust vs Python vs Go

| Feature | Rust (reqwest + scraper) | Python (BeautifulSoup) | Go (Colly) |
|---------|-------------------------|------------------------|------------|
| Speed | Very fast (async) | Slow (GIL limits) | Fast (goroutines) |
| Parsing | Fast (html5ever) | Slow (lxml) | Fast (goquery) |
| Memory | Low overhead | High (GC) | Medium (GC) |
| Error handling | Compile-time safety | Runtime errors | Runtime errors |
| Concurrency | Excellent (tokio) | Limited (GIL) | Excellent (goroutines) |

## Additional Challenges

1. **Concurrent scraper**: Modify the code to scrape multiple URLs concurrently using `tokio::spawn` and `join_all`.

2. **Data export**: Save scraped data to JSON or CSV format.

3. **Pagination handling**: Scrape multi-page websites by following "next page" links.

4. **JavaScript-rendered sites**: Use headless browser automation (e.g., `fantoccini` with Selenium/WebDriver).

5. **Retry logic**: Implement exponential backoff for failed requests.

6. **robots.txt parser**: Write a parser that respects robots.txt rules.

## Future Directions

- **Project 48**: Concurrent web crawler with distributed scraping
- **Advanced**: Build a full crawler with URL frontier, duplicate detection
- **Production**: Add proxy rotation, CAPTCHA solving, anti-bot detection

## Running This Project

```bash
cd 41-web-scraper
cargo run
```

**Note**: Add to `Cargo.toml`:
```toml
[dependencies]
reqwest = { version = "0.11", features = ["json"] }
scraper = "0.17"
tokio = { version = "1", features = ["full"] }
```

## Expected Output

The program will:
1. Fetch example.com (or another URL)
2. Parse the HTML content
3. Extract and display titles, links, and other data
4. Demonstrate error handling for failed requests
5. Show how to set custom headers and handle responses

## Common HTTP Status Codes

- **200 OK**: Request successful
- **404 Not Found**: Page doesn't exist
- **429 Too Many Requests**: You're being rate-limited!
- **403 Forbidden**: Access denied (check User-Agent, cookies)
- **500 Internal Server Error**: Server problem
- **503 Service Unavailable**: Server overloaded or down

## Dependencies Explained

- **reqwest**: Modern, async HTTP client (like Python's `requests`)
- **scraper**: HTML parsing with CSS selectors (like Beautiful Soup)
- **tokio**: Async runtime for handling concurrent I/O
- **html5ever**: Fast HTML5 parser (used internally by scraper)
