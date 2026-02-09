//! # Web Scraper - Demo

use std::error::Error;
use tokio::time::{sleep, Duration};
use web_scraper::solution::{extract_articles, extract_headings, extract_links, extract_title};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("=== Web Scraper Demo ===\n");

    let html = r#"<html><body><title>Demo</title><h1>Welcome</h1><a href=\"/1\">Link 1</a></body></html>"#;

    println!("Title: {:?}", extract_title(html));
    println!("Links: {:?}", extract_links(html, None));
    println!("Headings: {:?}", extract_headings(html, 1));
    println!("Articles: {:?}", extract_articles(html));

    sleep(Duration::from_millis(10)).await;
    Ok(())
}
