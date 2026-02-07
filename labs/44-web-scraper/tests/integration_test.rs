// Integration tests for Lab 44: Web Scraper
//
// Tests HTML parsing and data extraction functions using static HTML strings.
// No network access required -- all tests use inline HTML content.

use web_scraper::*;

// ============================================================================
// TEST HTML FIXTURES
// ============================================================================

const SIMPLE_HTML: &str = r#"
<!DOCTYPE html>
<html>
<head><title>Test Page</title></head>
<body>
    <h1>Welcome</h1>
    <p>Hello, World!</p>
    <a href="https://example.com">Example</a>
    <a href="/about">About Us</a>
    <a href="/contact">Contact</a>
</body>
</html>
"#;

const ARTICLE_HTML: &str = r#"
<!DOCTYPE html>
<html>
<head><title>Blog</title></head>
<body>
    <article>
        <h2>First Post</h2>
        <a href="/post/1">Read more</a>
        <p>This is the first post description.</p>
    </article>
    <article>
        <h2>Second Post</h2>
        <a href="/post/2">Read more</a>
        <p>This is the second post description.</p>
    </article>
    <article>
        <h2>Third Post</h2>
        <a href="/post/3">Read more</a>
    </article>
</body>
</html>
"#;

const HEADINGS_HTML: &str = r#"
<!DOCTYPE html>
<html>
<head><title>Headings</title></head>
<body>
    <h1>Main Title</h1>
    <h2>Section One</h2>
    <h2>Section Two</h2>
    <h3>Subsection A</h3>
    <h3>Subsection B</h3>
    <h3>Subsection C</h3>
</body>
</html>
"#;

// ============================================================================
// TITLE EXTRACTION TESTS
// ============================================================================

#[test]
fn test_extract_title_basic() {
    let title = extract_title(SIMPLE_HTML);
    assert_eq!(title, Some("Test Page".to_string()));
}

#[test]
fn test_extract_title_from_article_page() {
    let title = extract_title(ARTICLE_HTML);
    assert_eq!(title, Some("Blog".to_string()));
}

#[test]
fn test_extract_title_missing() {
    let html = "<html><body><p>No title here</p></body></html>";
    let title = extract_title(html);
    assert_eq!(title, None);
}

#[test]
fn test_extract_title_empty() {
    let html = "<html><head><title></title></head><body></body></html>";
    let title = extract_title(html);
    assert_eq!(title, Some("".to_string()));
}

#[test]
fn test_extract_title_with_whitespace() {
    let html = "<html><head><title>  Spaces Around  </title></head></html>";
    let title = extract_title(html);
    assert_eq!(title, Some("Spaces Around".to_string()));
}

// ============================================================================
// LINK EXTRACTION TESTS
// ============================================================================

#[test]
fn test_extract_links_all() {
    let links = extract_links(SIMPLE_HTML, None);
    assert_eq!(links.len(), 3);
}

#[test]
fn test_extract_links_with_limit() {
    let links = extract_links(SIMPLE_HTML, Some(2));
    assert_eq!(links.len(), 2);
}

#[test]
fn test_extract_links_content() {
    let links = extract_links(SIMPLE_HTML, None);
    assert_eq!(links[0].href, "https://example.com");
    assert_eq!(links[0].text, "Example");
    assert_eq!(links[1].href, "/about");
    assert_eq!(links[1].text, "About Us");
    assert_eq!(links[2].href, "/contact");
    assert_eq!(links[2].text, "Contact");
}

#[test]
fn test_extract_links_no_links() {
    let html = "<html><body><p>No links here</p></body></html>";
    let links = extract_links(html, None);
    assert!(links.is_empty());
}

#[test]
fn test_extract_links_limit_zero() {
    let links = extract_links(SIMPLE_HTML, Some(0));
    assert!(links.is_empty());
}

#[test]
fn test_extract_links_limit_exceeds_count() {
    let links = extract_links(SIMPLE_HTML, Some(100));
    assert_eq!(links.len(), 3);
}

#[test]
fn test_extract_links_ignores_anchors_without_href() {
    let html = r#"<html><body><a name="section1">No href</a><a href="/real">Real</a></body></html>"#;
    let links = extract_links(html, None);
    assert_eq!(links.len(), 1);
    assert_eq!(links[0].href, "/real");
}

// ============================================================================
// HEADING EXTRACTION TESTS
// ============================================================================

#[test]
fn test_extract_headings_h1() {
    let headings = extract_headings(HEADINGS_HTML, 1);
    assert_eq!(headings.len(), 1);
    assert_eq!(headings[0].text, "Main Title");
    assert_eq!(headings[0].level, 1);
}

#[test]
fn test_extract_headings_h2() {
    let headings = extract_headings(HEADINGS_HTML, 2);
    assert_eq!(headings.len(), 2);
    assert_eq!(headings[0].text, "Section One");
    assert_eq!(headings[1].text, "Section Two");
}

#[test]
fn test_extract_headings_h3() {
    let headings = extract_headings(HEADINGS_HTML, 3);
    assert_eq!(headings.len(), 3);
    assert_eq!(headings[0].text, "Subsection A");
    assert_eq!(headings[1].text, "Subsection B");
    assert_eq!(headings[2].text, "Subsection C");
}

