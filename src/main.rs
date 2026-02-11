// Phase 1: minimal CLI browser with fetch, parse, render, and navigation loop.

mod browser;

use std::io::{self, Write};

fn main() {
    let mut current_url = String::new();

    loop {
        // Get URL: first time ask, then use current_url after following a link
        if current_url.is_empty() {
            print!("Enter URL (or q to quit): ");
            let _ = io::stdout().flush();
            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                eprintln!("Failed to read input.");
                continue;
            }
            let input = input.trim();
            if input.eq_ignore_ascii_case("q") || input.is_empty() {
                break;
            }
            current_url = input.to_string();
        }

        // Fetch
        let html = match browser::fetch(&current_url) {
            Ok(body) => body,
            Err(e) => {
                eprintln!("Fetch error: {}", e);
                current_url.clear();
                continue;
            }
        };

        // Parse
        let page = match browser::parse(&html, &current_url) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Parse error: {}", e);
                current_url.clear();
                continue;
            }
        };

        // Render
        browser::render(&page);

        // Reset so next iteration we ask for link or quit
        current_url.clear();

        // Link navigation loop
        loop {
            print!("Enter link number (or q to quit): ");
            let _ = io::stdout().flush();
            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                eprintln!("Failed to read input.");
                continue;
            }
            let input = input.trim();

            if input.eq_ignore_ascii_case("q") || input.is_empty() {
                return;
            }

            let n: usize = match input.parse() {
                Ok(k) => k,
                Err(_) => {
                    println!("Invalid input. Enter a number 1-{} or q to quit.", page.links.len());
                    continue;
                }
            };

            if n >= 1 && n <= page.links.len() {
                current_url = page.links[n - 1].1.clone();
                break;
            }

            println!(
                "Invalid number. Enter 1 to {} or q to quit.",
                page.links.len()
            );
        }
    }
}
