mod browser;

fn main() {
    // Step 2: basic test - fetch and print first 200 chars
    match browser::fetch("https://example.com") {
        Ok(html) => {
            let preview: String = html.chars().take(200).collect();
            println!("Fetched {} bytes. First 200 chars:\n{}", html.len(), preview);
        }
        Err(e) => {
            eprintln!("Fetch error: {}", e);
        }
    }
}