#[test]
fn test_extract_headings_none_found() {
    let headings = extract_headings(HEADINGS_HTML, 4);
    assert!(headings.is_empty());
}

#[test]
fn test_extract_headings_invalid_level_zero() {
    let headings = extract_headings(HEADINGS_HTML, 0);
    assert!(headings.is_empty());
}

#[test]
fn test_extract_headings_invalid_level_seven() {
    let headings = extract_headings(HEADINGS_HTML, 7);
    assert!(headings.is_empty());
}

#[test]
fn test_extract_all_headings() {
    let headings = extract_all_headings(HEADINGS_HTML);
    assert_eq!(headings.len(), 6); // 1 h1 + 2 h2 + 3 h3
}

// ============================================================================
// ARTICLE EXTRACTION TESTS
// ============================================================================

#[test]
fn test_extract_articles_count() {
    let articles = extract_articles(ARTICLE_HTML);
    assert_eq!(articles.len(), 3);
}

#[test]
fn test_extract_articles_first() {
    let articles = extract_articles(ARTICLE_HTML);
    assert_eq!(articles[0].title, "First Post");
    assert_eq!(articles[0].url, "/post/1");
    assert_eq!(
        articles[0].description,
        Some("This is the first post description.".to_string())
    );
}

#[test]
fn test_extract_articles_second() {
    let articles = extract_articles(ARTICLE_HTML);
    assert_eq!(articles[1].title, "Second Post");
    assert_eq!(articles[1].url, "/post/2");
    assert_eq!(
        articles[1].description,
        Some("This is the second post description.".to_string())
    );
}

#[test]
fn test_extract_articles_missing_description() {
    let articles = extract_articles(ARTICLE_HTML);
    assert_eq!(articles[2].title, "Third Post");
    assert_eq!(articles[2].url, "/post/3");
    assert_eq!(articles[2].description, None);
}

#[test]
fn test_extract_articles_none_found() {
    let html = "<html><body><p>No articles here</p></body></html>";
    let articles = extract_articles(html);
    assert!(articles.is_empty());
}

// ============================================================================
// GENERIC SELECTOR TESTS
// ============================================================================

#[test]
fn test_extract_text_by_selector_paragraphs() {
    let texts = extract_text_by_selector(SIMPLE_HTML, "p");
    assert_eq!(texts.len(), 1);
    assert_eq!(texts[0], "Hello, World!");
}

#[test]
fn test_extract_text_by_selector_h1() {
    let texts = extract_text_by_selector(SIMPLE_HTML, "h1");
    assert_eq!(texts.len(), 1);
    assert_eq!(texts[0], "Welcome");
}

#[test]
fn test_extract_text_by_selector_no_matches() {
    let texts = extract_text_by_selector(SIMPLE_HTML, "table");
    assert!(texts.is_empty());
}

#[test]
fn test_extract_attribute_hrefs() {
    let hrefs = extract_attribute(SIMPLE_HTML, "a[href]", "href");
    assert_eq!(hrefs.len(), 3);
    assert_eq!(hrefs[0], "https://example.com");
    assert_eq!(hrefs[1], "/about");
    assert_eq!(hrefs[2], "/contact");
}

#[test]
fn test_extract_attribute_missing_attr() {
    let results = extract_attribute(SIMPLE_HTML, "p", "class");
    assert!(results.is_empty());
}

#[test]
fn test_extract_attribute_with_classes() {
    let html = r#"<html><body>
        <div class="highlight">A</div>
        <div class="normal">B</div>
        <div>C</div>
    </body></html>"#;
    let classes = extract_attribute(html, "div", "class");
    assert_eq!(classes.len(), 2);
    assert_eq!(classes[0], "highlight");
    assert_eq!(classes[1], "normal");
}

// ============================================================================
// EDGE CASES
// ============================================================================

#[test]
fn test_empty_html() {
    assert_eq!(extract_title(""), None);
    assert!(extract_links("", None).is_empty());
    assert!(extract_headings("", 1).is_empty());
    assert!(extract_articles("").is_empty());
}

#[test]
fn test_malformed_html() {
    let html = "<html><body><p>Unclosed paragraph<div>Nested wrong</p></div></body>";
    // Should not panic -- html5ever handles malformed HTML gracefully
    let texts = extract_text_by_selector(html, "p");
    assert!(!texts.is_empty());
}

#[test]
fn test_nested_links() {
    let html = r#"<html><body>
        <div><a href="/outer">Outer <span>inner text</span></a></div>
    </body></html>"#;
    let links = extract_links(html, None);
    assert_eq!(links.len(), 1);
    assert_eq!(links[0].href, "/outer");
    // Text includes inner span text
    assert!(links[0].text.contains("Outer"));
    assert!(links[0].text.contains("inner text"));
}

#[test]
fn test_special_characters_in_text() {
    let html = r#"<html><head><title>Caf&eacute; &amp; Bar</title></head></html>"#;
    let title = extract_title(html);
    assert!(title.is_some());
    let t = title.unwrap();
    assert!(t.contains("Caf"));
    assert!(t.contains("Bar"));
}
