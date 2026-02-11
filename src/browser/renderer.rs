// Terminal renderer: print page text and numbered links.

use crate::parser::Page;

/// Render a parsed page to the terminal: headings emphasized, paragraphs, then numbered links.
pub fn render(page: &Page) {
    // Print text content: first line as main heading (uppercase), rest as body
    if let Some(first) = page.text.first() {
        println!("{}\n", first.to_uppercase());
    }
    for line in page.text.iter().skip(1) {
        println!("{}\n", line);
    }

    // Numbered links
    if !page.links.is_empty() {
        println!("--- Links ---");
        for (i, (anchor, _href)) in page.links.iter().enumerate() {
            let label = if anchor.is_empty() { "[link]" } else { anchor.as_str() };
            println!("[{}] {}", i + 1, label);
        }
    }
}
