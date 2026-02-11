// HTML parser: extract headings, paragraphs, and links into a Page.

use anyhow::{Context, Result};
use scraper::{Html, Selector};
use url::Url;

/// Parsed page content: text blocks (headings + paragraphs) and links (anchor text, url).
pub struct Page {
    pub text: Vec<String>,
    pub links: Vec<(String, String)>,
}

/// Clean whitespace: trim and collapse runs of whitespace to a single space.
fn clean_whitespace(s: &str) -> String {
    let s = s.trim();
    if s.is_empty() {
        return String::new();
    }
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Parse HTML and extract text (h1, h2, h3, p) and links (a with valid href).
/// Relative URLs are resolved against base_url.
pub fn parse(html: &str, base_url: &str) -> Result<Page> {
    let document = Html::parse_document(html);
    let base = Url::parse(base_url).context("invalid base URL")?;

    let text_selector =
        Selector::parse("h1, h2, h3, p").context("invalid text selector")?;
    let link_selector =
        Selector::parse("a[href]").context("invalid link selector")?;

    let mut text = Vec::new();
    for element in document.select(&text_selector) {
        let raw: String = element.text().collect();
        let cleaned = clean_whitespace(&raw);
        if !cleaned.is_empty() {
            text.push(cleaned);
        }
    }

    let mut links = Vec::new();
    for element in document.select(&link_selector) {
        let href = match element.value().attr("href") {
            Some(h) if !h.trim().is_empty() => h.trim(),
            _ => continue,
        };
        // Skip javascript:, mailto:, etc.
        if href.starts_with('#') || href.starts_with("javascript:") || href.starts_with("mailto:") {
            continue;
        }
        let absolute = match base.join(href) {
            Ok(u) => u.to_string(),
            Err(_) => continue,
        };
        let anchor: String = element.text().collect();
        let anchor = clean_whitespace(&anchor);
        links.push((anchor, absolute));
    }

    Ok(Page { text, links })
}
