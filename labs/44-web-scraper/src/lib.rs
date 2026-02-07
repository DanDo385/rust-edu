// Lab 44: Web Scraper
//
// This module provides HTML parsing utilities and data structures for web scraping.
// The pure parsing/data-model logic is extracted here so it can be tested without
// network access. The async HTTP fetching remains in main.rs.
//
// Key concepts:
// - HTML parsing with the `scraper` crate
// - CSS selector-based data extraction
// - Structured data models for scraped content

use scraper::{Html, Selector};

// ============================================================================
// DATA STRUCTURES
// ============================================================================

/// Represents a hyperlink extracted from an HTML document.
#[derive(Debug, Clone, PartialEq)]
pub struct Link {
    /// The href attribute value
    pub href: String,
    /// The visible text of the link
    pub text: String,
}

/// Represents an article extracted from an HTML page.
#[derive(Debug, Clone, PartialEq)]
pub struct Article {
    pub title: String,
    pub url: String,
    pub description: Option<String>,
}

/// Represents a heading extracted from an HTML page.
#[derive(Debug, Clone, PartialEq)]
pub struct Heading {
    /// The heading level (1-6)
    pub level: u8,
    /// The text content of the heading
    pub text: String,
}

// ============================================================================
// HTML PARSING FUNCTIONS
// ============================================================================

/// Extract the page title from an HTML document.
///
/// Looks for the `<title>` element and returns its trimmed text content.
/// Returns `None` if no title element is found.
pub fn extract_title(html: &str) -> Option<String> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("title").ok()?;

    document
        .select(&selector)
        .next()
        .map(|el| el.text().collect::<String>().trim().to_string())
}

/// Extract all hyperlinks (`<a href="...">`) from an HTML document.
///
/// Returns a vector of `Link` structs containing each link's href and visible text.
/// An optional `limit` parameter restricts how many links to return.
pub fn extract_links(html: &str, limit: Option<usize>) -> Vec<Link> {
    let document = Html::parse_document(html);
    let selector = match Selector::parse("a[href]") {
        Ok(s) => s,
        Err(_) => return Vec::new(),
    };

    let iter = document.select(&selector).filter_map(|element| {
        let href = element.value().attr("href")?;
        let text = element.text().collect::<String>().trim().to_string();
        Some(Link {
            href: href.to_string(),
            text,
        })
    });

    match limit {
        Some(n) => iter.take(n).collect(),
        None => iter.collect(),
    }
}

/// Extract all headings of a given level (h1-h6) from an HTML document.
///
/// The `level` parameter must be between 1 and 6 (inclusive).
/// Returns an empty vector for invalid levels.
pub fn extract_headings(html: &str, level: u8) -> Vec<Heading> {
    if level == 0 || level > 6 {
        return Vec::new();
    }

    let tag = format!("h{}", level);
    let document = Html::parse_document(html);
    let selector = match Selector::parse(&tag) {
        Ok(s) => s,
        Err(_) => return Vec::new(),
    };

    document
        .select(&selector)
        .map(|el| Heading {
            level,
            text: el.text().collect::<String>().trim().to_string(),
        })
        .collect()
}

/// Extract all headings of all levels (h1 through h6) from an HTML document.
pub fn extract_all_headings(html: &str) -> Vec<Heading> {
    let mut headings = Vec::new();
    for level in 1..=6 {
        headings.extend(extract_headings(html, level));
    }
    headings
}

/// Extract articles from an HTML document using `<article>` elements.
///
/// For each `<article>`, extracts:
/// - Title from the first `<h2>` child
/// - URL from the first `<a href>` child
/// - Description from the first `<p>` child
pub fn extract_articles(html: &str) -> Vec<Article> {
    let document = Html::parse_document(html);
    let article_selector = match Selector::parse("article") {
        Ok(s) => s,
        Err(_) => return Vec::new(),
    };

    let mut articles = Vec::new();

    for article_el in document.select(&article_selector) {
        let title = Selector::parse("h2")
            .ok()
            .and_then(|sel| {
                article_el
                    .select(&sel)
                    .next()
                    .map(|el| el.text().collect::<String>().trim().to_string())
            })
            .unwrap_or_default();

        let url = Selector::parse("a[href]")
            .ok()
            .and_then(|sel| {
                article_el
                    .select(&sel)
                    .next()
                    .and_then(|el| el.value().attr("href"))
                    .map(|s| s.to_string())
            })
            .unwrap_or_default();

        let description = Selector::parse("p").ok().and_then(|sel| {
            article_el
                .select(&sel)
                .next()
                .map(|el| el.text().collect::<String>().trim().to_string())
        });

        articles.push(Article {
            title,
            url,
            description,
        });
    }

    articles
}

/// Extract text content from elements matching a CSS selector.
///
/// Returns the trimmed text content of each matching element.
pub fn extract_text_by_selector(html: &str, css_selector: &str) -> Vec<String> {
    let document = Html::parse_document(html);
    let selector = match Selector::parse(css_selector) {
        Ok(s) => s,
        Err(_) => return Vec::new(),
    };

    document
        .select(&selector)
        .map(|el| el.text().collect::<String>().trim().to_string())
        .collect()
}

/// Extract attribute values from elements matching a CSS selector.
///
/// Returns the value of the specified attribute for each matching element.
pub fn extract_attribute(html: &str, css_selector: &str, attr: &str) -> Vec<String> {
    let document = Html::parse_document(html);
    let selector = match Selector::parse(css_selector) {
        Ok(s) => s,
        Err(_) => return Vec::new(),
    };

    document
        .select(&selector)
        .filter_map(|el| el.value().attr(attr).map(|s| s.to_string()))
        .collect()
}

// ============================================================================
// WHAT RUST DOES UNDER THE HOOD
// ============================================================================
// 1. HTML PARSING
//    - scraper uses html5ever, a fast HTML5 parser written in Rust
//    - It builds a DOM tree in memory (like a browser does)
//    - CSS selectors are compiled to optimized matchers
//
// 2. MEMORY MANAGEMENT
//    - No garbage collection! All memory is freed when variables go out of scope
//    - The Html document owns all parsed nodes
//    - When document is dropped, entire DOM tree is freed
//
// 3. OPTION / FILTER_MAP
//    - filter_map combines filter and map in one pass
//    - Options propagate gracefully without panicking
//    - The ? operator in closures returns None to skip elements
