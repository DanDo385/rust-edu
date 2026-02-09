//! # Web Scraper - Student API

/// Represents a link with href and text.
pub struct Link {
    href: String,
    text: String,
}

impl Link {
    pub fn href(&self) -> &str {
        todo!("Return href")
    }

    pub fn text(&self) -> &str {
        todo!("Return text")
    }
}

pub struct Article;
pub struct Heading;

pub fn extract_title(_html: &str) -> Option<String> {
    todo!("Extract document title")
}

pub fn extract_links(_html: &str, _limit: Option<usize>) -> Vec<Link> {
    todo!("Extract anchor tags")
}

pub fn extract_headings(_html: &str, _level: u8) -> Vec<Heading> {
    todo!("Find headings of level")
}

pub fn extract_all_headings(_html: &str) -> Vec<Heading> {
    todo!("Extract all headings")
}

pub fn extract_articles(_html: &str) -> Vec<Article> {
    todo!("Collect articles")
}

pub fn extract_text_by_selector(_html: &str, _selector: &str) -> Vec<String> {
    todo!("Extract text via selector")
}

pub fn extract_attribute(_html: &str, _selector: &str, _attr: &str) -> Vec<String> {
    todo!("Extract attribute values")
}

#[doc(hidden)]
pub mod solution;
