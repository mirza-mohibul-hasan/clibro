// Blocking HTTP fetcher. Returns HTML body as String.

use anyhow::{Context, Result};

/// Fetch a URL and return the response body as a String.
/// Uses blocking reqwest. Fails on HTTP errors (non-2xx).
pub fn fetch(url: &str) -> Result<String> {
    let client = reqwest::blocking::Client::builder()
        .build()
        .context("failed to create HTTP client")?;

    let response = client
        .get(url)
        .send()
        .context("failed to send HTTP request")?;

    let status = response.status();
    if !status.is_success() {
        anyhow::bail!("HTTP error: {}", status);
    }

    let body = response.text().context("failed to read response body")?;
    Ok(body)
}
